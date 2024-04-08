use std::cmp::Ordering;

use crate::css::{AstNode, NodeList, Rule};

#[rustfmt::skip]
pub trait VariantMatchingFn: Fn(NodeList) -> Option<NodeList> + Sync + Send {}

#[rustfmt::skip]
impl<T: Fn(NodeList) -> Option<NodeList> + Sync + Send> VariantMatchingFn for T {}

#[allow(dead_code)]
pub struct VariantProcessor {
    handler: VariantHandler,
    compundable: bool,
}

pub enum VariantHandler {
    Nested(Box<dyn VariantMatchingFn>),
    Replacement(Box<dyn VariantMatchingFn>),
}

impl VariantHandler {
    pub fn as_handler(&self) -> &Box<dyn VariantMatchingFn> {
        match self {
            Self::Nested(f) => f,
            Self::Replacement(f) => f,
        }
    }

    pub fn create_constructor(
        &self,
    ) -> impl Fn(Box<dyn VariantMatchingFn>) -> Self {
        match self {
            Self::Nested(_) => VariantHandler::Nested,
            Self::Replacement(_) => VariantHandler::Replacement,
        }
    }
}

fn variant_fn(matcher: String) -> Option<VariantHandler> {
    let m = matcher.get(1..)?.to_owned();
    match matcher.chars().next()? {
        '&' => Some(VariantHandler::Replacement(Box::new(
            move |container: NodeList| {
                container.into_iter().map(|rule| {
                    match rule {
                        AstNode::Rule(it) => {
                            AstNode::Rule(Rule {
                                selector: it.selector + &m,
                                nodes: it.nodes,
                            })
                        }
                        _ => {
                            println!("Mismatched rule: {:?}, expect a CssRule::Style", rule);
                            rule.clone()
                        }
                    }
                }).collect::<Vec<_>>().into()
            },
        ))),
        '@' => Some(VariantHandler::Nested(Box::new(move |rule| {
            Some(
                AstNode::Rule(Rule {
                    selector: matcher.to_owned(),
                    nodes: rule.to_vec(),
                })
                .into(),
            )
        }))),
        _ => None,
    }
}

pub fn create_variant_fn<T>(_key: &str, matcher: T) -> Option<VariantHandler>
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
                let composed_fn: Box<dyn VariantMatchingFn> =
                    Box::new(move |rules| {
                        fns.iter().try_fold(rules, |acc, f| f(acc))
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
        Box::new(move |container: NodeList| {
            fns.iter()
                .map(|f| f(container.clone()))
                .collect::<Option<Vec<Vec<AstNode>>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<AstNode>>()
                .into()
        });

    Some(if has_replacement {
        VariantHandler::Replacement(handler)
    } else {
        VariantHandler::Nested(handler)
    })
}

// ----- Trait Implementations -----

impl PartialEq for VariantHandler {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Nested(_), Self::Nested(_))
                | (Self::Replacement(_), Self::Replacement(_))
        )
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

impl<'a> Fn<(NodeList<'a>,)> for VariantHandler {
    extern "rust-call" fn call(
        &self,
        args: (NodeList<'a>,),
    ) -> Option<NodeList<'a>> {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Replacement(f) => f(args.0),
        }
    }
}

impl<'a> FnOnce<(NodeList<'a>,)> for VariantHandler {
    type Output = Option<NodeList<'a>>;

    extern "rust-call" fn call_once(
        self,
        args: (NodeList<'a>,),
    ) -> Self::Output {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Replacement(f) => f(args.0),
        }
    }
}

impl<'a> FnMut<(NodeList<'a>,)> for VariantHandler {
    extern "rust-call" fn call_mut(
        &mut self,
        args: (NodeList<'a>,),
    ) -> Option<NodeList<'a>> {
        match self {
            VariantHandler::Nested(f) => f(args.0),
            VariantHandler::Replacement(f) => f(args.0),
        }
    }
}
