use std::ops::ControlFlow;

use crate::{
    context::VariantMatchingFn,
    css::{CSSAtRule, CSSRule},
};

pub fn strip_arbitrary(value: &str) -> Option<&str> {
    value.strip_prefix('[').and_then(|r| r.strip_suffix(']'))
}

pub trait StripArbitrary {
    fn strip_arbitrary(&self) -> Option<&str>;
}

impl StripArbitrary for str {
    fn strip_arbitrary(&self) -> Option<&str> {
        strip_arbitrary(self)
    }
}

fn create_nested_variant_fn(matcher: String) -> Box<dyn VariantMatchingFn> {
    Box::new(move |rule| {
        Some(CSSRule::AtRule(CSSAtRule {
            name: matcher.to_owned(),
            params: "".into(),
            nodes: vec![rule],
        }))
    })
}

fn create_replacement_variant_fn(
    matcher: String,
) -> Box<dyn VariantMatchingFn> {
    Box::new(move |rule| match rule {
        CSSRule::Style(mut it) => {
            it.selector += matcher.as_str();
            Some(CSSRule::Style(it))
        }
        _ => None,
    })
}

pub fn create_variant_fn(
    key: &str,
    matcher: &str,
) -> Option<Box<dyn VariantMatchingFn>> {
    matcher
        .split('|')
        .map(|matcher| matcher.trim())
        .try_fold(None, |acc: Option<Box<dyn VariantMatchingFn>>, item| {
            let new_fn: Box<dyn VariantMatchingFn> = match item.chars().next() {
                Some('@') => create_nested_variant_fn(
                    item.get(1..).unwrap().to_string(),
                ),
                Some('&') => create_replacement_variant_fn(
                    item.get(1..).unwrap().to_string(),
                ),
                _ => return ControlFlow::Break(()),
            };
            ControlFlow::Continue(match acc {
                Some(acc) => Some(Box::new(move |rule| {
                    new_fn(rule).and_then(|rule| acc(rule))
                })),
                None => Some(new_fn),
            })
        })
        .continue_value()
        .flatten()
}

#[cfg(test)]
mod tests {
    use crate::css::CSSStyleRule;

    use super::*;

    #[test]
    fn test_add_variant() {
        let variant: Box<dyn VariantMatchingFn> =
            create_variant_fn("disabled", "&:disabled").unwrap();
        let rule = CSSRule::Style(CSSStyleRule {
            selector: "flex".into(),
            nodes: vec![CSSRule::Decl(("display", "flex").into())],
        });
        let new_rule = variant(rule).unwrap();

        println!("{:?}", new_rule);
    }
}
