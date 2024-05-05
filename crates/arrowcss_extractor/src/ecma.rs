use cssparser_macros::match_byte;

use crate::cursor::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringType {
    SingleQuote,
    DoubleQuote,
    // Template,
}

pub struct EcmaExtractor<'a> {
    cursor: Cursor<'a>,
    input: &'a str,
    position: usize,
    at_start_of: Option<StringType>,
}

impl<'a> EcmaExtractor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
            input,
            position: 0,
            at_start_of: None,
        }
    }

    pub fn str_from(&self, start: usize) -> &'a str {
        self.str_from_to(start, self.position)
    }

    fn str_from_to(&self, from: usize, to: usize) -> &'a str {
        &self.input[from..to]
    }

    pub(crate) fn consume_until_string(&mut self) -> Result<StringType, ()> {
        while let Some(c) = self.cursor.bump() {
            match_byte! { c,
                b'/' => {
                    match self.cursor.first() {
                        '/' => self.consume_comment(),
                        '*' => { self.consume_block_comment(); },
                        _ => (),
                    }
                }
                b'\'' => {
                    self.at_start_of = Some(StringType::SingleQuote);
                    return Ok(StringType::SingleQuote);
                }
                b'"' => {
                    self.at_start_of = Some(StringType::DoubleQuote);
                    return Ok(StringType::DoubleQuote);
                }
                _ => (),
            }
        }
        Err(())
    }

    pub fn consume_comment(&mut self) {
        self.cursor.eat_while(|c| c != '\n');
    }

    // for /* */
    pub fn consume_block_comment(&mut self) -> bool {
        self.cursor.bump();

        let mut depth = 1usize;
        while let Some(c) = self.cursor.bump() {
            match c {
                '/' if self.cursor.first() == '*' => {
                    self.cursor.bump();
                    depth += 1;
                }
                '*' if self.cursor.first() == '/' => {
                    self.cursor.bump();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => (),
            }
        }
        depth == 0
    }

    pub fn consume_string(&mut self) -> bool {
        let string_type = self.at_start_of.expect(
            "consume_string should only be called after consume_until_string has been called",
        );
        self.at_start_of = None;

        match string_type {
            StringType::SingleQuote => self.consume_single_quoted_string(),
            StringType::DoubleQuote => self.consume_double_quoted_string(),
        }
    }

    pub fn consume_single_quoted_string(&mut self) -> bool {
        while let Some(c) = self.cursor.bump() {
            match c {
                '\'' => {
                    return true;
                }
                '\\' if self.cursor.first() == '\\' || self.cursor.first() == '\'' => {
                    self.cursor.bump();
                }
                _ => (),
            }
        }
        false
    }

    pub fn consume_double_quoted_string(&mut self) -> bool {
        while let Some(c) = self.cursor.bump() {
            match c {
                '"' => {
                    return true;
                }
                '\\' if self.cursor.first() == '\\' || self.cursor.first() == '"' => {
                    self.cursor.bump();
                }
                _ => (),
            }
        }
        false
    }
}

impl<'a> Iterator for EcmaExtractor<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_eof() {
            return None;
        }
        let res = match self.at_start_of {
            None => {
                self.consume_until_string().ok()?;
                self.next()
            }
            Some(_) => {
                let start = self.cursor.pos();
                self.consume_string();
                let end = self.cursor.pos() - 1;
                Some(self.str_from_to(start, end))
            }
        };
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_until_string() {
        let input = r#" 'string' "string" "#;
        let mut extractor = EcmaExtractor::new(input);
        assert_eq!(extractor.next(), Some("string"));
        assert_eq!(extractor.next(), Some("string"));
    }

    #[test]
    fn test_consume_line_comment() {
        let input = r#"
        // comment 'test'
        // comment "test"
        'this is a string'
        "this is a string"
        "#;
        let mut extractor = EcmaExtractor::new(input);
        assert_eq!(extractor.next(), Some("this is a string"));
        assert_eq!(extractor.next(), Some("this is a string"));
    }

    #[test]
    fn test_consume_block_comment() {
        let input = r#"
        /* comment 'test' */
        /*
        comment "test"
        /* nested comment "test" */
        /* nested comment 'test' */

        */
        */
        'this is a string'
        "this is a string"
        "#;
        let mut extractor = EcmaExtractor::new(input);
        assert_eq!(extractor.next(), Some("this is a string"));
        assert_eq!(extractor.next(), Some("this is a string"));
    }

    #[test]
    fn test_jsx() {
        let input = r#"
        <div
            className='this is string'
            className="this is string"
            className={"this is string"}
        >
            this is not a string
        </div>
        "#;
        let mut extractor = EcmaExtractor::new(input);
        assert_eq!(extractor.next(), Some("this is string"));
        assert_eq!(extractor.next(), Some("this is string"));
        assert_eq!(extractor.next(), Some("this is string"));
        assert_eq!(extractor.next(), None);
    }
}
