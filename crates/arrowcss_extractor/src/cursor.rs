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

    #[allow(dead_code)]
    pub(crate) fn second(&self) -> char {
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

    pub(crate) fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Eat a str if it matches the current cursor position
    /// Returns true if the str was eaten
    /// Returns false if the str was not eaten
    ///
    /// self.chars will be at the position after the str
    /// when not match,
    pub(crate) fn eat_str(&mut self, s: &str) -> bool {
        let state = self.chars.clone();
        if s.chars().all(|c| self.bump() == Some(c)) {
            true
        } else {
            self.chars = state;
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

    pub(crate) fn eat_until(&mut self, mut predicate: impl FnMut(char) -> bool) -> &mut Self {
        while !predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
        self
    }

    pub(crate) fn eat_while_cursor(&mut self, mut predicate: impl FnMut(&Cursor) -> bool) {
        while predicate(self) && !self.is_eof() {
            self.bump();
        }
    }

    pub(crate) fn eat_until_cursor(&mut self, mut predicate: impl FnMut(&Cursor) -> bool) {
        while !predicate(self) && !self.is_eof() {
            self.bump();
        }
    }
}
