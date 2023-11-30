// allow unused_vars
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fmt::Write;

use cssparser::{Parser, ParserInput};

use crate::css::CSSDecl;
use crate::utils::StripArbitrary;
use crate::{context::Context, css::CSSDecls};

#[derive(Debug)]
enum Utility {
    Literal(LiteralUtility),
    Arbitrary(ArbitraryUtility),
}

// static rule / arbitrary declaration
// E.g. `[text:red]` or `flex`(defined in config.theme)
#[derive(Debug)]
struct LiteralUtility {
    pub raw: String,
    pub important: bool,
    pub negative: bool,
    pub value: CSSDecls,
}

// dynamic rule
// E.g. `text-[#123]` or `!-text-[12px]`
#[derive(Debug)]
struct ArbitraryUtility {
    pub raw: String,
    pub value: String,
    pub important: bool,
    pub negative: bool,
    pub modifier: Option<String>,
}

impl Utility {
    pub fn lit(raw: String, important: bool, negative: bool, value: CSSDecls) -> Self {
        Self::Literal(LiteralUtility {
            raw,
            important,
            negative,
            value,
        })
    }

    pub fn arbitrary(
        raw: String,
        value: String,
        important: bool,
        negative: bool,
        modifier: Option<String>,
    ) -> Self {
        Self::Arbitrary(ArbitraryUtility {
            raw,
            value,
            important,
            negative,
            modifier,
        })
    }

    pub fn parse(ctx: &Context, value: &str) -> Option<Self> {
        let mut unprefixed = value;
        let mut important = false;

        if let Some(un) = value.strip_prefix('!') {
            unprefixed = un;
            important = true;
        }

        // Step 2: try arbitrary decl match (e.g. `[color:red]`)
        if let Some((k, v)) =
            unprefixed.strip_arbitrary().and_then(|r| r.split_once(':'))
        {
            return Some(Utility::lit(
                value.to_string(),
                important,
                false,
                CSSDecls::one(k, v),
            ));
        }

        // Step 3: try static match (e.g. `flex`)
        if let Some(decl) = ctx.static_rules.get(unprefixed) {
            return Some(Utility::lit(
                value.to_string(),
                important,
                false,
                decl.clone(),
            ));
        }

        // Step 4: try arbitrary rule match (e.g. `text-[#123]`)
        let mut parts = unprefixed.split('-').rev();

        let maybe_arbitrary = parts.next();

        if let Some(arbitrary) =
            maybe_arbitrary.and_then(StripArbitrary::strip_arbitrary)
        {
            return Some(Utility::arbitrary(
                value.to_string(),
                arbitrary.to_string(),
                important,
                false,
                None,
            ));
        } else if let Some(rule) = maybe_arbitrary {
            let mut negative = false;
            if let Some(un) = value.strip_prefix('-') {
                unprefixed = un;
                negative = true;
            }
            for (i, _) in unprefixed.match_indices('-') {
                let key = unprefixed.get(..i).unwrap();
                let func = ctx.rules.get(key)?;
                let v = func(unprefixed.get((i + 1)..).unwrap().to_string())?;
                return Some(Utility::lit(value.into(), important, negative, v));
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{static_rules, theme::Theme};

    use super::*;

    #[test]
    fn test_utility() {
        let ctx = Context::new(Theme::default().into());

        let utility = Utility::parse(&ctx, "![color:red]").unwrap();

        if let Utility::Literal(u) = utility {
            assert_eq!(u.raw, "![color:red]");
            assert_eq!(u.value, CSSDecls::one("color", "red"));
            assert!(u.important);
        }
    }

    #[test]
    fn test_utility_parse() {
        let mut ctx = Context::new(Theme::default().into());

        ctx.add_static(
            static_rules! {
                "flex" => { "display": "flex"; }
            }
            .get(0)
            .unwrap()
            .to_owned(),
        );

        let utility = Utility::parse(&ctx, "flex").unwrap();

        if let Utility::Literal(u) = utility {
            assert_eq!(u.raw, "flex");
            assert_eq!(u.value, CSSDecls::multi([("display", "flex")]));
            assert!(!u.important);
        }
    }

    #[test]
    fn test_utility_parse_arbitrary() {
        let ctx = Context::new(Theme::default().into());
        let utility = Utility::parse(&ctx, "text-[#123456]").unwrap();

        if let Utility::Arbitrary(u) = utility {
            assert_eq!(u.raw, "text-[#123456]");
            assert_eq!(u.value, "#123456");
            assert!(!u.important);
            assert!(!u.negative);
            assert!(u.modifier.is_none());
        } else {
            panic!("Expected Utility::Literal, found a different variant");
        }
    }

    #[test]
    fn test_utility_parse_theme() {
        let mut theme = Theme::default();
        theme.colors.insert("blue-500".into(), "#123456".into());
        let mut ctx = Context::new(theme.into());

        ctx.add_rule("text", |a, b| {
            Some(CSSDecls::one("color", b.colors.get(&a)?))
        });

        let utility = Utility::parse(&ctx, "text-blue-500").unwrap();

        if let Utility::Literal(u) = utility {
            assert_eq!(u.raw, "text-blue-500");
            assert_eq!(u.value, CSSDecls::one("color", "#123456"));
            assert!(!u.important);
        } else {
            panic!("Expected Utility::Literal, found a different variant");
        }
    }
}
