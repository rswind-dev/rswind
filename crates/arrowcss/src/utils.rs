use std::{iter, ops::ControlFlow, rc::Rc};

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

#[derive(Clone)]
pub enum VariantHandler {
    Nested(Rc<dyn VariantMatchingFn>),
    Replacement(Rc<dyn VariantMatchingFn>),
}

impl Fn<(CssRuleList,)> for VariantHandler {
    extern "rust-call" fn call(
        &self,
        args: (CssRuleList,),
    ) -> Option<CssRuleList> {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Replacement(f) => f(args.0),
        }
    }
}

impl FnOnce<(CssRuleList,)> for VariantHandler {
    type Output = Option<CssRuleList>;

    extern "rust-call" fn call_once(
        self,
        args: (CssRuleList,),
    ) -> Self::Output {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Replacement(f) => f(args.0),
        }
    }
}

impl FnMut<(CssRuleList,)> for VariantHandler {
    extern "rust-call" fn call_mut(
        &mut self,
        args: (CssRuleList,),
    ) -> Option<CssRuleList> {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Replacement(f) => f(args.0),
        }
    }
}

fn create_nested_variant_fn(matcher: String) -> VariantHandler {
    VariantHandler::Nested(Rc::new(move |rule| {
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

fn create_replacement_variant_fn(matcher: String) -> VariantHandler {
    VariantHandler::Replacement(Rc::new(move |mut container: CssRuleList| {
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
    key: &str,
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
                            Some(VariantHandler::Nested(Rc::new(
                                move |container: CssRuleList| {
                                    acc(container.clone())
                                        .and_then(|container| new_fn(container))
                                },
                            )))
                        }
                        Some(VariantHandler::Replacement(acc)) => {
                            Some(VariantHandler::Replacement(Rc::new(
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
    let fns = replace_fns.into_iter().chain(nested_fns);

    let handler = Rc::new(move |mut container: CssRuleList| {
        container = fns
            .clone()
            .into_iter()
            .filter_map(|f| f(container.clone()))
            .collect::<CssRuleList>();
        Some(container)
    });

    Some(if is_nested {
        VariantHandler::Nested(handler)
    } else {
        VariantHandler::Replacement(handler)
    })
}

#[cfg(test)]
mod tests {
    use smallvec::SmallVec;

    use crate::css::StyleRule;

    use super::*;

    #[test]
    fn test_add_variant() {
        let variant: VariantHandler =
            create_variant_fn("disabled", "&:disabled").unwrap();
        let rule = CssRule::Style(StyleRule {
            selector: "flex".into(),
            nodes: vec![CssRule::Decl(("display", "flex").into())],
        })
        .into();
        let new_rule = variant(rule).unwrap();

        println!("{:?}", new_rule);
    }
}
