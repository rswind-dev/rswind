use arrowcss_extractor::{ecma::EcmaExtractor, html::HtmlExtractor, BasicExtractor, Extractor};
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, Clone, Copy)]
pub enum SourceInput<T: AsRef<str>> {
    Html(T),
    Ecma(T),
    Unknown(T),
}

pub enum SourceType {
    Html,
    Ecma,
    Unknown,
}

impl<T: AsRef<str>> AsRef<SourceInput<T>> for SourceInput<T> {
    fn as_ref(&self) -> &SourceInput<T> {
        self
    }
}

impl SourceType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Html => "html",
            Self::Ecma => "ecma",
            Self::Unknown => "unknown",
        }
    }
}

impl From<&str> for SourceType {
    fn from(s: &str) -> Self {
        match s {
            "html" | "vue" | "svelte" => Self::Html,
            "js" | "ts" | "jsx" | "tsx" | "mjs" | "mts" | "cjs" | "cts" => Self::Ecma,
            _ => Self::Unknown,
        }
    }
}

impl<T: AsRef<str>> SourceInput<T> {
    pub fn new(source: T, typ: impl Into<SourceType>) -> Self {
        match typ.into() {
            SourceType::Html => Self::Html(source),
            SourceType::Ecma => Self::Ecma(source),
            SourceType::Unknown => Self::Unknown(source),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Html(s) => s.as_ref(),
            Self::Ecma(s) => s.as_ref(),
            Self::Unknown(s) => s.as_ref(),
        }
    }

    pub fn as_unknown(self) -> Self {
        match self {
            Self::Unknown(_) => self,
            Self::Html(s) => Self::Unknown(s),
            Self::Ecma(s) => Self::Unknown(s),
        }
    }
}

impl<'i, T: AsRef<str>> Extractor<'i> for SourceInput<T> {
    fn extract(&'i self) -> Box<dyn Iterator<Item = &'i str> + 'i> {
        match self {
            Self::Html(s) => Box::new(
                HtmlExtractor::new(s.as_ref())
                    .flat_map(|s| s.split_ascii_whitespace())
                    .filter(|s| s.starts_with(char::is_lowercase) || s.starts_with('-'))
                    .collect::<HashSet<_>>()
                    .into_iter(),
            ),
            Self::Ecma(s) => {
                Box::new(EcmaExtractor::new(s.as_ref()).flat_map(|s| s.split_ascii_whitespace()))
            }
            Self::Unknown(s) => Box::new(BasicExtractor::new(s.as_ref())._extract()),
        }
    }
}
