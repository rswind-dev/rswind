use lightningcss::{traits::IntoOwned, values::string::CowArcStr};
use lazy_static::lazy_static;
use crate::{
    common::MaybeArbitrary, css::NodeList, theme::ThemeValue,
    types::TypeValidator, utility::UtilityCandidate,
};

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct MetaData<'a> {
    pub candidate: UtilityCandidate<'a>,
    pub modifier: Option<String>,
}

impl<'a> MetaData<'a> {
    pub(crate) fn new(candidate: UtilityCandidate<'a>) -> Self {
        Self {
            candidate,
            modifier: None,
        }
    }
}

pub trait RuleMatchingFn =
    for<'a, 'b> Fn(MetaData<'a>, CowArcStr<'b>) -> NodeList<'b> + Send + Sync;

pub enum UtilityHandler {
    Static(for<'a, 'b> fn(MetaData<'a>, CowArcStr<'b>) -> NodeList<'b>),
    Dynamic(Box<dyn RuleMatchingFn>),
}

lazy_static! {
    pub static ref NOOP: for<'a, 'b> fn(MetaData<'a>, CowArcStr<'b>) -> NodeList<'b> =
        |_, _| vec![];
}

impl Default for UtilityHandler {
    fn default() -> Self {
        Self::Static(*NOOP)
    }
}

impl UtilityHandler {
    pub fn call<'a, 'b>(
        &self,
        meta: MetaData<'b>,
        value: CowArcStr<'a>,
    ) -> NodeList<'a> {
        match self {
            Self::Static(handler) => handler(meta, value),
            Self::Dynamic(handler) => handler(meta, value),
        }
    }
}

pub struct UtilityProcessor<'i> {
    pub handler: UtilityHandler,

    pub supports_negative: bool,

    pub supports_fraction: bool,
    // a Theme map
    pub allowed_values: Option<ThemeValue<'i>>,

    pub modifier: Option<ModifierProcessor<'i>>,

    pub validator: Option<Box<dyn TypeValidator>>,
}

pub struct ModifierProcessor<'i> {
    // handler: for<'a> fn(NodeList<'a>, CowArcStr<'static>) -> NodeList<'a>,
    validator: Option<Box<dyn TypeValidator>>,
    allowed_values: Option<ThemeValue<'i>>,
}

pub trait ArbitraryValueProcessor<'a> {
    fn validate(&self, value: &str) -> bool;
    fn allowed_values(&self) -> Option<&ThemeValue<'a>>;

    fn process<'b>(
        &self,
        value: MaybeArbitrary<'b>,
    ) -> Option<CowArcStr<'static>> {
        match value {
            MaybeArbitrary::Arbitrary(value) => self
                .validate(value)
                .then(|| CowArcStr::from(value).into_owned()),
            MaybeArbitrary::Named(value) => self
                .allowed_values()?
                .get(value)
                .map(|v| v.clone().into_owned()),
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

impl<'a> ArbitraryValueProcessor<'a> for UtilityProcessor<'a> {
    fn validate(&self, value: &str) -> bool {
        self.validator
            .as_ref()
            .map_or(true, |validator| validator.validate(value))
    }

    fn allowed_values(&self) -> Option<&ThemeValue<'a>> {
        self.allowed_values.as_ref()
    }
}

impl<'c, F: RuleMatchingFn + 'static> From<F> for UtilityProcessor<'c> {
    fn from(handler: F) -> Self {
        UtilityProcessor::new(handler)
    }
}

impl<'c> UtilityProcessor<'c> {
    pub fn new<F: RuleMatchingFn + 'static>(handler: F) -> Self {
        Self {
            handler: UtilityHandler::Dynamic(Box::new(handler)),
            supports_fraction: false,
            supports_negative: false,
            allowed_values: None,
            modifier: None,
            validator: None,
        }
    }

    pub fn infer_by(mut self, id: impl TypeValidator + 'static) -> Self {
        self.validator = Some(Box::new(id));
        self
    }

    #[allow(dead_code)]
    pub fn support_negative(mut self) -> Self {
        self.supports_negative = true;
        self
    }

    pub fn allow_values(mut self, values: ThemeValue<'c>) -> Self {
        self.allowed_values = Some(values);
        self
    }

    #[allow(dead_code)]
    pub fn allow_modifier(mut self, modifier: ModifierProcessor<'c>) -> Self {
        self.modifier = Some(modifier);
        self
    }

    // Rules:
    // 1. try_apply can return a Vec of AstNode
    // 2. this Vec can only contain one Rule, which will be flatten as root of this
    //    e. g.
    //    & > :not([hidden]) ~ :not([hidden]) {
    //        color: #123456;
    //    }
    //    will be flatten as
    //   .${value} > :not([hidden]) ~ :not([hidden]) {
    //        color: #123456;
    //    }
    // 3. this Vec can contain multiple AtRule, and attach to the root
    pub fn apply_to<'a>(
        &self,
        candidate: UtilityCandidate<'a>,
    ) -> Option<NodeList<'c>> {
        if !self.supports_negative && candidate.negative {
            return None;
        }
        if candidate.is_fraction_like() && self.supports_fraction {
            todo!()
        }
        let process_result = self.process(candidate.value)?;
        let mut meta = MetaData::new(candidate);

        // handing modifier
        if let (Some(modifier), Some(candidate)) =
            (&self.modifier, candidate.modifier)
        {
            meta.modifier = modifier.process(candidate).map(|v| v.to_string());
        }

        let node = self.handler.call(meta, process_result);

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use arrowcss_css_macro::css;

    use super::*;
    use crate::{css::AstNode, types::PropertyId};

    #[test]
    fn test_rule_builder() {
        let rule = UtilityProcessor::new(|MetaData { modifier, .. }, value| {
            let mut res = css!("font-size": value);
            if let Some(modifier) = modifier {
                res.extend(css!("line-height": modifier));
            }
            res
        })
        .support_negative()
        .infer_by(PropertyId::FontSize)
        .allow_modifier(ModifierProcessor {
            validator: Some(Box::new(PropertyId::LineHeight)),
            allowed_values: None,
        });

        let res = rule.apply_to(UtilityCandidate {
            key: "text",
            value: MaybeArbitrary::Arbitrary("16px"),
            modifier: Some(MaybeArbitrary::Arbitrary("1.5rem")),
            arbitrary: false,
            important: false,
            negative: false,
        });

        println!("{:?}", res);
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
