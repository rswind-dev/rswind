use std::ops::Not;

use regex::Regex;

use crate::{
    context::Context,
    css::{CSSDecl, CSSRule},
};

lazy_static! {
    static ref EXTRACT_RE: Regex = Regex::new(r#"[\\:]?[\s'"`;{}]+"#).unwrap();
}

fn to_css_rule<'a>(value: &'a str, ctx: &Context<'a>) -> Option<CSSRule> {
    // Step 1(todo): split the rules by `:`, get [...modifier, rule]
    // Step 2: try static match
    let mut decls: Vec<CSSDecl> = vec![];
    if let Some(static_rule) = ctx.static_rules.get(value) {
        decls = static_rule.to_vec();
    } else {
        // Step 3: get all index of `-`
        for (i, _) in value.match_indices("-") {
            let key = value.get(..i).unwrap();
            if let Some(func) = ctx.rules.get(key) {
                if let Some(v) = func(value.get((i + 1)..).unwrap()) {
                    decls.append(&mut v.to_vec());
                }
                break;
            }
        }
    }
    decls.is_empty().not().then(|| CSSRule {
        selector: format!("{}", value),
        nodes: decls,
    })
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
