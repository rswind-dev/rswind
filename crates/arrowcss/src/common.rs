use std::{fmt::Debug, ops::Deref};

use smallvec::SmallVec;
use smol_str::format_smolstr;

pub trait StrReplaceExt {
    fn replace_char(&self, chr: char, replacement: &str) -> smol_str::SmolStr;
}

impl StrReplaceExt for str {
    fn replace_char(&self, chr: char, replacement: &str) -> smol_str::SmolStr {
        if let Some(pos) = memchr::memchr(chr as u8, self.as_bytes()) {
            return format_smolstr!("{}{}{}", &self[..pos], replacement, &self[pos + 1..]);
        }
        smol_str::SmolStr::from(self)
    }
}

pub trait StrSplitExt {
    fn split_toplevel(&self, delimiter: u8) -> Option<SmallVec<[&str; 2]>>;
}

impl StrSplitExt for str {
    fn split_toplevel(&self, delimiter: u8) -> Option<SmallVec<[&str; 2]>> {
        let mut result = smallvec::smallvec![];
        let mut start = 0;
        let mut in_brackets = 0;

        let bytes = self.as_bytes();

        for index in memchr::memchr3_iter(delimiter, b'[', b']', bytes) {
            match bytes[index] {
                b':' if in_brackets == 0 => {
                    result.push(&self[start..index]);
                    start = index + 1;
                }
                b'[' => {
                    in_brackets += 1;
                }
                b']' => {
                    in_brackets -= 1;

                    if in_brackets < 0 {
                        return None;
                    }
                }
                _ => {}
            }
        }

        result.push(&self[start..]);

        Some(result)
    }
}

pub trait BasicParser {
    fn advance(&mut self, n: usize);
    fn is_eof(&self) -> bool;
    fn next_byte(&self) -> u8;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MaybeArbitrary<'a> {
    Arbitrary(&'a str),
    Named(&'a str),
}

impl MaybeArbitrary<'_> {
    pub fn take_arbitrary(&self) -> Option<&str> {
        match self {
            MaybeArbitrary::Arbitrary(s) => Some(s),
            _ => None,
        }
    }

    pub fn take_named(&self) -> Option<&str> {
        match self {
            MaybeArbitrary::Named(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MaybeArbitrary::Arbitrary(s) => s,
            MaybeArbitrary::Named(s) => s,
        }
    }
}

impl Default for MaybeArbitrary<'_> {
    fn default() -> Self {
        MaybeArbitrary::Named("")
    }
}

impl<'a> Deref for MaybeArbitrary<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeArbitrary::Arbitrary(s) => s,
            MaybeArbitrary::Named(s) => s,
        }
    }
}

pub trait Inspector {
    fn dbg(self) -> Self;
    fn also(self, f: impl FnOnce(&Self)) -> Self;
}

impl<T> Inspector for T
where
    T: Debug,
{
    fn dbg(self) -> Self {
        dbg!(&self);
        self
    }

    fn also(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }
}

pub trait ScopeFunctions: Sized {
    fn run_if<B: FnOnce(Self) -> Self + 'static>(self, predictor: bool, block: B) -> Self;
    fn run_unless<B: FnOnce(Self) -> Self + 'static>(self, predictor: bool, block: B) -> Self {
        self.run_if(!predictor, block)
    }
}

impl<T> ScopeFunctions for T {
    fn run_if<B: FnOnce(Self) -> Self + 'static>(self, predictor: bool, block: B) -> Self {
        if predictor {
            block(self)
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_replace() {
        let s = "hello";
        assert_eq!(s.replace_char('e', "abc").as_str(), "habcllo");
    }
}
