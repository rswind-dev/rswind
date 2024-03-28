use std::{
    iter,
    ops::{ControlFlow, Deref},
    sync::Arc,
};

use crate::{
    context::VariantMatchingFn,
    css::{AtRule, CssRule, CssRuleList},
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

pub enum VariantHandler {
    Nested(Box<dyn VariantMatchingFn>),
    Replacement(Box<dyn VariantMatchingFn>),
}

impl<'a, 'b> Fn<(CssRuleList<'a>,)> for VariantHandler {
    extern "rust-call" fn call(
        &self,
        args: (CssRuleList<'a>,),
    ) -> Option<CssRuleList<'a>> {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Replacement(f) => f(args.0),
        }
    }
}

impl<'a, 'b> FnOnce<(CssRuleList<'a>,)> for VariantHandler {
    type Output = Option<CssRuleList<'a>>;

    extern "rust-call" fn call_once(
        self,
        args: (CssRuleList<'a>,),
    ) -> Self::Output {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Replacement(f) => f(args.0),
        }
    }
}

impl<'a, 'b> FnMut<(CssRuleList<'a>,)> for VariantHandler {
    extern "rust-call" fn call_mut(
        &mut self,
        args: (CssRuleList<'a>,),
    ) -> Option<CssRuleList<'a>> {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Replacement(f) => f(args.0),
        }
    }
}

fn create_nested_variant_fn<'a>(matcher: String) -> VariantHandler {
    VariantHandler::Nested(Box::new(move |rule| {
        Some(
            CssRule::AtRule(AtRule {
                name: matcher.to_owned(),
                params: "".into(),
                nodes: vec![rule],
            })
            .into(),
        )
    }))
}

fn create_replacement_variant_fn<'a>(matcher: String) -> VariantHandler {
    VariantHandler::Replacement(Box::new(move |mut container: CssRuleList| {
        for rule in container.nodes.iter_mut() {
            match rule {
                CssRule::Style(ref mut it) => {
                    it.selector += matcher.as_str();
                }
                _ => {}
            }
        }
        Some(container)
    }))
}

pub fn create_variant_fn<'a, M: Matcher<'a>>(
    _key: &str,
    matcher: M,
) -> Option<VariantHandler> {
    let fns = matcher
        .into_matcher()
        .into_iter()
        .map(|matcher| {
            matcher
                .split('|')
                .map(|matcher| matcher.trim())
                .try_fold(None, |acc: Option<VariantHandler>, item| {
                    let new_fn: VariantHandler = match item.chars().next() {
                        Some('@') => create_nested_variant_fn(
                            item.get(1..).unwrap().to_string(),
                        ),
                        Some('&') => create_replacement_variant_fn(
                            item.get(1..).unwrap().to_string(),
                        ),
                        _ => return ControlFlow::Break(()),
                    };
                    ControlFlow::Continue(match acc {
                        Some(VariantHandler::Nested(acc)) => {
                            Some(VariantHandler::Nested(Box::new(
                                move |container: CssRuleList| {
                                    acc(container.clone())
                                        .and_then(|container| new_fn(container))
                                },
                            )))
                        }
                        Some(VariantHandler::Replacement(acc)) => {
                            Some(VariantHandler::Replacement(Box::new(
                                move |container: CssRuleList| {
                                    acc(container.clone())
                                        .and_then(|container| new_fn(container))
                                },
                            )))
                        }
                        None => Some(new_fn),
                    })
                })
                .continue_value()
                .flatten()
        })
        .collect::<Option<Vec<_>>>()?;

    // sort fns by VariantHandler type
    let (nested_fns, replace_fns): (Vec<_>, Vec<_>) = fns
        .into_iter()
        .partition(|f| matches!(f, VariantHandler::Nested(_)));
    let is_nested = !nested_fns.is_empty();
    let fns = Arc::new(
        replace_fns
            .into_iter()
            .chain(nested_fns)
            .collect::<Vec<_>>(),
    );

    let handler: Box<dyn Fn(CssRuleList) -> Option<CssRuleList>> =
        Box::new(move |mut container: CssRuleList| {
            container.nodes = fns
                .deref()
                .into_iter()
                .filter_map(|f| f(container.clone()))
                .collect::<CssRuleList>()
                .nodes;
            Some(container)
        });

    // Some(VariantHandler::Nested(Box::new(|_| None)))
    Some(if is_nested {
        VariantHandler::Nested(handler)
    } else {
        VariantHandler::Replacement(handler)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_variant() {
        // let variant: VariantHandler =
        //     create_variant_fn("disabled", "&:disabled").unwrap();
        // let rule = CssRule::Style(StyleRule {
        //     selector: "flex".into(),
        //     nodes: vec![CssRule::Decl(("display", "flex").into())],
        // })
        // .into();
        // let new_rule = variant(rule).unwrap();

        // println!("{:?}", new_rule);
    }
}
