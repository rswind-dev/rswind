use lazy_static::lazy_static;
use lightningcss::values::string::CowArcStr;

use super::{ArbitraryValueProcessor, MetaData};
use crate::{
    css::{rule::RuleList, Decl, Rule},
    ordering::OrderingKey,
    parsing::UtilityCandidate,
    theme::ThemeValue,
    types::TypeValidator,
};

#[rustfmt::skip]
pub trait RuleMatchingFn: for<'a, 'b> Fn(MetaData<'a>, CowArcStr<'b>) -> Rule<'b> + Send + Sync {}

#[rustfmt::skip]
impl<T> RuleMatchingFn for T where T: for<'a, 'b> Fn(MetaData<'a>, CowArcStr<'b>) -> Rule<'b> + Send + Sync {}

pub enum UtilityHandler {
    Static(for<'a, 'b> fn(MetaData<'a>, CowArcStr<'b>) -> Rule<'b>),
    Dynamic(Box<dyn RuleMatchingFn>),
}

lazy_static! {
    pub static ref NOOP: for<'a, 'b> fn(MetaData<'a>, CowArcStr<'b>) -> Rule<'b> =
        |_, _| Rule::default();
}

impl Default for UtilityHandler {
    fn default() -> Self {
        Self::Static(*NOOP)
    }
}

impl UtilityHandler {
    pub fn call<'a>(&self, meta: MetaData<'_>, value: CowArcStr<'a>) -> Rule<'a> {
        match self {
            Self::Static(handler) => handler(meta, value),
            Self::Dynamic(handler) => handler(meta, value),
        }
    }
}

#[derive(Default)]
pub struct Utility<'i> {
    pub handler: UtilityHandler,

    pub supports_negative: bool,

    pub supports_fraction: bool,

    pub allowed_values: Option<ThemeValue<'i>>,

    pub modifier: Option<ModifierProcessor<'i>>,

    pub validator: Option<Box<dyn TypeValidator>>,

    /// This will be use as generated Rule selector
    /// default: '&'
    pub wrapper: Option<String>,

    /// Additional css which append to stylesheet root
    /// useful when utilities like `animate-spin`
    pub additional_css: Option<RuleList<'i>>,

    pub ordering_key: Option<OrderingKey>,

    pub group: Option<UtilityGroup>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum UtilityGroup {
    Filter,
    BackdropFilter,
}

impl UtilityGroup {
    pub fn as_decls(&self) -> Vec<Decl> {
        match self {
            Self::Filter => vec![Decl::new(
                "filter", "var(--tw-blur,) var(--tw-brightness,) var(--tw-contrast,) var(--tw-grayscale,) var(--tw-hue-rotate,) var(--tw-invert,) var(--tw-saturate,) var(--tw-sepia,) var(--tw-drop-shadow,)"

            )],
            Self::BackdropFilter => vec![
                Decl::new("-webkit-backdrop-filter", "var(--tw-backdrop-blur,) var(--tw-backdrop-brightness,) var(--tw-backdrop-contrast,) var(--tw-backdrop-grayscale,) var(--tw-backdrop-hue-rotate,) var(--tw-backdrop-invert,) var(--tw-backdrop-opacity,) var(--tw-backdrop-saturate,) var(--tw-backdrop-sepia,)"),
                Decl::new("backdrop-filter", "var(--tw-backdrop-blur,) var(--tw-backdrop-brightness,) var(--tw-backdrop-contrast,) var(--tw-backdrop-grayscale,) var(--tw-backdrop-hue-rotate,) var(--tw-backdrop-invert,) var(--tw-backdrop-opacity,) var(--tw-backdrop-saturate,) var(--tw-backdrop-sepia,)")
            ],
        }
    }
}

pub struct UtilityOptions {
    pub ordering_key: Option<OrderingKey>,
    pub validator: Option<Box<dyn TypeValidator>>,
    pub supports_negative: bool,
    pub supports_fraction: bool,
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

pub struct ModifierProcessor<'i> {
    pub validator: Option<Box<dyn TypeValidator>>,
    pub allowed_values: Option<ThemeValue<'i>>,
}

impl<'i> ModifierProcessor<'i> {
    pub fn new(allowed_values: ThemeValue<'i>) -> Self {
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

impl<'a> ArbitraryValueProcessor<'a> for ModifierProcessor<'a> {
    fn validate(&self, value: &str) -> bool {
        self.validator
            .as_ref()
            .map_or(true, |validator| validator.validate(value))
    }

    fn allowed_values(&self) -> Option<&ThemeValue<'a>> {
        self.allowed_values.as_ref()
    }
}

impl<'a> ArbitraryValueProcessor<'a> for Utility<'a> {
    fn validate(&self, value: &str) -> bool {
        self.validator
            .as_ref()
            .map_or(true, |validator| validator.validate(value))
    }

    fn allowed_values(&self) -> Option<&ThemeValue<'a>> {
        self.allowed_values.as_ref()
    }
}

impl<'c, F: RuleMatchingFn + 'static> From<F> for Utility<'c> {
    fn from(handler: F) -> Self {
        Utility::new(handler)
    }
}

impl<'c> Utility<'c> {
    pub fn new<F: RuleMatchingFn + 'static>(handler: F) -> Self {
        Self {
            handler: UtilityHandler::Dynamic(Box::new(handler)),
            ..Default::default()
        }
    }

    pub fn allow_values(mut self, values: ThemeValue<'c>) -> Self {
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

    pub fn apply_to<'a>(
        &self,
        candidate: UtilityCandidate<'a>,
    ) -> Option<(Rule<'c>, OrderingKey, Option<UtilityGroup>)>
    where
        'c: 'a,
    {
        if !self.supports_negative && candidate.negative {
            return None;
        }

        let mut process_result = self.process(candidate.value)?;
        let mut meta = MetaData::new(candidate);

        // handing modifier
        if let (Some(modifier), Some(candidate)) = (&self.modifier, candidate.modifier) {
            meta.modifier = modifier.process(Some(candidate)).map(|v| v.to_string());
        }

        if self.supports_fraction {
            if let Some(fraction) = candidate.take_fraction() {
                process_result = format!("calc({} * 100%)", fraction).into();
            }
        }

        if candidate.negative {
            process_result = format!("calc({} * -1)", process_result).into();
        }

        let mut node = self.handler.call(meta, process_result);

        if let Some(wrapper) = &self.wrapper {
            node.selector = wrapper.clone();
        }

        Some((
            node,
            self.ordering_key.unwrap_or(OrderingKey::Disorder),
            self.group,
        ))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_rule_builder() {
        // let rule = UtilityProcessor::new(|MetaData { modifier, .. }, value| {
        //     let mut res = css!("font-size": value);
        //     if let Some(modifier) = modifier {
        //         res.extend(css!("line-height": modifier));
        //     }
        //     res
        // })
        // .support_negative()
        // .infer_by(PropertyId::FontSize)
        // .allow_modifier(ModifierProcessor {
        //     validator: Some(Box::new(PropertyId::LineHeight)),
        //     allowed_values: None,
        // });

        // let res = rule.apply_to(UtilityCandidate {
        //     key: "text",
        //     value: MaybeArbitrary::Arbitrary("16px"),
        //     modifier: Some(MaybeArbitrary::Arbitrary("1.5rem")),
        //     arbitrary: false,
        //     important: false,
        //     negative: false,
        // });

        // println!("{:?}", res);
    }

    #[test]
    fn test_rule_handler() {
        // let ctx = Arc::new(Context::default());
        // let rule = Rule::new(|_, value| {
        //     Some(decls! {
        //         "font-size": value.to_string();
        //     })
        // })
        // .support_negative()
        // .infer_by(PropertyId::FontSize)
        // .bind_context(ctx.clone());

        // assert_eq!(
        //     rule.apply_to("[16px]"),
        //     Some(decls! {
        //         "font-size": "16px";
        //     })
        // );

        // assert_eq!(
        //     rule.apply_to("[larger]"),
        //     Some(decls! {
        //         "font-size": "larger";
        //     })
        // );

        // assert_eq!(
        //     rule.apply_to("[.5%]"),
        //     Some(decls! {
        //         "font-size": ".5%";
        //     })
        // );
    }

    #[test]
    fn test_handle_background_position() {
        // let rule = UtilityProcessor::new(|_, value| {
        //     css! {
        //         "background-position": value;
        //     }
        // })
        // .support_negative()
        // .infer_by(PropertyId::BackgroundPosition);

        // // let ctx = Arc::new(Context::default());

        // assert_eq!(
        //     rule.apply_to("[top]"),
        //     Some(css! {
        //         "background-position": "top";
        //     })
        // );

        // assert_eq!(
        //     rule.apply_to("[center]").unwrap(),
        //     css! {
        //         "background-position": "center";
        //     }
        // );

        // assert_eq!(
        //     rule.apply_to("[50% 50%]").unwrap(),
        //     css! {
        //         "background-position": "50% 50%";
        //     }
        // );

        // assert_eq!(
        //     rule.apply_to("[50% top]").unwrap(),
        //     css! {
        //         "background-position": "50% top";
        //     }
        // );

        // assert_eq!(
        //     rule.apply_to("[left 50%]").unwrap(),
        //     css! {
        //         "background-position": "left 50%";
        //     }
        // );

        // assert_eq!(
        //     rule.apply_to("[bottom 10px right 20px]").unwrap(),
        //     css! {
        //         "background-position": "bottom 10px right 20px";
        //     }
        // );
    }
}
