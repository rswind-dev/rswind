use std::{fmt::Debug, sync::Arc};

use cssparser::serialize_name;
use rswind_css::{rule::RuleList, Decl, Rule, ToCssString};
use rswind_theme::ThemeMap;
use smallvec::{smallvec, SmallVec};
use smol_str::{format_smolstr, SmolStr};

use super::{MetaData, ValueDef, ValuePreprocessor};
use crate::{
    ordering::OrderingKey,
    parsing::{AdditionalCssHandler, UtilityCandidate},
};

#[rustfmt::skip]
pub trait RuleMatchingFn: Fn(MetaData, SmolStr) -> Rule + Send + Sync + 'static {}

#[rustfmt::skip]
impl<T: Fn(MetaData, SmolStr) -> Rule + Send + Sync + 'static> RuleMatchingFn for T {}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct UtilityHandler(Box<dyn RuleMatchingFn>);

impl Debug for UtilityHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UtilityHandler { ")?;

        // Call the function, simply get the css
        let rule = self.0(MetaData::modifier("$2"), SmolStr::new("$1"));
        write!(f, "{}", rule.to_css_minified())?;

        f.write_str(" }")
    }
}

impl UtilityHandler {
    pub fn new(handler: impl RuleMatchingFn + 'static) -> Self {
        Self(Box::new(handler))
    }

    pub fn call(&self, meta: MetaData, value: SmolStr) -> Rule {
        (self.0)(meta, value)
    }
}

#[derive(Debug)]
pub struct Utility {
    pub handler: UtilityHandler,

    pub supports_negative: bool,

    pub supports_fraction: bool,

    pub value_def: ValueDef,

    pub modifier: Option<ValueDef>,

    /// This will be use as generated Rule selector
    /// default: '&'
    pub wrapper: Option<SmolStr>,

    /// Additional css which append to stylesheet root
    /// useful when utilities like `animate-spin`
    pub additional_css: Option<Box<dyn AdditionalCssHandler>>,

    pub ordering_key: Option<OrderingKey>,

    pub group: Option<UtilityGroup>,
}

// TODO: make this configurable
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub enum UtilityGroup {
    Transform,
    Filter,
    BackdropFilter,
}

impl UtilityGroup {
    pub fn as_decls(&self) -> SmallVec<[Decl; 2]> {
        match self {
            Self::Filter => smallvec![Decl::new(
                "filter", "var(--tw-blur,) var(--tw-brightness,) var(--tw-contrast,) var(--tw-grayscale,) var(--tw-hue-rotate,) var(--tw-invert,) var(--tw-saturate,) var(--tw-sepia,) var(--tw-drop-shadow,)"

            )],
            Self::BackdropFilter => smallvec![
                Decl::new("-webkit-backdrop-filter", "var(--tw-backdrop-blur,) var(--tw-backdrop-brightness,) var(--tw-backdrop-contrast,) var(--tw-backdrop-grayscale,) var(--tw-backdrop-hue-rotate,) var(--tw-backdrop-invert,) var(--tw-backdrop-opacity,) var(--tw-backdrop-saturate,) var(--tw-backdrop-sepia,)"),
                Decl::new("backdrop-filter", "var(--tw-backdrop-blur,) var(--tw-backdrop-brightness,) var(--tw-backdrop-contrast,) var(--tw-backdrop-grayscale,) var(--tw-backdrop-hue-rotate,) var(--tw-backdrop-invert,) var(--tw-backdrop-opacity,) var(--tw-backdrop-saturate,) var(--tw-backdrop-sepia,)")
            ],
            Self::Transform => smallvec![Decl::new(
                "transform", "var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)"
            )],
        }
    }
}
pub fn build_group_selector(selectors: impl IntoIterator<Item = SmolStr>) -> String {
    let mut selector = String::with_capacity(64);

    for (i, s) in selectors.into_iter().enumerate() {
        if i > 0 {
            selector.push_str(", ");
        }
        selector.push('.');
        let _ = serialize_name(&s, &mut selector);
    }

    selector
}

impl ValuePreprocessor for Utility {
    fn validate(&self, value: &str) -> bool {
        self.value_def.validate(value)
    }

    fn allowed_values(&self) -> Option<&ThemeMap> {
        self.value_def.allowed_values()
    }
}

impl<F: RuleMatchingFn + 'static> From<F> for Utility {
    fn from(handler: F) -> Self {
        Utility::new(handler)
    }
}

pub struct UtilityApplyResult {
    pub rule: Rule,
    pub ordering: OrderingKey,
    pub group: Option<UtilityGroup>,
    pub additional_css: Option<Arc<RuleList>>,
}

impl Utility {
    pub fn new<F: RuleMatchingFn + 'static>(handler: F) -> Self {
        Self {
            handler: UtilityHandler(Box::new(handler)),
            supports_negative: false,
            supports_fraction: false,
            value_def: ValueDef::default(),
            modifier: None,
            wrapper: None,
            additional_css: None,
            ordering_key: None,
            group: None,
        }
    }

    pub fn apply_to(&self, candidate: UtilityCandidate<'_>) -> Option<UtilityApplyResult> {
        if !self.supports_negative && candidate.negative {
            return None;
        }

        let preprocess = self.preprocess(candidate.value)?;

        let process_result = match preprocess.as_str() {
            Some(plain) => {
                let mut process_result = SmolStr::from(plain);
                if self.supports_fraction {
                    if let Some(fraction) = candidate.take_fraction() {
                        process_result = format_smolstr!("calc({} * 100%)", fraction);
                    }
                }

                if candidate.negative {
                    process_result = format_smolstr!("calc({} * -1)", process_result);
                }
                process_result
            }
            None => SmolStr::default(),
        };

        let mut meta = MetaData::from_candidate(&candidate).theme_value(preprocess);

        // handing modifier
        if let (Some(modifier), Some(candidate)) = (&self.modifier, candidate.modifier) {
            meta.modifier =
                modifier.preprocess(Some(candidate)).and_then(|v| v.as_str().map(Into::into));
        }

        let mut css = None;
        if let Some(additional_css) = &self.additional_css {
            css = additional_css.handle(candidate.value.unwrap_or_default().as_str().into());
        }

        let mut node = self.handler.call(meta, process_result);

        if let Some(wrapper) = &self.wrapper {
            node.selector.clone_from(wrapper);
        }

        Some(UtilityApplyResult {
            rule: node,
            ordering: self.ordering_key.unwrap_or(OrderingKey::Disorder),
            group: self.group,
            additional_css: css,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_css_macro() {
        let css = rswind_css_macro::css! {
            "@property --tw-translate-x" {
                "syntax": "<length-percentage>";
                "inherits": "false";
                "initial-value": "0";
            }
        };
        println!("{:?}", css);
    }
}
