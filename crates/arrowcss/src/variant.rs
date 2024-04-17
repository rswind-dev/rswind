use crate::{common::MaybeArbitrary, context::Context};

pub fn create_variants(ctx: &mut Context) {
    ctx
        // Positional
        .add_variant("first", ["&:first-child"])
        .add_variant("last", ["&:last-child"])
        .add_variant("only", ["&:only-child"])
        .add_variant("odd", ["&:nth-child(odd)"])
        .add_variant("even", ["&:nth-child(even)"])
        .add_variant("first-of-type", ["&:first-of-type"])
        .add_variant("last-of-type", ["&:last-of-type"])
        .add_variant("only-of-type", ["&:only-of-type"])
        // State
        .add_variant("visited", ["&:visited"])
        .add_variant("target", ["&:target"])
        .add_variant("open", ["&:is([open], :popover-open)"])
        // Forms
        .add_variant("default", ["&:default"])
        .add_variant("checked", ["&:checked"])
        .add_variant("indeterminate", ["&:indeterminate"])
        .add_variant("placeholder-shown", ["&:placeholder-shown"])
        .add_variant("autofill", ["&:autofill"])
        .add_variant("optional", ["&:optional"])
        .add_variant("required", ["&:required"])
        .add_variant("valid", ["&:valid"])
        .add_variant("invalid", ["&:invalid"])
        .add_variant("in-range", ["&:in-range"])
        .add_variant("out-of-range", ["&:out-of-range"])
        .add_variant("read-only", ["&:read-only"])
        // Content
        .add_variant("empty", ["&:empty"])
        // Interactive
        .add_variant("focus-within", ["&:focus-within"])
        .add_variant("hover", ["&:hover"])
        .add_variant("focus", ["&:focus"])
        .add_variant("focus-visible", ["&:focus-visible"])
        .add_variant("active", ["&:active"])
        .add_variant("enabled", ["&:enabled"])
        .add_variant("disabled", ["&:disabled"])
        .add_variant("marker", ["& *::marker", "&::marker"])
        .add_variant("*", ["& > *"])
        // Accessibility
        .add_variant(
            "motion-safe",
            ["@media (prefers-reduced-motion: no-preference)"],
        )
        .add_variant(
            "motion-reduce",
            ["@media (prefers-reduced-motion: reduce)"],
        )
        .add_variant("contrast-more", ["@media (prefers-contrast: more)"])
        .add_variant("contrast-less", ["@media (prefers-contrast: less)"])
        // Others
        .add_variant("portrait", ["@media (orientation: portrait)"])
        .add_variant("landscape", ["@media (orientation: landscape)"])
        .add_variant("ltr", ["&:where([dir=\"ltr\"], [dir=\"ltr\"] *)"])
        .add_variant("rtl", ["&:where([dir=\"rtl\"], [dir=\"rtl\"] *)"])
        .add_variant("dark", ["@media (prefers-color-scheme: dark)"])
        .add_variant("starting", ["@starting-style"])
        .add_variant("print", ["@media print"])
        .add_variant("forced-colors", ["@media (forced-colors: active)"]);

    ctx.add_variant_fn("aria", |rule, candidate| match candidate.value {
        Some(MaybeArbitrary::Arbitrary(value)) => {
            rule.modify_with(|s| format!("{}[aria-{}]", s, value))
        }
        Some(MaybeArbitrary::Named(value)) => {
            rule.modify_with(|s| format!("{}[aria-{}=\"true\"]", s, value))
        }
        None => rule,
    });

    ctx.add_variant_fn("data", |rule, candidate| {
        rule.modify_with(|s| {
            format!("{}[data-{}]", s, take_or_default(&candidate.value))
        })
    });

    ctx.add_variant_composable("has", |rule, _| {
        rule.modify_with(|s| format!("&:has({})", s.replace('&', "*")))
    });

    ctx.add_variant_composable("not", |rule, _| {
        rule.modify_with(|s| format!("&:not({})", s.replace('&', "*")))
    });

    ctx.add_variant_composable("group", |rule, candidate| {
        let group_name = take_or_default(&candidate.modifier);
        let selector = format!(
            ":where(.group{}{})",
            if group_name.is_empty() { "" } else { r"\/" },
            group_name
        );

        rule.modify_with(|s| format!("&:is({} *)", s.replace('&', &selector)))
    });

    ctx.add_variant_composable("peer", |rule, candidate| {
        let group_name = take_or_default(&candidate.modifier);
        let selector = format!(
            ":where(.peer{}{})",
            if group_name.is_empty() { "" } else { r"\/" },
            group_name
        );

        rule.modify_with(|s| format!("&:is({} ~ *)", s.replace('&', &selector)))
    });

    ctx.get_theme("breakpoints").map(|theme| {
        theme.iter().for_each(|(k, v)| {
            ctx.add_variant(k, [format!("@media (width >= {})", v)]);
        })
    });
}

fn take_or_default<'a, 'b>(value: &'b Option<MaybeArbitrary<'a>>) -> &'b str {
    value.as_ref().map(|m| m.as_str()).unwrap_or("")
}

#[cfg(test)]
mod tests {
    use arrowcss_css_macro::css;

    use super::*;
    use crate::{context::Context, css::ToCssString, parsing::VariantParser};

    #[test]
    fn test_load_variants() {
        let mut ctx = Context::default();
        create_variants(&mut ctx);

        let rule = css!("display": "flex").to_rule_list();

        let candidate =
            VariantParser::new("group-hover/aaa").parse(&ctx).unwrap();

        let res = candidate.handle(rule);

        println!("{}", res.to_css_string().unwrap());
    }
}
