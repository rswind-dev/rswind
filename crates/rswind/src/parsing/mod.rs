pub mod candidate;
pub mod state;

use std::{fmt::Debug, sync::Arc};

use rswind_css::rule::RuleList;
use rswind_theme::Theme;
use serde::Deserialize;
use smallvec::{smallvec, SmallVec};
use smol_str::{format_smolstr, SmolStr};

use crate::{
    common::MaybeArbitrary,
    ordering::OrderingKey,
    process::{
        ComposableHandler, RawValueDef, RuleMatchingFn, ThemeParseError, Utility, UtilityGroup,
        UtilityHandler, Variant, VariantHandlerExt,
    },
    types::TypeValidator,
};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct UtilityCandidate<'a> {
    pub key: &'a str,
    pub value: Option<MaybeArbitrary<'a>>,
    pub modifier: Option<MaybeArbitrary<'a>>,
    // fully arbitrary, e.g. [color:red] [text:--my-font-size]
    pub arbitrary: bool,
    pub important: bool,
    pub negative: bool,
}

impl<'a> UtilityCandidate<'a> {
    pub fn with_key(key: &'a str) -> Self {
        Self { key, ..Default::default() }
    }

    // only if value and modifier are both named
    pub fn take_fraction(&self) -> Option<SmolStr> {
        match (self.value, self.modifier) {
            (Some(MaybeArbitrary::Named(v)), Some(MaybeArbitrary::Named(m))) => {
                Some(format_smolstr!("{v}/{m}",))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ThemeKey {
    Single(SmolStr),
    Multi(Vec<SmolStr>),
}

impl From<SmolStr> for ThemeKey {
    fn from(s: SmolStr) -> Self {
        Self::Single(s)
    }
}

impl<const N: usize, T> From<[T; N]> for ThemeKey
where
    T: Into<SmolStr>,
{
    fn from(v: [T; N]) -> Self {
        Self::Multi(v.map(Into::into).into())
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct UtilityBuilder {
    /// The key of the utility， e.g. `bg`
    pub key: SmolStr,

    /// The css handler for the utility, e.g. `background-color: $1`
    #[serde(rename = "css")]
    pub handler: Option<UtilityHandler>,

    /// The modifier for the utility, e.g. `bg-blue-500/50 <-`
    #[serde(default)]
    pub modifier: Option<RawValueDef>,

    /// The theme key for the utility, will read from `theme` by this key later, e.g. `colors`
    #[serde(rename = "theme")]
    pub theme_key: Option<SmolStr>,

    /// The type validator for the utility, only used at `arbitrary values`
    ///
    /// e.g. `length-percentage` for `width`
    #[serde(rename = "type")]
    pub validator: Option<Box<dyn TypeValidator>>,

    /// The wrapper selector for the utility
    #[serde(default)]
    pub wrapper: Option<SmolStr>,

    /// Whether the utility supports negative values
    #[serde(default)]
    pub supports_negative: bool,

    /// Whether the utility supports fraction values, e.g. `w-1/2`
    #[serde(default)]
    pub supports_fraction: bool,

    #[serde(default)]
    pub ordering_key: Option<OrderingKey>,

    // TODO: add support for below fields
    #[serde(skip_deserializing)]
    pub additional_css: Option<Box<dyn AdditionalCssHandler>>,

    #[serde(skip_deserializing)]
    pub group: Option<UtilityGroup>,
}

pub trait AdditionalCssHandler: Sync + Send {
    fn handle(&self, value: SmolStr) -> Option<Arc<RuleList>>;
}

impl<T: Fn(SmolStr) -> Option<RuleList> + Sync + Send> AdditionalCssHandler for T {
    fn handle(&self, value: SmolStr) -> Option<Arc<RuleList>> {
        self(value).map(Arc::new)
    }
}

impl AdditionalCssHandler for Arc<RuleList> {
    fn handle(&self, _value: SmolStr) -> Option<Arc<RuleList>> {
        Some(self.clone())
    }
}

impl Debug for dyn AdditionalCssHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdditionalCssHandler").field("address", &format!("{:p}", self)).finish()
    }
}

impl UtilityBuilder {
    pub fn new(key: impl Into<SmolStr>, handler: impl RuleMatchingFn + 'static) -> Self {
        Self {
            key: key.into(),
            handler: Some(UtilityHandler::new(handler)),
            theme_key: None,
            supports_negative: false,
            supports_fraction: false,
            modifier: None,
            validator: None,
            additional_css: None,
            wrapper: None,
            ordering_key: None,
            group: None,
        }
    }

    pub fn parse(self, theme: &Theme) -> Result<(SmolStr, Utility), ThemeParseError> {
        Ok((
            self.key,
            Utility {
                handler: self.handler.unwrap(),
                supports_negative: self.supports_negative,
                supports_fraction: self.supports_fraction,
                value_def: RawValueDef { theme_key: self.theme_key, validator: self.validator }
                    .parse(theme)?,
                modifier: self.modifier.map(|m| m.parse(theme)).transpose()?,
                wrapper: self.wrapper,
                additional_css: self.additional_css,
                ordering_key: self.ordering_key,
                group: self.group,
            },
        ))
    }

    pub fn with_theme(&mut self, key: impl Into<SmolStr>) -> &mut Self {
        self.theme_key = Some(key.into());
        self
    }

    pub fn support_negative(&mut self) -> &mut Self {
        self.supports_negative = true;
        self
    }

    pub fn support_fraction(&mut self) -> &mut Self {
        self.supports_fraction = true;
        self
    }

    pub fn with_modifier(&mut self, modifier: RawValueDef) -> &mut Self {
        self.modifier = Some(modifier);
        self
    }

    pub fn with_validator(&mut self, validator: impl TypeValidator + 'static) -> &mut Self {
        self.validator = Some(Box::new(validator));
        self
    }

    pub fn with_additional_css(&mut self, css: impl AdditionalCssHandler + 'static) -> &mut Self {
        self.additional_css = Some(Box::new(css));
        self
    }

    pub fn with_wrapper(&mut self, wrapper: &str) -> &mut Self {
        self.wrapper = Some(wrapper.into());
        self
    }

    pub fn with_ordering(&mut self, key: OrderingKey) -> &mut Self {
        self.ordering_key = Some(key);
        self
    }

    pub fn with_group(&mut self, group: UtilityGroup) -> &mut Self {
        self.group = Some(group);
        self
    }
}

#[derive(Debug, Clone)]
pub struct VariantCandidate<'a> {
    pub key: &'a str,
    pub value: Option<MaybeArbitrary<'a>>,
    pub modifier: Option<MaybeArbitrary<'a>>,
    // fully arbitrary, e.g. [@media(min-width:300px)] [&:nth-child(3)]
    pub arbitrary: bool,
    pub processor: Variant,
    pub layers: SmallVec<[ComposableHandler; 1]>,
}

impl<'a> VariantCandidate<'a> {
    pub fn new(processor: Variant, key: &'a str) -> Self {
        Self { key, value: None, modifier: None, arbitrary: false, processor, layers: smallvec![] }
    }

    pub fn with_value(mut self, value: Option<MaybeArbitrary<'a>>) -> Self {
        self.value = value;
        self
    }

    pub fn with_modifier(mut self, modifier: Option<MaybeArbitrary<'a>>) -> Self {
        self.modifier = modifier;
        self
    }

    pub fn with_layers(mut self, layers: SmallVec<[ComposableHandler; 1]>) -> Self {
        self.layers = layers;
        self
    }

    pub fn arbitrary(mut self) -> Self {
        self.arbitrary = true;
        self
    }

    pub fn handle(&self, rule: RuleList) -> RuleList {
        let rule = self.processor.handle(self, rule);
        self.layers.iter().rev().fold(rule, |rule, handler| handler.handle(self, rule))
    }
}
