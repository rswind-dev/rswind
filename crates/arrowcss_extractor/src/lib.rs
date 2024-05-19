use cssparser::match_byte;
use ecma::EcmaExtractor;
use html::HtmlExtractor;
use rustc_hash::FxHashSet as HashSet;

pub mod css;
pub mod cursor;
pub mod ecma;
pub mod html;
pub mod item;

pub trait Extractable<'a> {
    fn extract(self) -> impl Iterator<Item = &'a str>;
}

pub struct BasicExtractor<'i> {
    pub haystack: &'i str,
}

impl<'i> BasicExtractor<'i> {
    pub fn new(haystack: &'i str) -> Self {
        Self { haystack }
    }

    pub fn extract_inner(&self) -> std::collections::hash_set::IntoIter<&'i str> {
        self.haystack
            .split(['\n', '\r', '\t', ' ', '"', '\'', ';', '{', '}', '`'])
            .filter(|s| {
                match_byte! { *s.as_bytes().first().unwrap_or(&b'\0'),
                    b'a'..=b'z' | b'-' | b'!' | b'[' => true,
                    _ => false,
                }
            })
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

impl<'a> Extractable<'a> for &'a str {
    fn extract(self) -> impl Iterator<Item = &'a str> {
        BasicExtractor::new(self).extract_inner()
    }
}

pub enum InputKind {
    Html,
    // Css,
    Ecma,
    Unknown,
}

impl From<&str> for InputKind {
    fn from(kind: &str) -> Self {
        match kind {
            "html" | "vue" | "svelte" => InputKind::Html,
            // "css" => InputKind::Css,
            "js" | "ts" | "jsx" | "tsx" | "mjs" | "mts" | "cjs" | "cts" => InputKind::Ecma,
            _ => InputKind::Unknown,
        }
    }
}

pub trait UniqueCandidate<'a> {
    fn filter_invalid(self) -> std::collections::hash_set::IntoIter<&'a str>;
}

impl<'a, T: Iterator<Item = &'a str>> UniqueCandidate<'a> for T {
    fn filter_invalid(self) -> std::collections::hash_set::IntoIter<&'a str> {
        self.flat_map(|s| s.split_ascii_whitespace())
            .filter(|s| {
                match_byte! { *s.as_bytes().first().unwrap_or(&b'\0'),
                    b'a'..=b'z' | b'-' | b'!' | b'[' => true,
                    _ => false,
                }
            })
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

pub struct Extractor<'a> {
    haystack: &'a str,
    kind: InputKind,
}

impl<'a> Extractor<'a> {
    pub fn new(haystack: &'a str, kind: impl Into<InputKind>) -> Self {
        Self {
            haystack,
            kind: kind.into(),
        }
    }
}

impl<'a> Extractable<'a> for Extractor<'a> {
    fn extract(self) -> impl Iterator<Item = &'a str> {
        match self.kind {
            InputKind::Html => HtmlExtractor::new(self.haystack).filter_invalid(),
            InputKind::Ecma => EcmaExtractor::new(self.haystack).filter_invalid(),
            InputKind::Unknown => BasicExtractor::new(self.haystack).extract_inner(),
        }
    }
}
