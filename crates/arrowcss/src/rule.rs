use std::sync::{Arc, Weak};

use lightningcss::properties::{Property, PropertyId};

use crate::{
    context::Context, css::CSSDecls, theme::ThemeValue, utils::StripArbitrary,
};

pub trait RuleMatchingFn = Fn(Arc<Context>, &str) -> Option<CSSDecls> + 'static;

pub struct Rule<'i> {
    pub handler: Box<dyn RuleMatchingFn>,
    pub supports_negative: bool,
    // a Theme map
    pub allowed_values: Option<ThemeValue>,
    pub allowed_modifiers: Option<ThemeValue>,
    // a lightningcss PropertyId
    pub infer_property_id: Option<PropertyId<'i>>,
}

impl<'a> Rule<'a> {
    pub fn new<F: RuleMatchingFn>(handler: F) -> Self {
        Self {
            handler: Box::new(handler),
            supports_negative: false,
            allowed_values: None,
            allowed_modifiers: None,
            infer_property_id: None,
        }
    }

    pub fn infer_by(mut self, id: PropertyId<'a>) -> Self {
        self.infer_property_id = Some(id);
        self
    }

    pub fn support_negative(mut self) -> Self {
        self.supports_negative = true;
        self
    }

    pub fn allow_values(mut self, values: ThemeValue) -> Self {
        self.allowed_values = Some(values);
        self
    }

    pub fn allow_modifiers(mut self, modifiers: ThemeValue) -> Self {
        self.allowed_modifiers = Some(modifiers);
        self
    }

    pub fn apply_to(
        &self,
        ctx: Arc<Context<'a>>,
        value: &str,
    ) -> Option<CSSDecls> {
        // arbitrary value
        if let Some(stripped) = value.strip_arbitrary() {
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
                        Ok(_) => return (self.handler)(ctx.clone(), stripped),
                    }
                }
                None => return (self.handler)(ctx.clone(), stripped),
            }
        }

        // theme value
        if let Some(allowed_values) = &self.allowed_values {
            if let Some(v) = allowed_values.get(value) {
                return (self.handler)(ctx.clone(), v);
            }
        }

        None
    }

    pub fn bind_context(self, ctx: Arc<Context<'a>>) -> InContextRule<'a>
    {
        InContextRule {
            rule: self,
            ctx: Arc::downgrade(&ctx),
        }
    }
}

pub struct InContextRule<'a> {
    pub rule: Rule<'a>,
    pub ctx: Weak<Context<'a>>,
}

impl<'a> InContextRule<'a> {
    pub fn apply_to(&'a self, value: &str) -> Option<CSSDecls> {
        self.rule.apply_to(self.ctx.upgrade().unwrap(), value)
    }
}

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
        let rule = Rule::new(|_, value| {
            Some(decls! {
                "font-size" => &value,
            })
        })
        .support_negative()
        .infer_by(PropertyId::FontSize);

        let ctx = Arc::new(Context::default());

        assert_eq!(
            rule.apply_to(ctx.clone(), "[16px]"),
            Some(decls! {
                "font-size" => "16px",
            })
        );

        assert_eq!(
            rule.apply_to(ctx.clone(), "[larger]"),
            Some(decls! {
                "font-size" => "larger",
            })
        );

        assert_eq!(
            rule.apply_to(ctx.clone(), "[.5%]"),
            Some(decls! {
                "font-size" => ".5%",
            })
        );
    }

    #[test]
    fn test_handle_background_position() {
        let rule = Rule::new(|_, value| {
            Some(decls! {
                "background-position" => &value,
            })
        })
        .support_negative()
        .infer_by(PropertyId::BackgroundPosition);

        let ctx = Arc::new(Context::default());

        assert_eq!(
            rule.apply_to(ctx.clone(), "[top]"),
            Some(decls! {
                "background-position" => "top",
            })
        );

        assert_eq!(
            rule.apply_to(ctx.clone(), "[center]"),
            Some(decls! {
                "background-position" => "center",
            })
        );

        assert_eq!(
            rule.apply_to(ctx.clone(), "[50% 50%]"),
            Some(decls! {
                "background-position" => "50% 50%",
            })
        );

        assert_eq!(
            rule.apply_to(ctx.clone(), "[50% top]"),
            Some(decls! {
                "background-position" => "50% top",
            })
        );

        assert_eq!(rule.apply_to(ctx.clone(), "[top 50%]"), None);

        assert_eq!(
            rule.apply_to(ctx.clone(), "[left 50%]"),
            Some(decls! {
                "background-position" => "left 50%",
            })
        );

        assert_eq!(
            rule.apply_to(ctx.clone(), "[bottom 10px right 20px]"),
            Some(decls! {
                "background-position" => "bottom 10px right 20px",
            })
        );
    }
}
