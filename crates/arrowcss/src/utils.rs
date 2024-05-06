use std::str::pattern::{DoubleEndedSearcher, Pattern, ReverseSearcher, SearchStep, Searcher};

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

        let slice = unsafe { self.haystack.get_unchecked(old_finger..self.finger_back) };
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
        let slice = unsafe { self.haystack.get_unchecked(self.finger..old_finger) };
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
