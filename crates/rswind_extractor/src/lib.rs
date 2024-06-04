use cssparser::match_byte;
use ecma::EcmaExtractor;
use html::HtmlExtractor;
use rayon::iter::ParallelIterator;
use rswind_common::iter::MaybeParallelIterator;
use rustc_hash::FxHashSet as HashSet;

pub mod css;
pub mod cursor;
pub mod ecma;
pub mod html;
pub mod item;

pub trait Extractable<'a> {
    fn extract(self) -> HashSet<&'a str>;
}

pub struct BasicExtractor<'i> {
    pub haystack: &'i str,
}

impl<'i> BasicExtractor<'i> {
    pub fn new(haystack: &'i str) -> Self {
        Self { haystack }
    }

    pub fn extract_inner(&self) -> HashSet<&'i str> {
        self.haystack
            .split(['\n', '\r', '\t', ' ', '"', '\'', ';', '{', '}', '`'])
            .filter(|s| {
                match_byte! { *s.as_bytes().first().unwrap_or(&b'\0'),
                    b'a'..=b'z' | b'-' | b'!' | b'[' => true,
                    _ => false,
                }
            })
            .collect::<HashSet<_>>()
    }
}

impl<'a> Extractable<'a> for &'a str {
    fn extract(self) -> HashSet<&'a str> {
        BasicExtractor::new(self).extract_inner()
    }
}

#[derive(Debug)]
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
    fn filter_invalid(self) -> HashSet<&'a str>;
}

impl<'a, T: Iterator<Item = &'a str>> UniqueCandidate<'a> for T {
    fn filter_invalid(self) -> HashSet<&'a str> {
        self.flat_map(|s| s.split_ascii_whitespace())
            .filter(|s| {
                match_byte! { *s.as_bytes().first().unwrap_or(&b'\0'),
                    b'a'..=b'z' | b'-' | b'!' | b'[' => true,
                    _ => false,
                }
            })
            .collect::<HashSet<_>>()
    }
}

#[derive(Debug)]
pub struct Extractor<'a> {
    haystack: &'a str,
    kind: InputKind,
}

impl<'a> Extractor<'a> {
    pub fn new(haystack: &'a str, kind: impl Into<InputKind>) -> Self {
        Self { haystack, kind: kind.into() }
    }
}

impl<'a> From<&'a str> for Extractor<'a> {
    fn from(haystack: &'a str) -> Self {
        Self::new(haystack, InputKind::Unknown)
    }
}

impl<'a, K: Into<InputKind>> From<(&'a str, K)> for Extractor<'a> {
    fn from((haystack, kind): (&'a str, K)) -> Self {
        Self::new(haystack, kind)
    }
}

impl<'a> Extractable<'a> for Extractor<'a> {
    fn extract(self) -> HashSet<&'a str> {
        match self.kind {
            InputKind::Html => HtmlExtractor::new(self.haystack).filter_invalid(),
            InputKind::Ecma => EcmaExtractor::new(self.haystack).filter_invalid(),
            InputKind::Unknown => BasicExtractor::new(self.haystack).extract_inner(),
        }
    }
}

pub trait CollectExtracted<'a> {
    fn collect_extracted(self) -> HashSet<&'a str>;
}

impl<'a, I: Iterator<Item = T>, T: Into<Extractor<'a>>> CollectExtracted<'a> for I {
    fn collect_extracted(self) -> HashSet<&'a str> {
        self.map(Into::into).flat_map(Extractable::extract).collect::<HashSet<_>>()
    }
}

pub trait ParCollectExtracted<'a> {
    fn collect_extracted(self) -> HashSet<&'a str>;
}

impl<'a, I: ParallelIterator<Item = T>, T: Into<Extractor<'a>> + Send> ParCollectExtracted<'a>
    for I
{
    fn collect_extracted(self) -> HashSet<&'a str> {
        self.map(Into::into).map(Extractable::extract).reduce(HashSet::default, |mut acc, i| {
            acc.extend(i);
            acc
        })
    }
}

pub trait MaybeParCollectExtracted<'a> {
    fn collect_extracted(self) -> HashSet<&'a str>;
}

impl<'a, I, T> MaybeParCollectExtracted<'a> for I
where
    I: MaybeParallelIterator<Item = T>,
    T: Into<Extractor<'a>> + Send,
{
    fn collect_extracted(self) -> HashSet<&'a str> {
        self.map(Into::into).map(Extractable::extract).reduce(HashSet::default, |mut acc, i| {
            acc.extend(i);
            acc
        })
    }
}
