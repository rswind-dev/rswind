use lightningcss::{
    properties::{Property, PropertyId},
    values::string::CowArcStr,
};

use crate::{css::CssDecls, theme::ThemeValue, utils::StripArbitrary};

#[allow(dead_code)]
pub struct MetaData {
    pub raw: String,
}

// trait ContextExt<'i, 'c> {
//     fn with_meta(self, meta: MetaData) -> Arc<ExtendedContext<'i, 'c>>;
// }

// impl<'i, 'c: 'i> ContextExt<'i, 'c> for Arc<Context<'i, 'c>> {
//     fn with_meta(self, meta: MetaData) -> Arc<ExtendedContext<'i, 'c>> {
//         Arc::new(ExtendedContext {
//             ctx: self.clone(),
//             meta,
//         })
//     }
// }

// impl<'i, 'c> Deref for ExtendedContext<'i, 'c> {
//     type Target = Context<'i, 'c>;

//     fn deref(&self) -> &Self::Target {
//         &self.ctx
//     }
// }

pub trait RuleMatchingFn = Fn(MetaData, CowArcStr) -> Option<CssDecls>;

pub struct Rule<'i> {
    pub handler: Box<dyn RuleMatchingFn>,
    #[allow(dead_code)]
    pub supports_negative: bool,
    // a Theme map
    pub allowed_values: Option<ThemeValue<'i>>,
    #[allow(dead_code)]
    pub allowed_modifiers: Option<ThemeValue<'i>>,
    // a lightningcss PropertyId
    pub infer_property_id: Option<PropertyId<'i>>,
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

    pub fn infer_by(mut self, id: PropertyId<'c>) -> Self {
        self.infer_property_id = Some(id);
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

    pub fn apply_to<'a>(&self, value: &'a str) -> Option<CssDecls<'a>>
    where
        'c: 'a,
    {
        // arbitrary value
        // let ctx = ctx.clone();
        if let Some(stripped) = value.strip_arbitrary() {
            // TODO: add escape support
            // let stripped = &stripped.replace("_", " ");
            // when infer_property_id is None, default not check it
            match &self.infer_property_id {
                Some(id) => {
                    match Property::parse_string(
                        id.clone(),
                        stripped,
                        Default::default(),
                    ) {
                        Ok(Property::Unparsed(_)) => return None,
                        Err(_) => return None,
                        Ok(_) => {}
                    }
                }
                None => {}
            }

            return (self.handler)(
                MetaData {
                    raw: value.to_string(),
                },
                stripped.into(),
            );
        }

        // theme value
        if let Some(allowed_values) = &self.allowed_values {
            if let Some(v) = allowed_values.get(value) {
                return (self.handler)(
                    MetaData {
                        raw: value.to_string(),
                    },
                    v.clone(),
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
    use crate::decls;

    use super::*;

    #[test]
    fn test_rule_builder() {
        let rule = Rule::new(|_, _| None)
            .support_negative()
            .infer_by(PropertyId::FontSize);

        assert!(rule.supports_negative);
        assert_eq!(rule.infer_property_id, Some(PropertyId::FontSize));
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
