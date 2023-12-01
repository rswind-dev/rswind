use crate::{
    context::{Context, Variant},
    css::{CSSRule, CSSStyleRule},
};
use lazy_static::lazy_static;
use regex::Regex;

pub trait Parse<T> {
    fn parse<'a>(ctx: &'a Context, input: T) -> Option<Self>
    where
        Self: Sized;
}

lazy_static! {
    static ref EXTRACT_RE: Regex = Regex::new(r#"[\\:]?[\s'"`;{}]+"#).unwrap();
}

fn to_css_rule<'a>(value: &'a str, ctx: &Context<'a>) -> Option<CSSRule> {
    let (modifiers, rule) = extract_variants(value);
    // Step 2: try static match
    let mut decls: Vec<CSSRule> = vec![];
    if let Some(static_rule) = ctx.static_rules.get(&rule) {
        decls = static_rule
            .to_vec()
            .into_iter()
            .map(CSSRule::Decl)
            .collect();
    } else {
        // Step 3: get all index of `-`
        for (i, _) in rule.match_indices('-') {
            let key = rule.get(..i).unwrap();
            if let Some(func) = ctx.rules.get(key) {
                if let Some(v) = func(rule.get((i + 1)..).unwrap().to_string())
                {
                    decls.append(
                        &mut v
                            .to_vec()
                            .into_iter()
                            .map(CSSRule::Decl)
                            .collect(),
                    );
                }
                break;
            }
        }
    }

    if decls.is_empty() {
        return None;
    }

    let mut rule = CSSRule::Style(CSSStyleRule {
        selector: rule.to_string(),
        nodes: decls,
    });

    // Step 4: apply modifiers
    let (at_rules_variants, plain_variants): (Vec<_>, Vec<_>) = modifiers
        .iter()
        .filter_map(|modifier| ctx.variants.get(modifier))
        .partition(|variant| variant.needs_nesting);

    for variant in plain_variants.iter().chain(at_rules_variants.iter()) {
        let new_rule = (variant.handler)(rule)?;
        rule = new_rule;
    }

    Some(rule)
}

pub fn extract_variants(value: &str) -> (Vec<String>, String) {
    // Step 1(todo): split the rules by `:`, get [...modifier, rule]
    let mut modifiers =
        value.split(':').map(String::from).collect::<Vec<String>>();

    let value = modifiers.pop().unwrap();

    (modifiers, value)
}

pub fn parse<'b>(input: &'b str, ctx: &mut Context<'b>) {
    let parts = EXTRACT_RE.split(input);
    for token in parts.into_iter() {
        if token.is_empty() {
            continue;
        }
        if ctx.tokens.contains_key(token) {
            continue;
        }
        ctx.tokens.insert(token, to_css_rule(token, ctx));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_modifiers() {
        assert_eq!(
            extract_variants("md:opacity-50"),
            (vec!["md".into()], "opacity-50".into())
        );
        assert_eq!(
            extract_variants("opacity-50"),
            (vec![], "opacity-50".into())
        );
        assert_eq!(
            extract_variants("md:disabled:hover:opacity-50"),
            (
                vec!["md".into(), "disabled".into(), "hover".into()],
                "opacity-50".into()
            )
        );
        assert_eq!(extract_variants(""), (vec![], "".into()));
    }
}
