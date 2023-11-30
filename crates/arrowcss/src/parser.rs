use std::ops::Not;
use lazy_static::lazy_static;
use regex::Regex;
use crate::{
    context::Context,
    css::{CSSStyleRule, CSSRule},
};

lazy_static! {
    static ref EXTRACT_RE: Regex = Regex::new(r#"[\\:]?[\s'"`;{}]+"#).unwrap();
}

fn to_css_rule<'a>(value: &'a str, ctx: &Context<'a>) -> Option<CSSRule> {
    let (modifiers, rule) = extract_modifiers(value);
    // Step 2: try static match
    let mut decls: Vec<CSSRule> = vec![];
    if let Some(static_rule) = ctx.static_rules.get(&rule) {
        decls = static_rule.to_vec().into_iter().map(|it| CSSRule::Decl(it)).collect();
    } else {
        // Step 3: get all index of `-`
        for (i, _) in rule.match_indices("-") {
            let key = rule.get(..i).unwrap();
            if let Some(func) = ctx.rules.get(key) {
                if let Some(v) = func(rule.get((i + 1)..).unwrap().to_string()) {
                    decls.append(&mut v.to_vec().into_iter().map(|it| CSSRule::Decl(it)).collect());
                }
                break;
            }
        }
    }

    if decls.is_empty() {
        return None
    }

    let mut rule = CSSRule::Style(CSSStyleRule {
        selector: format!("{}", rule),
        nodes: decls,
    });

    // Step 4: apply modifiers
    for modifier in modifiers {
        if let Some(variant_fn) = ctx.variants.get(&modifier) {
            if let Some(new_rule) = variant_fn(rule) {
                rule = new_rule
            } else {
                return None
            }
        } else {
            return None
        }
    }

    Some(rule)
    // decls.is_empty().not().then(|| CSSStyleRule {
    //     selector: format!("{}", rule),
    //     nodes: decls,
    // })
}

pub fn extract_modifiers(value: &str) -> (Vec<String>, String) {
    // Step 1(todo): split the rules by `:`, get [...modifier, rule]
    let mut modifiers = value.split(":")
        .map(String::from)
        .collect::<Vec<String>>();

    let value = modifiers.pop().unwrap();

    (modifiers, value)
}

pub fn parse<'a, 'b>(input: &'b str, ctx: &'a mut Context<'b>) {
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
            extract_modifiers("md:opacity-50"),
            (vec!["md".into()], "opacity-50".into())
        );
        assert_eq!(
            extract_modifiers("opacity-50"),
            (vec![], "opacity-50".into())
        );
        assert_eq!(
            extract_modifiers("md:disabled:hover:opacity-50"),
            (vec!["md".into(), "disabled".into(), "hover".into()], "opacity-50".into())
        );
        assert_eq!(
            extract_modifiers(""),
            (vec![], "".into())
        );
    }
}