use std::str::Chars;

#[derive(Debug)]
pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            chars: input.chars(),
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn third(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn pos(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    pub(crate) fn try_bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub(crate) fn bump(&mut self) -> char {
        self.chars.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn as_bytes(&self) -> &'a [u8] {
        self.chars.as_str().as_bytes()
    }

    /// Eat a str if it matches the current cursor position
    ///
    /// Returns a bool indicate whether the str was eaten
    ///
    /// self.chars will be at the position after the str
    /// when not match,
    pub(crate) fn eat_str(&mut self, s: &str) -> bool {
        let checkpoint = self.chars.clone();
        if s.chars().all(|c| self.bump() == c) {
            true
        } else {
            self.chars = checkpoint;
            false
        }
    }

    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub(crate) fn eat_whitespace(&mut self) {
        self.eat_while(char::is_whitespace);
    }

    pub(crate) fn eat_until(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while !predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub(crate) fn eat_until_char(&mut self, c: u8) -> &'a str {
        let Some(pos) = memchr::memchr(c, self.as_bytes()) else {
            // not found, eat all
            self.chars = "".chars();
            return self.chars.as_str();
        };
        let (slice, next) = self.chars.as_str().split_at(pos);
        self.chars = next.chars();
        slice
    }

    pub(crate) fn eat_until_after_char(&mut self, c: u8) -> &'a str {
        let res = self.eat_until_char(c);
        self.bump();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let mut cursor = Cursor::new("hello world");

        let ate = cursor.eat_until_char(b'w');

        assert_eq!(ate, "hello ");
        assert_eq!(cursor.as_str(), "world");
    }
}
