use std::{fs::read_to_string, path::PathBuf};

use arrowcss_extractor::{ecma::EcmaExtractor, html::HtmlExtractor};
use cssparser_macros::match_byte;
use fxhash::FxHashSet as HashSet;

use crate::common::BasicParser;

pub enum SourceType<T = String> {
    Html(T),
    Ecma(T),
    Unknown(T),
}

impl Default for SourceType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

impl SourceType<String> {
    pub fn new(source: String, typ: &str) -> Self {
        match typ {
            "html" => Self::Html(source),
            "js" | "ts" | "jsx" | "tsx" | "mjs" | "mts" | "cjs" | "cts" => Self::Ecma(source),
            _ => Self::Unknown(source),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Html(s) => s,
            Self::Ecma(s) => s,
            Self::Unknown(s) => s,
        }
    }

    pub fn from_file(f: &PathBuf) -> Self {
        Self::new(
            read_to_string(f).unwrap(),
            f.extension().unwrap().to_str().unwrap_or_default(),
        )
    }

    pub fn as_unknown(self) -> Self {
        match self {
            Self::Unknown(_) => self,
            Self::Html(s) => Self::Unknown(s),
            Self::Ecma(s) => Self::Unknown(s),
        }
    }
}

impl<'i> Extractor<'i> for SourceType<String> {
    fn extract(&'i self) -> Box<dyn Iterator<Item = &'i str> + 'i> {
        match self {
            Self::Html(s) => Box::new(
                HtmlExtractor::new(s)
                    .into_iter()
                    .flat_map(|s| StringExtractor::new(s))
                    .collect::<HashSet<_>>()
                    .into_iter(),
            ),
            Self::Ecma(s) => Box::new(EcmaExtractor::new(s).flat_map(|s| StringExtractor::new(s))),
            Self::Unknown(s) => Box::new(BasicExtractor::new(s)._extract()),
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
