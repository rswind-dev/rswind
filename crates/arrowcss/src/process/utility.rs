use lazy_static::lazy_static;
use lightningcss::values::string::CowArcStr;

use super::{ArbitraryValueProcessor, MetaData};
use crate::{
    css::{rule::RuleList, Rule},
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
}

pub struct ModifierProcessor<'i> {
    pub validator: Option<Box<dyn TypeValidator>>,
    pub allowed_values: Option<ThemeValue<'i>>,
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

    pub fn ordering(&mut self, key: OrderingKey) -> &mut Self {
        self.ordering_key = Some(key);
        self
    }

    pub fn validator(&mut self, validator: impl TypeValidator + 'static) -> &mut Self {
        self.validator = Some(Box::new(validator));
        self
    }

    pub fn apply_to<'a>(&self, candidate: UtilityCandidate<'a>) -> Option<(Rule<'c>, OrderingKey)>
    where
        'c: 'a,
    {
        if !self.supports_negative && candidate.negative {
            return None;
        }

        if candidate.is_fraction_like() && self.supports_fraction {
            todo!()
        }

        let process_result = self.process(candidate.value?)?;
        let mut meta = MetaData::new(candidate);

        // handing modifier
        if let (Some(modifier), Some(candidate)) = (&self.modifier, candidate.modifier) {
            meta.modifier = modifier.process(candidate).map(|v| v.to_string());
        }

        let mut node = self.handler.call(meta, process_result);

        if let Some(wrapper) = &self.wrapper {
            node.selector = wrapper.clone();
        }

        Some((
            node,
            self.ordering_key.unwrap_or(OrderingKey::Disorder),
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
