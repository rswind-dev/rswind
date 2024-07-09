use rswind_css::{rule::RuleList, Rule};
use smallvec::SmallVec;
use smol_str::{format_smolstr, SmolStr};
use thiserror::Error;

use crate::{common::StrReplaceExt, parse::VariantCandidate};

pub trait VariantMatchingFn: Fn(RuleList) -> Option<RuleList> + Sync + Send {}

impl<T: Fn(RuleList) -> Option<RuleList> + Sync + Send> VariantMatchingFn for T {}

pub trait VariantHandlerExt {
    fn handle(&self, candidate: &VariantCandidate<'_>, rule: RuleList) -> RuleList;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VariantKind {
    Static,
    Dynamic,
    Composable,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VariantHandler {
    Static(StaticHandler),
    Dynamic(DynamicHandler),
    Composable(ComposableHandler),
}

impl VariantHandler {
    pub fn take_composable(self) -> Option<ComposableHandler> {
        match self {
            Self::Composable(handler) => Some(handler),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub handler: VariantHandler,
    pub composable: bool,
    pub kind: VariantKind,
    pub ordering: VariantOrdering,
    pub nested: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum VariantOrdering {
    Unset,
    /// Insert order
    Insertion(u64),
    /// Length in pixels
    Length(u64),
    /// Arbitrary variant, place it at the end
    Arbitrary,
}

#[derive(Debug, Error)]
pub enum OrderingParseError {
    #[error("Invalid unit {0}, only `px` and `rem` are supported")]
    InvalidUnit(SmolStr),
    #[error("Invalid value {0}, expected a u64 integer")]
    InvalidValue(#[from] std::num::ParseIntError),
}

static PX_PER_REM: u64 = 16;

impl VariantOrdering {
    pub fn from_length(s: &str) -> Result<Self, OrderingParseError> {
        match s {
            _ if s.ends_with("px") => {
                let value = s.trim_end_matches("px").parse::<u64>()?;
                Ok(Self::Length(value))
            }
            _ if s.ends_with("rem") => {
                let value = s.trim_end_matches("rem").parse::<u64>()?;
                Ok(Self::Length(value * PX_PER_REM))
            }
            _ => Err(OrderingParseError::InvalidUnit(s.into())),
        }
    }
}

impl Variant {
    pub fn new_static<T>(matcher: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<SmolStr>,
        T::IntoIter: ExactSizeIterator,
    {
        let handler = StaticHandler::new(matcher);
        Self {
            nested: handler.is_nested(),
            handler: VariantHandler::Static(handler),
            composable: true,
            kind: VariantKind::Static,
            ordering: VariantOrdering::Unset,
        }
    }

    pub fn new_composable(handler: fn(RuleList, &VariantCandidate) -> RuleList) -> Self {
        Self {
            handler: VariantHandler::Composable(ComposableHandler::new(handler)),
            composable: true,
            kind: VariantKind::Composable,
            ordering: VariantOrdering::Unset,
            // composable variants are always nested
            nested: false,
        }
    }

    pub fn new_dynamic(handler: fn(RuleList, &VariantCandidate) -> RuleList, nested: bool) -> Self {
        Self {
            handler: VariantHandler::Dynamic(DynamicHandler::new(handler)),
            composable: true,
            kind: VariantKind::Dynamic,
            ordering: VariantOrdering::Unset,
            nested,
        }
    }

    pub fn with_ordering(self, ordering: VariantOrdering) -> Self {
        Self { ordering, ..self }
    }

    pub fn process(&self, candidate: &VariantCandidate<'_>, rule: RuleList) -> RuleList {
        match &self.handler {
            VariantHandler::Static(handler) => handler.handle(candidate, rule),
            VariantHandler::Dynamic(handler) => handler.handle(candidate, rule),
            VariantHandler::Composable(handler) => handler.handle(candidate, rule),
        }
    }

    pub fn take_composable(&self) -> Option<&ComposableHandler> {
        match &self.handler {
            VariantHandler::Composable(handler) => Some(handler),
            _ => None,
        }
    }

    pub fn is_composable(&self) -> bool {
        self.composable
    }
}

impl VariantHandlerExt for Variant {
    fn handle(&self, candidate: &VariantCandidate<'_>, rule: RuleList) -> RuleList {
        self.process(candidate, rule)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StaticHandler {
    // for single rule
    Selector(SmolStr),
    // for single rule
    PseudoElement(SmolStr),
    // for multiple rules
    Nested(SmolStr),
    // for multiple rules
    Duplicate(SmallVec<[SmolStr; 2]>),
}

impl StaticHandler {
    pub fn new<T>(matcher: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<SmolStr>,
        T::IntoIter: ExactSizeIterator,
    {
        let mut iter = matcher.into_iter();
        let is_duplicate = iter.len() > 1;
        if !is_duplicate {
            let next = iter.next().unwrap().into();
            match next.chars().next() {
                Some('&') => {
                    if next.starts_with("&::") {
                        Self::PseudoElement(next)
                    } else {
                        Self::Selector(next)
                    }
                }
                Some('@') => Self::Nested(next),
                _ => Self::Selector(format_smolstr!("&:is({})", next)),
            }
        } else {
            Self::new_duplicate(iter)
        }
    }

    pub fn new_duplicate<T>(matcher: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<SmolStr>,
    {
        Self::Duplicate(matcher.into_iter().map(Into::into).collect())
    }

    pub fn is_nested(&self) -> bool {
        matches!(self, Self::Nested(_))
    }
}

impl VariantHandlerExt for StaticHandler {
    fn handle(&self, _candidate: &VariantCandidate<'_>, rules: RuleList) -> RuleList {
        match self {
            Self::Selector(a) | Self::PseudoElement(a) => rules
                .into_iter()
                .map(|rule| rule.modify_with(|selector| selector.replace_char('&', a)))
                .collect(),
            Self::Nested(a) => RuleList::new(Rule { selector: a.clone(), decls: vec![], rules }),
            Self::Duplicate(list) => list
                .iter()
                .flat_map(move |a| {
                    rules
                        .clone()
                        .into_iter()
                        .map(|rule| rule.modify_with(|selector| selector.replace_char('&', a)))
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynamicHandler {
    pub handler: fn(RuleList, &VariantCandidate) -> RuleList,
    pub composable: bool,
}

impl VariantHandlerExt for DynamicHandler {
    fn handle(&self, candidate: &VariantCandidate<'_>, rule: RuleList) -> RuleList {
        (self.handler)(rule, candidate)
    }
}

impl DynamicHandler {
    pub fn new(handler: fn(RuleList, &VariantCandidate) -> RuleList) -> Self {
        Self { handler, composable: true }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComposableHandler {
    pub handler: fn(RuleList, &VariantCandidate) -> RuleList,
    pub composable: bool,
}

impl ComposableHandler {
    pub fn new(handler: fn(RuleList, &VariantCandidate) -> RuleList) -> Self {
        Self { handler, composable: true }
    }

    pub fn composable(self) -> Self {
        Self { composable: true, ..self }
    }
}

impl VariantHandlerExt for ComposableHandler {
    fn handle(&self, candidate: &VariantCandidate<'_>, rule: RuleList) -> RuleList {
        (self.handler)(rule, candidate)
    }
}

#[cfg(test)]
mod tests {
    use rswind_css::{rule::RuleList, Decl, Rule, css};
    use smol_str::format_smolstr;

    use super::{DynamicHandler, VariantHandlerExt};
    use crate::{design::DesignSystem, parse::candidate::CandidateParser};

    #[test]
    fn test_variant_process() {
        let mut design = DesignSystem::default();
        design.add_variant("hover", ["&:hover"]);
        design.add_variant("active", ["&:active"]);

        let candidates = vec![
            CandidateParser::new("hover").parse_variant(&design.variants).unwrap(),
            CandidateParser::new("active").parse_variant(&design.variants).unwrap(),
        ];

        let _input = css! {
            ".flex" {
                "display": "flex";
            }
        };

        let selector = RuleList::new(Rule {
            selector: "&".into(),
            rules: RuleList::default(),
            decls: vec![Decl { name: "display".into(), value: "flex".into() }],
        });

        let _res = candidates
            .into_iter()
            .map(|candidate| {
                let processor = design.variants.get(candidate.key).unwrap();
                (processor, candidate)
            })
            .fold(selector, |acc, (processor, candidate)| processor.process(&candidate, acc));
    }

    #[test]
    fn test_dynamic_process() {
        let mut design = DesignSystem::default();
        design.add_variant("hover", ["&:hover"]);
        design.add_variant("active", ["&:active"]);

        let candidate = CandidateParser::new("hover").parse_variant(&design.variants).unwrap();
        let input = css! {
            ".flex" {
                "display": "flex";
            }
        }
        .to_rule_list();
        // @media (hover: hover) and (pointer: fine) | &:hover
        let variant = DynamicHandler::new(|rule, _| {
            let hovered = rule
                .into_iter()
                .map(|rule| rule.modify_with(|s| format_smolstr!("{}:hover", s)))
                .collect();
            Rule {
                selector: "@media (hover: hover) and (pointer: fine)".into(),
                decls: vec![],
                rules: hovered,
            }
            .to_rule_list()
        });

        let res = variant.handle(&candidate, input);

        println!("{:#?}", res);
    }
}
