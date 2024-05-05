use arrowcss_extractor::{ecma::EcmaExtractor, html::HtmlExtractor};
use cssparser_macros::match_byte;
use fxhash::FxHashSet as HashSet;

use crate::common::BasicParser;

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

pub trait Extractor<'i> {
    fn extract(&'i self) -> Box<dyn Iterator<Item = &'i str> + 'i>;
}

pub struct BasicExtractor<'i> {
    pub haystack: &'i str,
}

impl<'i> BasicExtractor<'i> {
    pub fn new(haystack: &'i str) -> Self {
        Self { haystack }
    }

    fn _extract(&self) -> impl Iterator<Item = &'i str> + 'i {
        self.haystack
            .split(['\n', '\r', '\t', ' ', '"', '\'', ';', '{', '}', '`'])
            .filter(|s| s.starts_with(char::is_lowercase) || s.starts_with('-'))
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

impl<'i> Extractor<'i> for BasicExtractor<'i> {
    fn extract(&self) -> Box<dyn Iterator<Item = &'i str> + 'i> {
        Box::new(self._extract())
    }
}

impl BasicParser for StringExtractor<'_> {
    fn next_byte(&self) -> u8 {
        self.byte_at(0)
    }

    fn is_eof(&self) -> bool {
        !self.has_at_least(0)
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }
}

#[derive(Debug, Clone)]
pub struct StringExtractor<'a> {
    position: usize,
    input: &'a str,
}

impl<'a> StringExtractor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { position: 0, input }
    }

    fn byte_at(&self, offset: usize) -> u8 {
        self.input.as_bytes()[self.position + offset]
    }

    fn has_at_least(&self, n: usize) -> bool {
        self.position + n < self.input.len()
    }

    pub fn consume_until_candidate(&mut self) {
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b'[' | b'@' | b'!' | b'-' | b'<' | b'>' | b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'*' => {
                    break;
                }
                _ => {
                    self.skip_until_candidate();
                }
            }
        }
    }

    pub fn skip_until_candidate(&mut self) {
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b' ' => {
                    self.advance(1);
                    break;
                }
                _ => {
                    self.advance(1);
                }
            }
        }
    }

    /// Consume arbitrary content
    /// Arbitrary content is enclosed in square brackets
    /// '_' will be replaced with ' ' (space)
    /// ']' and ' ' literal must be escaped with '\'
    /// e.g. [#f00] [a] [@media(min-width:_640px)] [&:hover] [color:red]
    pub fn consume_arbitrary(&mut self) {
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b']' => {
                    self.advance(1);
                    break;
                }
                b'\\' => {
                    self.advance(1);
                    match_byte! { self.next_byte(),
                        b']' | b' ' => {
                            self.advance(1);
                        }
                        _ => {}
                    }
                }
                _ => {
                    self.advance(1);
                }
            }
        }
    }

    pub fn consume_candidate(&mut self) {
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b'@' | b'-' | b':' | b'/' | b'!' => {
                    self.advance(1);
                    if !self.is_eof() && self.next_byte() == b'[' {
                        self.consume_arbitrary();
                    }
                }
                // b'[' => {
                //     self.consume_arbitrary();
                // }
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'.' | b'-' | b'_' | b'@' => {
                    self.advance(1);
                }
                _ => {
                    break;
                }
            }
        }
    }
}

impl<'a> Iterator for StringExtractor<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_until_candidate();
        if self.is_eof() {
            return None;
        }
        let start = self.position;
        self.consume_candidate();
        let candidate = &self.input[start..self.position];
        Some(candidate)
    }
}

impl<'i> Extractor<'i> for StringExtractor<'i> {
    fn extract(&self) -> Box<dyn Iterator<Item = &'i str> + 'i> {
        Box::new(self.clone())
    }
}
