use smallvec::{smallvec, SmallVec};
use smol_str::{format_smolstr, SmolStr};

use super::{ArbitraryValueProcessor, MetaData};
use crate::{
    css::{rule::RuleList, Decl, Rule},
    ordering::OrderingKey,
    parsing::UtilityCandidate,
    theme::ThemeValue,
    types::TypeValidator,
};

#[rustfmt::skip]
pub trait RuleMatchingFn: Fn(MetaData, SmolStr) -> Rule + Send + Sync + 'static {}

#[rustfmt::skip]
impl<T: Fn(MetaData, SmolStr) -> Rule + Send + Sync + 'static> RuleMatchingFn for T {}

pub struct UtilityHandler(Box<dyn RuleMatchingFn>);

impl UtilityHandler {
    pub fn new(handler: impl RuleMatchingFn + 'static) -> Self {
        Self(Box::new(handler))
    }

    pub fn call(&self, meta: MetaData, value: SmolStr) -> Rule {
        (self.0)(meta, value)
    }
}

pub struct Utility {
    pub handler: UtilityHandler,

    pub supports_negative: bool,

    pub supports_fraction: bool,

    pub allowed_values: Option<ThemeValue>,

    pub modifier: Option<ModifierProcessor>,

    pub validator: Option<Box<dyn TypeValidator>>,

    /// This will be use as generated Rule selector
    /// default: '&'
    pub wrapper: Option<SmolStr>,

    /// Additional css which append to stylesheet root
    /// useful when utilities like `animate-spin`
    pub additional_css: Option<RuleList>,

    pub ordering_key: Option<OrderingKey>,

    pub group: Option<UtilityGroup>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

pub struct UtilityOptions {
    pub ordering_key: Option<OrderingKey>,
    pub validator: Option<Box<dyn TypeValidator>>,
    pub supports_negative: bool,
    pub supports_fraction: bool,
}

impl Default for UtilityOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityOptions {
    pub fn new() -> Self {
        Self {
            ordering_key: None,
            validator: None,
            supports_negative: false,
            supports_fraction: false,
        }
    }

    pub fn ordering(&mut self, key: Option<OrderingKey>) -> &mut Self {
        self.ordering_key = key;
        self
    }

    pub fn validator(&mut self, validator: Option<impl TypeValidator + 'static>) -> &mut Self {
        self.validator = validator.map(|v| Box::new(v) as _);
        self
    }

    pub fn support_negative(&mut self, value: bool) -> &mut Self {
        self.supports_negative = value;
        self
    }

    pub fn support_fraction(&mut self, value: bool) -> &mut Self {
        self.supports_fraction = value;
        self
    }
}

pub struct ModifierProcessor {
    pub validator: Option<Box<dyn TypeValidator>>,
    pub allowed_values: Option<ThemeValue>,
}

impl ModifierProcessor {
    pub fn new(allowed_values: ThemeValue) -> Self {
        Self {
            validator: None,
            allowed_values: Some(allowed_values),
        }
    }

    pub fn with_validator(self, validator: impl TypeValidator + 'static) -> Self {
        Self {
            validator: Some(Box::new(validator)),
            allowed_values: self.allowed_values,
        }
    }
}

impl ArbitraryValueProcessor for ModifierProcessor {
    fn validate(&self, value: &str) -> bool {
        self.validator
            .as_ref()
            .map_or(true, |validator| validator.validate(value))
    }

    fn allowed_values(&self) -> Option<&ThemeValue> {
        self.allowed_values.as_ref()
    }
}

impl ArbitraryValueProcessor for Utility {
    fn validate(&self, value: &str) -> bool {
        self.validator
            .as_ref()
            .map_or(true, |validator| validator.validate(value))
    }

    fn allowed_values(&self) -> Option<&ThemeValue> {
        self.allowed_values.as_ref()
    }
}

impl<F: RuleMatchingFn + 'static> From<F> for Utility {
    fn from(handler: F) -> Self {
        Utility::new(handler)
    }
}

impl Utility {
    pub fn new<F: RuleMatchingFn + 'static>(handler: F) -> Self {
        Self {
            handler: UtilityHandler(Box::new(handler)),
            supports_negative: false,
            supports_fraction: false,
            allowed_values: None,
            modifier: None,
            validator: None,
            wrapper: None,
            additional_css: None,
            ordering_key: None,
            group: None,
        }
    }

    pub fn allow_values(mut self, values: ThemeValue) -> Self {
        self.allowed_values = Some(values);
        self
    }

    pub fn apply_options(mut self, options: UtilityOptions) -> Self {
        self.supports_negative = options.supports_negative;
        self.supports_fraction = options.supports_fraction;
        self.validator = options.validator;
        self.ordering_key = options.ordering_key;
        self
    }

    pub fn apply_to(
        &self,
        candidate: UtilityCandidate<'_>,
    ) -> Option<(Rule, OrderingKey, Option<UtilityGroup>)> {
        if !self.supports_negative && candidate.negative {
            return None;
        }

        let mut process_result = self.process(candidate.value)?;
        let mut meta = MetaData::new(candidate);

        // handing modifier
        if let (Some(modifier), Some(candidate)) = (&self.modifier, candidate.modifier) {
            meta.modifier = modifier.process(Some(candidate));
        }

        if self.supports_fraction {
            if let Some(fraction) = candidate.take_fraction() {
                process_result = format_smolstr!("calc({} * 100%)", fraction);
            }
        }

        if candidate.negative {
            process_result = format_smolstr!("calc({} * -1)", process_result);
        }

        let mut node = self.handler.call(meta, process_result);

        if let Some(wrapper) = &self.wrapper {
            node.selector.clone_from(wrapper);
        }

        Some((
            node,
            self.ordering_key.unwrap_or(OrderingKey::Disorder),
            self.group,
        ))
    }
}
