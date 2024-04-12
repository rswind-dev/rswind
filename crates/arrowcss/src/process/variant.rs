use std::cmp::Ordering;

use either::Either;
use smallvec::SmallVec;

use crate::{
    css::{rule::RuleList, Rule},
    parsing::VariantCandidate,
};

#[rustfmt::skip]
pub trait VariantMatchingFn: Fn(RuleList) -> Option<RuleList> + Sync + Send {}

#[rustfmt::skip]
impl<T: Fn(RuleList) -> Option<RuleList> + Sync + Send> VariantMatchingFn for T {}

// hover -> &:hover not-hover -> &:not(:hover)
#[derive(Debug, Clone)]
pub struct VariantProcessor {
    pub handler: Either<StaticHandler, DynamicHandler>,
    pub composable: bool,
}

impl VariantProcessor {
    pub fn new_static(
        matcher: impl IntoIterator<Item: Into<String>, IntoIter: ExactSizeIterator>,
    ) -> Self {
        Self {
            handler: Either::Left(StaticHandler::new(matcher)),
            composable: true,
        }
    }

    pub fn process<'a>(
        &self,
        candidate: VariantCandidate,
        rule: RuleList<'a>,
    ) -> RuleList<'a> {
        match self.handler {
            Either::Left(ref handler) => handler.process(candidate, rule),
            Either::Right(ref handler) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StaticHandler {
    // for single rule
    Selector(String),
    // for single rule
    PesudoElement(String),
    // for multiple rules
    Nested(String),
    // for multiple rules
    Duplicate(SmallVec<[String; 2]>),
}

impl StaticHandler {
    pub fn new(
        matcher: impl IntoIterator<Item: Into<String>, IntoIter: ExactSizeIterator>,
    ) -> Self {
        let mut iter = matcher.into_iter();
        let is_duplicate = iter.len() > 1;
        if !is_duplicate {
            let matcher = iter.next().unwrap().into();
            match matcher.chars().next() {
                Some('&') => {
                    if matcher.starts_with("&::") {
                        Self::PesudoElement(matcher)
                    } else {
                        Self::Selector(matcher)
                    }
                }
                Some('@') => Self::Nested(matcher),
                _ => panic!("Invalid matcher: {}", matcher),
            }
        } else {
            Self::new_duplicate(iter)
        }
    }

    pub fn new_duplicate(
        matcher: impl IntoIterator<Item: Into<String>>,
    ) -> Self {
        Self::Duplicate(matcher.into_iter().map(Into::into).collect())
    }

    pub fn process<'a>(
        &self,
        candidate: VariantCandidate,
        rules: RuleList<'a>,
    ) -> RuleList<'a> {
        match self {
            Self::Selector(a) | Self::PesudoElement(a) => rules
                .into_iter()
                .map(|rule| {
                    rule.modify_with(|selector| selector.replace("&", a))
                })
                .collect(),
            Self::Nested(a) => RuleList::new(Rule {
                selector: a.clone(),
                decls: vec![],
                rules,
            }),
            Self::Duplicate(list) => list
                .iter()
                .flat_map(move |a| {
                    rules.clone().into_iter().map(|rule| {
                        rule.modify_with(|selector| selector.replace("&", a))
                    })
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicHandler {}

pub enum VariantHandler {
    Nested(Box<dyn VariantMatchingFn>),
    Selector(Box<dyn VariantMatchingFn>),
}

#[rustfmt::skip]
pub trait ComposeVairantFn: Fn(VariantHandler) -> VariantHandler {}

impl VariantHandler {
    pub fn as_handler(&self) -> &Box<dyn VariantMatchingFn> {
        match self {
            Self::Nested(f) => f,
            Self::Selector(f) => f,
        }
    }

    pub fn create_constructor(
        &self,
    ) -> impl Fn(Box<dyn VariantMatchingFn>) -> Self {
        match self {
            Self::Nested(_) => VariantHandler::Nested,
            Self::Selector(_) => VariantHandler::Selector,
        }
    }
}

fn variant_fn(matcher: String) -> Option<VariantHandler> {
    let m = matcher.get(1..)?.to_owned();
    match matcher.chars().next()? {
        '&' => Some(VariantHandler::Selector(Box::new(
            move |container: RuleList| {
                Some(
                    container
                        .into_iter()
                        .map(|rule| Rule {
                            selector: rule.selector + &m,
                            decls: rule.decls,
                            rules: rule.rules,
                        })
                        .collect(),
                )
            },
        ))),
        '@' => Some(VariantHandler::Nested(Box::new(move |rule| {
            Some(
                Rule {
                    selector: m.clone(),
                    decls: vec![],
                    rules: rule,
                }
                .into(),
            )
        }))),
        _ => None,
    }
}

pub fn create_variant_fn<T>(_key: &str, matcher: T) -> Option<VariantHandler>
where
    T: IntoIterator,
    T::Item: AsRef<str>,
    T::IntoIter: ExactSizeIterator,
{
    let mut has_selector_handler = false;
    let fns = matcher
        .into_iter()
        // .map(|item| item.as_ref())
        .map(|s| {
            let s = s.as_ref();
            let this_fn: VariantHandler = if s.find('|').is_some() {
                let mut fns = s
                    .split('|')
                    .map(|matcher| matcher.trim())
                    .map(|item| variant_fn(item.into()))
                    .collect::<Option<Vec<_>>>()?;

                fns.sort();

                let wrapper = VariantHandler::create_constructor(&fns[0]);
                let composed_fn: Box<dyn VariantMatchingFn> =
                    Box::new(move |rules| {
                        fns.iter().try_fold(rules, |acc, f| f(acc))
                    });
                wrapper(composed_fn)
            } else {
                // Normal
                variant_fn(s.into())?
            };
            if matches!(this_fn, VariantHandler::Selector(_)) {
                has_selector_handler = true;
            }
            Some(this_fn)
        })
        .collect::<Option<Vec<_>>>()?;

    let handler: Box<dyn VariantMatchingFn> =
        Box::new(move |container: RuleList| {
            Some(
                fns.iter()
                    .map(|f| f(container.clone()))
                    .collect::<Option<Vec<RuleList>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<Rule>>()
                    .into(),
            )
        });

    Some(if has_selector_handler {
        VariantHandler::Selector(handler)
    } else {
        VariantHandler::Nested(handler)
    })
}

// ----- Trait Implementations -----

impl PartialEq for VariantHandler {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Nested(_), Self::Nested(_))
                | (Self::Selector(_), Self::Selector(_))
        )
    }
}

impl Eq for VariantHandler {}

impl PartialOrd for VariantHandler {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Nested(_), Self::Selector(_)) => Some(Ordering::Greater),
            (Self::Selector(_), Self::Nested(_)) => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Ord for VariantHandler {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> Fn<(RuleList<'a>,)> for VariantHandler {
    extern "rust-call" fn call(
        &self,
        args: (RuleList<'a>,),
    ) -> Option<RuleList<'a>> {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Selector(f) => f(args.0),
        }
    }
}

impl<'a> FnOnce<(RuleList<'a>,)> for VariantHandler {
    type Output = Option<RuleList<'a>>;

    extern "rust-call" fn call_once(
        self,
        args: (RuleList<'a>,),
    ) -> Self::Output {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Selector(f) => f(args.0),
        }
    }
}

impl<'a> FnMut<(RuleList<'a>,)> for VariantHandler {
    extern "rust-call" fn call_mut(
        &mut self,
        args: (RuleList<'a>,),
    ) -> Option<RuleList<'a>> {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Selector(f) => f(args.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use arrowcss_css_macro::css;

    use crate::{
        context::Context,
        css::{rule::RuleList, Decl, Rule},
        parsing::VariantParser,
    };

    #[test]
    fn test_variant_process() {
        let mut ctx = Context::default();
        ctx.add_variant("hover", ["&:hover"]);
        ctx.add_variant("active", ["&:active"]);

        let candidates = vec![
            VariantParser::new("hover").parse(&ctx).unwrap(),
            VariantParser::new("active").parse(&ctx).unwrap(),
        ];

        let input = css! {
            ".flex" {
                "display": "flex";
            }
        };

        let selector = RuleList::new(Rule {
            selector: "&".to_string(),
            rules: RuleList::default(),
            decls: vec![Decl {
                name: "display".into(),
                value: "flex".into(),
            }],
        });

        let res = candidates
            .into_iter()
            .map(|candidate| {
                let processor = ctx.variants.get(candidate.key).unwrap();
                (processor, candidate)
            })
            .fold(selector, |acc, (processor, candidate)| {
                processor.process(candidate, acc)
            });

        // let res = res
        //     .into_iter()
        //     .map(|res| Rule {
        //         selector: res.selector.replace("&", ".flex"),
        //         nodes: res.nodes,
        //     })
        //     .collect::<Vec<_>>();

        println!("{input:?}");
        println!("res: {:#?}", res);
        // expect: ".flex:hover"
    }
}
