use crate::{
    context::Context,
    parser::{extract_variants, Parse},
    utility::Utility,
    variant::Variant,
};

struct Rule {
    variants: Vec<Variant>,
    utility: Utility,
}

impl Parse<&str> for Rule {
    fn parse(ctx: &Context, input: &str) -> Option<Self> {
        let (variants, utility) = extract_variants(input);

        let utility = Utility::parse(ctx, &utility)?;
        let variants = variants
            .iter()
            .map(|v| Variant::parse(ctx, v))
            .collect::<Option<Vec<_>>>()?;

        Some(Self {
            variants,
            utility,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::css::{CSSRule, CSSDecls};

    use super::*;

    #[test]
    fn test_rule() {
        let mut ctx = Context::default();

        ctx.add_static(("flex", CSSDecls::one("display", "flex")));

        ctx.add_variant("disabled", |a| {
            match a {
                CSSRule::Style(mut it) => {
                    it.selector += ":disabled";
                    Some(CSSRule::Style(it))
                }
                _ => None,
            }
        });
        let rule = Rule::parse(&ctx, "[@media(min-width200px)]:flex").unwrap();
        assert_eq!(
            rule.utility,
            Utility::lit("flex".into(), false, false, CSSDecls::one("display", "flex"))
        );
    }
}