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

fn create_nested_variant_fn(matcher: String) -> impl VariantMatchingFn {
    move |rule| {
        Some(CSSRule::AtRule(CSSAtRule {
            name: matcher.to_owned(),
            params: "".into(),
            nodes: vec![rule],
        }))
    }
}
// 11520 * 1.25 * 1.15
fn create_replacement_variant_fn(matcher: String) -> impl VariantMatchingFn {
    move |rule| match rule {
        CSSRule::Style(mut it) => {
            it.selector += matcher.as_str();
            Some(CSSRule::Style(it))
        }
        _ => None,
    }
}

fn add_variant(key: &str, matcher: &str) -> Option<Box<dyn VariantMatchingFn>> {
    // match first char of matcher.
    match matcher.chars().next()? {
        '@' => Some(Box::new(create_nested_variant_fn(
            matcher.get(1..)?.to_string(),
        ))),
        '&' => Some(Box::new(create_replacement_variant_fn(
            matcher.get(1..)?.to_string(),
        ))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::css::CSSStyleRule;

    use super::*;

    #[test]
    fn test_add_variant() {
        let variant: Box<dyn VariantMatchingFn> = add_variant("disabled", "&:disabled").unwrap();
        let rule = CSSRule::Style(CSSStyleRule {
            selector: "flex".into(),
            nodes: vec![
                CSSRule::Decl(("display", "flex").into()),
            ],
        });
        let new_rule = variant(rule).unwrap();

        println!("{:?}", new_rule);
    }
}
