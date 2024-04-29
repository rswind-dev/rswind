use cssparser_macros::match_byte;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringType {
    DoubleQuote,
}

#[derive(Debug, Clone)]
pub struct HtmlExtractor<'a> {
    input: &'a str,
    position: usize,
    inside_tag: bool,
}

// impl BasicParser for HtmlExtractor<'_> {

// }

impl<'a> HtmlExtractor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            inside_tag: false,
        }
    }

    #[inline]
    fn next_byte(&self) -> u8 {
        self.byte_at(0)
    }

    #[inline]
    fn is_eof(&self) -> bool {
        !self.has_at_least(0)
    }

    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    #[inline]
    fn byte_at(&self, offset: usize) -> u8 {
        self.input.as_bytes()[self.position + offset]
    }

    #[inline]
    fn has_at_least(&self, n: usize) -> bool {
        self.position + n < self.input.len()
    }

    pub fn consume_until_class(&mut self) -> Option<()> {
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b'<' => {
                    self.advance(1);
                    if self.next_byte() == b'/' {
                        self.advance(1);
                        self.consume_until_close();
                    }
                    self.inside_tag = true;
                }
                b'>' => {
                    self.advance(1);
                    self.inside_tag = false;
                }
                b'=' => {
                    if !self.inside_tag {
                        self.advance(1);
                        continue;
                    }
                    let start = self.position;
                    let is_class = self.inside_tag && self.input.get(start-5..start).map(|s| s == "class").unwrap_or_default();

                    if is_class {
                        self.advance(1);
                        return Some(());
                    }
                    self.advance(1);
                }
                _ => {
                    self.advance(1);
                }
            }
        }
        None
    }

    pub fn consume_string(&mut self) -> Option<&'a str> {
        let quote = self.next_byte();
        self.advance(1);
        let start = self.position;
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b'"' => {
                    let res = &self.input[start..self.position];
                    self.advance(1);
                    return Some(res);
                }
                _ => {
                    self.advance(1);
                }
            }
        }
        None
    }

    pub fn consume_until_close(&mut self) {
        while !self.is_eof() {
            match_byte! { self.next_byte(),
                b'>' => {
                    self.advance(1);
                    return;
                }
                _ => {
                    self.advance(1);
                }
            }
        }
    }
}

impl<'a> Iterator for HtmlExtractor<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        }
        // let start = std::time::Instant::now();
        // let pos = self.position;
        self.consume_until_class()?;
        // println!(
        //     "{:?} => {:?}",
        //     start.elapsed(),
        //     self.position  - pos
        // );
        self.consume_string()
        // println!("{:?}", s);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_basic_usage() {
        let input = r#"
            <div class="flex">
                <span class="bg-red-500" src="http://xxx.xxx">
                </span>
            </div>
        "#;
        let extractor = HtmlExtractor {
            input,
            position: 0,
            inside_tag: false,
        };
        extractor.for_each(|item| {
            // println!("{:?}", item);
            // let ce = StringExtractor::new(item);
        });
    }
}
