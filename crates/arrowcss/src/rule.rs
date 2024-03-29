use lightningcss::{traits::IntoOwned, values::string::CowArcStr};

use crate::{
    css::CssDecls,
    theme::ThemeValue,
    types::TypeValidator,
    utils::{decode_arbitrary_value, StripArbitrary},
};

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct MetaData {
    pub raw: String,
}

impl MetaData {
    pub(crate) fn new(raw: &str) -> Self {
        Self {
            raw: raw.to_owned(),
        }
    }
}

pub trait RuleMatchingFn = Fn(MetaData, CowArcStr) -> Option<CssDecls>;

pub struct Rule<'i> {
    handler: Box<dyn RuleMatchingFn>,
    #[allow(dead_code)]
    pub supports_negative: bool,
    // a Theme map
    pub allowed_values: Option<ThemeValue<'i>>,
    #[allow(dead_code)]
    pub allowed_modifiers: Option<ThemeValue<'i>>,
    // a lightningcss PropertyId
    pub infer_property_id: Option<Box<dyn TypeValidator>>,
}

impl<'c, F: RuleMatchingFn + 'static> From<F> for Rule<'c> {
    fn from(handler: F) -> Self {
        Rule::new(handler)
    }
}

impl<'c> Rule<'c> {
    pub fn new<F: RuleMatchingFn + 'static>(handler: F) -> Self {
        Self {
            handler: Box::new(handler),
            supports_negative: false,
            allowed_values: None,
            allowed_modifiers: None,
            infer_property_id: None,
        }
    }

    pub fn infer_by(mut self, id: impl TypeValidator + 'static) -> Self {
        self.infer_property_id = Some(Box::new(id));
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
    pub fn allow_modifiers(mut self, modifiers: ThemeValue<'c>) -> Self {
        self.allowed_modifiers = Some(modifiers);
        self
    }

    pub fn apply_to<'a>(&self, value: &'a str) -> Option<CssDecls<'c>> {
        // arbitrary value
        if let Some(stripped) = value.strip_arbitrary() {
            // TODO: add escape support
            let stripped = decode_arbitrary_value(stripped);
            // when infer_property_id is None, default not check it
            if let Some(validator) = &self.infer_property_id {
                if !validator.validate(&stripped) {
                    return None;
                }
            }

            return (self.handler)(
                MetaData {
                    raw: value.to_string(),
                },
                CowArcStr::from(stripped).into_owned(),
            );
        }

        // theme value
        if let Some(allowed_values) = &self.allowed_values {
            if let Some(v) = allowed_values.get(value) {
                return (self.handler)(
                    MetaData {
                        raw: value.to_string(),
                    },
                    v.clone().into_owned(),
                );
            }
        }

        None
    }

    // pub fn bind_context(
    //     self,
    //     ctx: Arc<Context<'i, 'c>>,
    // ) -> InContextRule<'i, 'c>
    // {
    //     InContextRule {
    //         rule: self,
    //         ctx: Arc::downgrade(&ctx),
    //     }
    // }
}

// pub struct InContextRule<'i, 'c> {
//     pub rule: Rule<'c>,
//     pub ctx: Weak<Context<'i, 'c>>,
// }

// impl<'i, 'c: 'i> InContextRule<'i, 'c> {
//     pub fn apply_to<'a>(&'a self, value: &'i str) -> Option<CssDecls<'i>> {
//         self.rule.apply_to(self.ctx.upgrade().unwrap(), value)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decls;
    use crate::types::PropertyId;

    #[test]
    fn test_rule_builder() {
        let rule = Rule::new(|_, _| None)
            .support_negative()
            .infer_by(PropertyId::FontSize);

        assert!(rule.supports_negative);
        assert!(rule.infer_property_id.is_some());
    }

    #[test]
    fn test_rule_handler() {
        // let ctx = Arc::new(Context::default());
        // let rule = Rule::new(|_, value| {
        //     Some(decls! {
        //         "font-size" => value.to_string(),
        //     })
        // })
        // .support_negative()
        // .infer_by(PropertyId::FontSize)
        // .bind_context(ctx.clone());

        // assert_eq!(
        //     rule.apply_to("[16px]"),
        //     Some(decls! {
        //         "font-size" => "16px",
        //     })
        // );

        // assert_eq!(
        //     rule.apply_to("[larger]"),
        //     Some(decls! {
        //         "font-size" => "larger",
        //     })
        // );

        // assert_eq!(
        //     rule.apply_to("[.5%]"),
        //     Some(decls! {
        //         "font-size" => ".5%",
        //     })
        // );
    }

    #[test]
    fn test_handle_background_position() {
        let rule = Rule::new(|_, value| {
            Some(decls! {
                "background-position" => value,
            })
        })
        .support_negative()
        .infer_by(PropertyId::BackgroundPosition);

        // let ctx = Arc::new(Context::default());

        assert_eq!(
            rule.apply_to("[top]"),
            Some(decls! {
                "background-position" => "top",
            })
        );

        assert_eq!(
            rule.apply_to("[center]"),
            Some(decls! {
                "background-position" => "center",
            })
        );

        assert_eq!(
            rule.apply_to("[50% 50%]"),
            Some(decls! {
                "background-position" => "50% 50%",
            })
        );

        assert_eq!(
            rule.apply_to("[50% top]"),
            Some(decls! {
                "background-position" => "50% top",
            })
        );

        assert_eq!(rule.apply_to("[top 50%]"), None);

        assert_eq!(
            rule.apply_to("[left 50%]"),
            Some(decls! {
                "background-position" => "left 50%",
            })
        );

        assert_eq!(
            rule.apply_to("[bottom 10px right 20px]"),
            Some(decls! {
                "background-position" => "bottom 10px right 20px",
            })
        );

        // enum Item<'i> {
        //     Custom(&'i str)
        // }

        // struct Theme<'i> {
        //     item: Item<'i>,
        //     ctx: Weak<Context<'i>>,
        // }

        // struct Context<'i> {
        //     theme: Theme<'i>,
        // }
    }
}
