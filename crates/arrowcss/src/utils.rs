use std::{iter, ops::ControlFlow, rc::Rc};

use crate::{
    context::VariantMatchingFn,
    css::{Container, CSSAtRule, CSSRule},
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

pub trait Matcher<'a> {
    fn into_matcher(self) -> impl IntoIterator<Item = &'a str>;
}

impl<'a> Matcher<'a> for &'a str {
    fn into_matcher(self) -> impl IntoIterator<Item = &'a str> {
        iter::once(self)
    }
}

impl<'a> Matcher<'a> for &'a [&'a str] {
    fn into_matcher(self) -> impl IntoIterator<Item = &'a str> {
        self.iter().copied()
    }
}

// impl for ["& *::marker", "&::marker"])
// impl<'a, const N: usize> Matcher<'a> for [&'a str; N] {
//     fn into_matcher(self) -> impl IntoIterator<Item = &'a str> {
//         self.iter().copied()
//     }
// }

impl<'a> Matcher<'a> for Vec<&'a str> {
    fn into_matcher(self) -> impl IntoIterator<Item = &'a str> {
        self.into_iter()
    }
}

fn create_nested_variant_fn(matcher: String) -> Rc<dyn VariantMatchingFn> {
    Rc::new(move |rule| {
        Some(
            CSSRule::AtRule(CSSAtRule {
                name: matcher.to_owned(),
                params: "".into(),
                nodes: vec![rule],
            })
            .into(),
        )
    })
}

fn create_replacement_variant_fn(matcher: String) -> Rc<dyn VariantMatchingFn> {
    Rc::new(move |mut container: Container| {
        for rule in container.nodes.iter_mut() {
            match rule {
                CSSRule::Style(ref mut it) => {
                    it.selector += matcher.as_str();
                }
                _ => {}
            }
        }
        Some(container)
    })
}

pub fn create_variant_fn<'a, M: Matcher<'a>>(
    key: &str,
    matcher: M,
) -> Option<Rc<dyn VariantMatchingFn>> {
    let fns = matcher
        .into_matcher()
        .into_iter()
        .map(|matcher| {
            matcher
                .split('|')
                .map(|matcher| matcher.trim())
                .try_fold(
                    None,
                    |acc: Option<Rc<dyn VariantMatchingFn>>, item| {
                        let new_fn: Rc<dyn VariantMatchingFn> =
                            match item.chars().next() {
                                Some('@') => create_nested_variant_fn(
                                    item.get(1..).unwrap().to_string(),
                                ),
                                Some('&') => create_replacement_variant_fn(
                                    item.get(1..).unwrap().to_string(),
                                ),
                                _ => return ControlFlow::Break(()),
                            };
                        ControlFlow::Continue(match acc {
                            Some(acc) => Some(Rc::new(move |rule| {
                                new_fn(rule).and_then(|rule| acc(rule))
                            })
                                as Rc<dyn VariantMatchingFn>),
                            None => Some(new_fn),
                        })
                    },
                )
                .continue_value()
                .flatten()
        })
        .collect::<Vec<_>>();
    Some(Rc::new(move |mut container: Container| {
        container = fns
            .clone()
            .into_iter()
            .filter_map(|f| (f.unwrap())(container.clone()))
            .collect::<Container>();
        Some(container)
    }) as Rc<dyn VariantMatchingFn>)
}

#[cfg(test)]
mod tests {
    use crate::css::CSSStyleRule;

    use super::*;

    #[test]
    fn test_add_variant() {
        let variant: Rc<dyn VariantMatchingFn> =
            create_variant_fn("disabled", "&:disabled").unwrap();
        let rule = CSSRule::Style(CSSStyleRule {
            selector: "flex".into(),
            nodes: vec![CSSRule::Decl(("display", "flex").into())],
        })
        .into();
        let new_rule = variant(rule).unwrap();

        println!("{:?}", new_rule);
    }
}
