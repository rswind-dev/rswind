use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Weak},
};

use cssparser::{Parser, ParserInput, Token};

use crate::{
    context::Context, css::CSSDecls, theme::ThemeValue, utils::StripArbitrary,
};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum DataType {
    Length,
    Percentage,
    LengthPercentage,
    Color,
    Any,
}

pub trait RuleMatchingFn = Fn(Arc<Context>, &str) -> Option<CSSDecls> + 'static;

pub struct Rule {
    pub handler: Box<dyn RuleMatchingFn>,
    pub supports_negative: bool,
    pub allowed_types: HashSet<DataType>,
    pub allowed_values: Option<ThemeValue>,
    pub allowed_modifiers: Option<ThemeValue>,
}

impl Rule {
    pub fn new<F: RuleMatchingFn>(handler: F) -> Self {
        Self {
            handler: Box::new(handler),
            supports_negative: false,
            allowed_types: HashSet::new(),
            allowed_values: None,
            allowed_modifiers: None,
        }
    }

    pub fn support_negative(mut self) -> Self {
        self.supports_negative = true;
        self
    }

    pub fn allow_type(mut self, ty: DataType) -> Self {
        self.allowed_types.insert(ty);
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

    pub fn apply_to(&self, ctx: Arc<Context>, value: &str) -> Option<CSSDecls> {
        if let Some(stripped) = value.strip_arbitrary() {
            if self.allowed_types.is_empty() {
                return None;
            }
            let mut input = ParserInput::new(stripped);
            let mut parser = Parser::new(&mut input);

            let mut typ = DataType::Any;
            match parser.next() {
                Ok(Token::Percentage { .. }) => {
                    typ = DataType::Percentage;
                }
                Ok(Token::Dimension { .. }) => {
                    typ = DataType::Length;
                }
                _ => {}
            }

            if !self.allowed_types.contains(&typ) {
                return None;
            }
            return (self.handler)(ctx, stripped);
        }

        let a = self.allowed_values.as_ref()?.clone();

        if let Some(v) = a.get(value) {
            return (self.handler)(ctx, v);
        }

        None
    }

    pub fn bind_context(self, ctx: Arc<Context>) -> InContextRule {
        InContextRule {
            rule: self,
            ctx: Arc::downgrade(&ctx),
        }
    }
}

pub struct InContextRule {
    pub rule: Rule,
    pub ctx: Weak<Context>,
}

impl InContextRule {
    pub fn apply_to(&self, value: &str) -> Option<CSSDecls> {
        (self.rule.handler)(self.ctx.upgrade().unwrap(), value)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{decls, themes::theme};

    use super::*;

    #[test]
    fn test_rule_builder() {
        let rule = Rule::new(|_, _| None)
            .support_negative()
            .allow_type(DataType::Length);

        assert!(rule.supports_negative);
        assert!(rule.allowed_types.contains(&DataType::LengthPercentage));
    }

    #[test]
    fn test_rule_apply() {
        let mut ctx = Context::default();
        let themes = theme();
        ctx.theme = Rc::new(themes).into();

        let rule = Rule::new(|_, v| Some(CSSDecls::from_pair(("width", v))))
            .support_negative()
            .allow_values(ctx.get_theme("spacing").unwrap())
            .allow_type(DataType::Length);
        let ctx: Arc<Context> = Context::default().into();

        assert_eq!(rule.apply_to(ctx.clone(), "[10px]").unwrap()[0].value, "10px");
        let a = rule.apply_to(ctx.clone(), "4").unwrap();
        assert_eq!(a[0].value, "1rem")
    }

    #[test]
    fn test_bind_context() {
        let mut ctx = Context::default();
        let themes = theme();
        ctx.theme = Rc::new(themes).into();
        let ctx: Arc<Context> = Context::default().into();

        let rule = Rule::new(|_ctx, value| {
            Some(decls! {
                    "--tw-blur" => &format!("blur({})", value),
            })
        })
        .support_negative()
        .allow_values(ctx.get_theme("blur").unwrap())
        .allow_type(DataType::LengthPercentage);

        let rule = rule.bind_context(ctx.clone());

        assert!(rule.apply_to("10px").is_some());
    }

    #[test]
    fn test_aaa() {
        let mut input = ParserInput::new("center top 1rem");
        let mut parser = Parser::new(&mut input);

        println!("{:?}", parser.next());
    }
}
