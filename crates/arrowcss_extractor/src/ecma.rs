use cssparser_macros::match_byte;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringType {
    SingleQuote,
    DoubleQuote,
    // Template,
}

#[derive(Debug, Clone)]
pub struct EcmaExtractor<'a> {
    input: &'a str,
    position: usize,
    at_start_of: Option<StringType>,
}

// impl BasicParser for Extractor<'_> {

// }

impl<'a> EcmaExtractor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            at_start_of: None,
        }
    }

    fn next_byte(&self) -> u8 {
        self.byte_at(0)
    }

    fn is_eof(&self) -> bool {
        !self.has_at_least(0)
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn byte_at(&self, offset: usize) -> u8 {
        self.input.as_bytes()[self.position + offset]
    }

    fn has_at_least(&self, n: usize) -> bool {
        self.position + n < self.input.len()
    }

    pub(crate) fn consume_until_string(&mut self) -> Result<StringType, ()> {
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b'\'' => {
                    self.at_start_of = Some(StringType::SingleQuote);
                    self.advance(1);
                    return Ok(StringType::SingleQuote)
                }
                b'"' => {
                    self.at_start_of = Some(StringType::DoubleQuote);
                    self.advance(1);
                    return Ok(StringType::DoubleQuote)
                }
                b'/' => {
                    self.advance(1);
                    match_byte! { self.next_byte(),
                        b'/' => {
                            self.advance(1);
                            self.consume_comment();
                        }
                        b'*' => {
                            self.advance(1);
                            self.consume_multiline_comment();
                        }
                        _ => {}
                    }
                }
                _ => {
                    self.advance(1);
                }
            }
        }
        Err(())
    }

    pub fn consume_comment(&mut self) {
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b'\n' => {
                    self.advance(1);
                    break;
                }
                _ => {
                    self.advance(1);
                }
            }
        }
    }

    // for /* */
    pub fn consume_multiline_comment(&mut self) {
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b'*' => {
                    self.advance(1);
                    if self.next_byte() == b'/' {
                        self.advance(1);
                        break;
                    }
                }
                _ => {
                    self.advance(1);
                }
            }
        }
    }

    pub fn consume_string(&mut self) -> Result<&'a str, ()> {
        let string_type = self.at_start_of.expect(
            "consume_string should only be called after consume_until_string has been called",
        );
        let start = self.position;

        while !self.is_eof() {
            let byte = self.next_byte();
            match_byte! { byte,
                    b'\'' => {
                        if string_type == StringType::SingleQuote {
                            self.at_start_of = None;
                            let res = Ok(&self.input[start..self.position]);
                            self.advance(1);
                            return res;
                        } else {
                            self.advance(1);
                        }
                    }
                    b'"' => {
                        if string_type == StringType::DoubleQuote {
                            self.at_start_of = None;
                            let res =  Ok(&self.input[start..self.position]);
                            self.advance(1);
                            return res;
                        } else {
                            self.advance(1);
                        }
                    }
                    b'\\' => {
                        consume_escape_sequence(self)?;
                    }
                    _ => {
                        self.advance(1);
                    }
            }
        }
        Err(())
    }
}

#[inline]
#[cold]
fn consume_hex_digit_once(parser: &mut EcmaExtractor<'_>) {
    match_byte! { parser.next_byte(),
        b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
            parser.advance(1);
        }
        _ => {}
    }
}

#[cold]
fn consume_hex_digit(parser: &mut EcmaExtractor<'_>, time: usize) -> Result<(), ()> {
    for _ in 0..time {
        if parser.is_eof() {
            return Err(());
        }
        consume_hex_digit_once(parser);
    }
    Ok(())
}

#[cold]
fn consume_code_point<'a>(parser: &mut EcmaExtractor<'a>) -> Result<&'a str, ()> {
    let start = parser.position;
    while !parser.is_eof() {
        match_byte! { parser.next_byte(),
            b'}' => {
                let cp = &parser.input[start..parser.position];
                let code_point = u32::from_str_radix(cp, 16).unwrap();
                return if code_point <= 0x10FFFF {
                    parser.advance(1);
                    Ok(cp)
                } else {
                    Err(())
                }
            }
            _ => {
                consume_hex_digit_once(parser);
            }
        }
    }
    Err(())
}

#[cold]
fn consume_escape_sequence(parser: &mut EcmaExtractor<'_>) -> Result<(), ()> {
    parser.advance(1);
    match_byte! { parser.next_byte(),
        b'\'' | b'"' | b'\\' | b'b' | b'f' | b'n' | b'r' | b't' | b'v' => {
            parser.advance(1);
            return Ok(());
        }
        b'0'..=b'3' => {
            if !parser.is_eof() {
                consume_hex_digit_once(parser);
                return Ok(());
            } else {
                return Err(());
            }
        }
        b'4'..=b'7' => {
            consume_hex_digit_once(parser);
            return Ok(());
        }
        b'8'..=b'9' => {
            return Ok(());
        }
        b'x' => {
            parser.advance(1);
            let _ = consume_hex_digit(parser, 2);
        }
        b'u' => {
            parser.advance(1);
            match_byte! { parser.next_byte(),
                b'{' => {
                    parser.advance(1);
                    consume_code_point(parser)?;
                }
                _ => {
                    let _ = consume_hex_digit(parser, 4);
                }
            }
        }
        _ => {}
    }

    Ok(())
}

impl<'a> Iterator for EcmaExtractor<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        }
        match self.at_start_of {
            None => {
                self.consume_until_string().ok()?;
                self.next()
            }
            Some(_) => self.consume_string().ok(),
        }
    }
}
