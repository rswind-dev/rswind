use std::{
    cmp::Ordering,
    str::pattern::{
        DoubleEndedSearcher, Pattern, ReverseSearcher, Searcher, SearchStep,
    },
};

use crate::{
    context::VariantMatchingFn,
    css::{AstNode, NodeList, Rule},
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

    pub fn create_constructor(
        &self,
    ) -> impl Fn(Box<dyn VariantMatchingFn>) -> Self {
        match self {
            Self::Nested(_) => VariantHandler::Nested,
            Self::Replacement(_) => VariantHandler::Replacement,
        }
    }
}

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

pub struct TopLevelPattern<'a, P> {
    needle: P,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl TopLevelPattern<'_, char> {
    pub fn new(needle: char) -> Self {
        Self {
            needle,
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct TopLevelCharSearcher<'a> {
    haystack: &'a str,
    needle: char,
    finger: usize,
    finger_back: usize,
    // for "()"
    parentheses: isize,
    // for "[]"
    brackets: isize,
}

impl<'a> TopLevelCharSearcher<'a> {
    fn new(haystack: &'a str, needle: char) -> Self {
        Self {
            haystack,
            needle,
            finger: 0,
            finger_back: haystack.len(),
            parentheses: 0,
            brackets: 0,
        }
    }

    fn is_top_level(&self) -> bool {
        self.parentheses == 0 && self.brackets == 0
    }
}

struct LocalChars<'a> {
    iter: core::slice::Iter<'a, u8>,
}

unsafe impl<'a> Searcher<'a> for TopLevelCharSearcher<'a> {
    fn next(&mut self) -> SearchStep {
        let old_finger = self.finger;

        let slice = unsafe {
            self.haystack.get_unchecked(old_finger..self.finger_back)
        };
        let mut iter = slice.chars();
        let local_iter: &LocalChars = unsafe { std::mem::transmute(&iter) };
        let old_len = local_iter.iter.len();

        if let Some(ch) = iter.next() {
            self.finger += old_len - local_iter.iter.len();
            match ch {
                '(' => self.parentheses += 1,
                ')' => self.parentheses -= 1,
                '[' => self.brackets += 1,
                ']' => self.brackets -= 1,
                _ => (),
            }
            if ch == self.needle && self.is_top_level() {
                SearchStep::Match(old_finger, self.finger)
            } else {
                SearchStep::Reject(old_finger, self.finger)
            }
        } else {
            SearchStep::Done
        }
    }

    fn haystack(&self) -> &'a str {
        self.haystack
    }
}

unsafe impl<'a> ReverseSearcher<'a> for TopLevelCharSearcher<'a> {
    fn next_back(&mut self) -> SearchStep {
        let old_finger = self.finger_back;
        // SAFETY: see the comment for next() above
        let slice =
            unsafe { self.haystack.get_unchecked(self.finger..old_finger) };
        let mut iter = slice.chars();
        let local_iter: &LocalChars = unsafe { std::mem::transmute(&iter) };

        let old_len = local_iter.iter.len();
        if let Some(ch) = iter.next_back() {
            // subtract byte offset of current character
            // without re-encoding as utf-8
            self.finger_back -= old_len - local_iter.iter.len();
            match ch {
                '(' => self.parentheses += 1,
                ')' => self.parentheses -= 1,
                '[' => self.brackets += 1,
                ']' => self.brackets -= 1,
                _ => (),
            }
            if ch == self.needle && self.is_top_level() {
                SearchStep::Match(self.finger_back, old_finger)
            } else {
                SearchStep::Reject(self.finger_back, old_finger)
            }
        } else {
            SearchStep::Done
        }
    }
}

impl<'a> DoubleEndedSearcher<'a> for TopLevelCharSearcher<'a> {}

impl<'a> Pattern<'a> for TopLevelPattern<'a, char> {
    type Searcher = TopLevelCharSearcher<'a>;

    fn into_searcher(self, haystack: &'a str) -> Self::Searcher {
        TopLevelCharSearcher::new(haystack, self.needle)
    }
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
