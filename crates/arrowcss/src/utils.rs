use std::{
    cmp::Ordering,
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

pub enum VariantHandler {
    Nested(Box<dyn VariantMatchingFn>),
    Replacement(Box<dyn VariantMatchingFn>),
}

impl VariantHandler {
    pub fn as_handler(self) -> Box<dyn VariantMatchingFn> {
        match self {
            Self::Nested(f) => f,
            Self::Replacement(f) => f,
        }
    }

    pub fn create_constructor(&self) -> impl Fn(Box<dyn VariantMatchingFn>) -> Self {
        match self {
            Self::Nested(_) => VariantHandler::Nested,
            Self::Replacement(_) => VariantHandler::Replacement,
        }
    }
}

impl PartialEq for VariantHandler {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nested(_), Self::Nested(_)) => true,
            (Self::Replacement(_), Self::Replacement(_)) => true,
            _ => false,
        }
    }
}

impl Eq for VariantHandler {}

impl PartialOrd for VariantHandler {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Nested(_), Self::Replacement(_)) => Some(Ordering::Greater),
            (Self::Replacement(_), Self::Nested(_)) => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Ord for VariantHandler {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
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

fn variant_fn<'a>(matcher: String) -> Option<VariantHandler> {
    let m = matcher.get(1..)?.to_owned();
    match matcher.chars().next()? {
        '&' => Some(VariantHandler::Replacement(Box::new(
            move |mut container: CssRuleList| {
                for rule in container.nodes.iter_mut() {
                    match rule {
                        CssRule::Style(ref mut it) => {
                            it.selector += m.as_str();
                        }
                        _ => {
                            println!("Mismatched rule: {:?}, expect a CssRule::Style", rule)
                        }
                    }
                }
                Some(container)
            },
        ))),
        '@' => Some(VariantHandler::Nested(Box::new(move |rule| {
            Some(
                CssRule::AtRule(AtRule {
                    name: m.to_owned(),
                    params: "".into(),
                    nodes: rule.nodes.to_vec(),
                })
                .into(),
            )
        }))),
        _ => None,
    }
}

pub fn create_variant_fn<'a, T>(
    _key: &str,
    matcher: T,
) -> Option<VariantHandler>
where
    T: IntoIterator,
    T::Item: AsRef<str>,
    T::IntoIter: ExactSizeIterator,
{
    let mut has_replacement = false;
    let fns = matcher
        .into_iter()
        // .map(|item| item.as_ref())
        .map(|s| {
            let s = s.as_ref();
            let this_fn: VariantHandler = if s.find('|').is_some() {
                let mut fns = s
                    .split('|')
                    .map(|matcher| matcher.trim())
                    .map(|item| variant_fn(item.into()))
                    .collect::<Option<Vec<_>>>()?;

                fns.sort();

                let wrapper = VariantHandler::create_constructor(&fns[0]);
                let composed_fn: Box<dyn VariantMatchingFn> = Box::new(move |rules| {
                    fns.iter().fold(Some(rules), |acc, f| {
                        acc.and_then(|r| f(r.clone()))
                    })
                });
                wrapper(composed_fn)
            } else {
                // Normal
                variant_fn(s.into())?
            };
            if matches!(this_fn, VariantHandler::Replacement(_)) {
                has_replacement = true;
            }
            Some(this_fn)
        })
        .collect::<Option<Vec<_>>>()?;

    let handler: Box<dyn VariantMatchingFn> =
        Box::new(move |mut container: CssRuleList| {
            container = fns
                .iter()
                .map(|f| f(container.clone()))
                .collect::<Option<CssRuleList>>()?;
            Some(container)
        });

    Some(if has_replacement {
        VariantHandler::Replacement(handler)
    } else {
        VariantHandler::Nested(handler)
    })
}

pub fn decode_arbitrary_value(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next_char) = chars.peek() {
                if *next_char == '_' {
                    chars.next();
                    output.push('_');
                    continue;
                }
            }
        }
        output.push(if c == '_' { ' ' } else { c });
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::utils::decode_arbitrary_value;

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

    #[test]
    fn test_decode_arbitrary_value() {
        assert_eq!(
            decode_arbitrary_value(r"hello\_world"),
            "hello_world".to_string()
        );
        assert_eq!(
            decode_arbitrary_value(r"hello_world"),
            "hello world".to_string()
        );
    }
}
