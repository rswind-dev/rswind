use crate::{cursor::Cursor, ecma::EcmaExtractor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringType {
    DoubleQuote,
}

/// Extractor for HTML / Vue
///
/// We only care about the attribute value and the JS expressions
/// e.g.
/// ```html
/// <div class="foo" :class="bar"></div>
/// <div v-bind:style="{ color: 'red' }"></div>
/// ```
/// will be extracted as:
/// [ foo bar red ]
pub struct HtmlExtractor<'a> {
    input: &'a str,
    cursor: Cursor<'a>,
    in_js: Option<Box<dyn Iterator<Item = &'a str> + 'a>>,
    in_start_tag: bool,
    options: HtmlExtractOptions,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FileType {
    #[default]
    Html,
    Vue,
    Svelte,
}

impl FileType {
    pub fn from_suffix(suffix: &str) -> Self {
        match suffix {
            "vue" => Self::Vue,
            "svelte" => Self::Svelte,
            _ => Self::Html,
        }
    }
}

#[derive(Default)]
pub struct HtmlExtractOptions {
    pub class_only: bool,
    pub file_type: FileType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateValue<'a> {
    Plain(&'a str),
    Ecma,
}

impl<'a> HtmlExtractor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: Cursor::new(input),
            in_js: None,
            in_start_tag: false,
            options: HtmlExtractOptions::default(),
        }
    }

    pub fn with_options(self, options: HtmlExtractOptions) -> Self {
        Self { options, ..self }
    }

    pub fn str_from(&self, start: usize) -> &'a str {
        &self.input[start..self.cursor.pos()]
    }

    fn consume(&mut self, f: impl FnOnce(&mut Cursor<'a>)) -> &'a str {
        let start = self.cursor.pos();
        f(&mut self.cursor);
        self.str_from(start)
    }

    fn consume_tag_name(&mut self) -> &'a str {
        self.consume(|s| {
            // first char is always valid start char
            s.bump();
            s.eat_while(is_name_char);
        })
    }

    fn consume_whitespace(&mut self) {
        self.cursor.eat_while(|c| c.is_whitespace());
    }

    fn in_start_tag(&mut self) -> bool {
        self.in_start_tag
    }

    fn extend_js_extractor(&mut self, iter: impl Iterator<Item = &'a str> + 'a) {
        if let Some(old) = self.in_js.take() {
            self.in_js = Some(Box::new(old.chain(iter)));
        } else {
            self.in_js = Some(Box::new(iter));
        }
    }

    fn consume_until_value(&mut self) -> Option<CandidateValue<'a>> {
        // move to start tag, skip end tag
        // e.g. <div>

        // valid start tag
        self.cursor
            .eat_until_cursor(|c| c.first() == '<' && is_name_start_char(c.second()));
        self.cursor.bump();
        self.in_start_tag = true;

        // eat tag name
        let tag_name = self.consume_tag_name();
        let end_or_space = self.cursor.bump().unwrap_or('\0');

        match tag_name {
            // use `EcmaExtractor` to extract JS str lit
            "script" => {
                // skip start tag
                // TODO: handle class name here?
                if end_or_space != '>' {
                    self.cursor.eat_until(|c| c == '>').bump();
                }

                // read js content
                let js = self.consume(|c| {
                    c.eat_until_cursor(|c| {
                        c.first() == '<' && c.second() == '/' && c.as_str().starts_with("</script>")
                    });
                });
                dbg!(self.cursor.as_str());

                // skip end tag
                self.cursor.eat_while(|c| c != '>');
                self.cursor.bump();

                self.in_js = Some(Box::new(EcmaExtractor::new(js)));
                return Some(CandidateValue::Ecma);
            }
            // skip
            "style" => {
                self.cursor.eat_while(|c| c != '>');
                self.cursor.bump();

                self.cursor
                    .eat_while_cursor(|c| c.first() != '<' && c.second() != '/');

                self.cursor.eat_while(|c| c != '>');
                self.cursor.bump();

                return None;
            }
            _ => (),
        }

        None
    }

    fn consume_attrs(&mut self) -> Option<CandidateValue<'a>> {
        self.cursor.eat_while(|c| c.is_whitespace());
        if self.cursor.is_eof() {
            return None;
        }

        // <div class="foo"> <div class> <div class >
        let name = self.consume(|s| s.eat_while(|c| c != '=' && c != '>' && !c.is_whitespace()));

        if self.options.class_only && !name.starts_with("class") && !name.starts_with(':') {
            return None;
        }

        match self.cursor.first() {
            '=' => {
                // jump the `=`
                self.cursor.bump();
                // jump the `"` or `{` (svelte)
                let start = self.cursor.bump().unwrap_or('\0');

                // filter out invalid start
                if start != '"' && start != '{' {
                    return None;
                }

                let end = if start == '{' { '}' } else { '"' };

                let value = self.consume(|c| c.eat_while(|c| c != end));

                // svelte class
                if self.options.file_type == FileType::Svelte && name.starts_with("class:") {
                    self.extend_js_extractor(EcmaExtractor::new(value));
                    return Some(CandidateValue::Plain(name.trim_start_matches("class:")));
                }

                // vue
                if self.options.file_type == FileType::Vue
                    && (name.starts_with(':') || name.starts_with("v-"))
                {
                    self.extend_js_extractor(EcmaExtractor::new(value));
                    return Some(CandidateValue::Ecma);
                }
                return Some(CandidateValue::Plain(value));
            }
            _ => {
                if self.cursor.first() == '>' {
                    self.cursor.bump();
                    self.in_start_tag = false;
                    return None;
                } else {
                    self.cursor.bump();
                    self.consume_whitespace();
                }
            }
        }
        None
    }
}

impl<'a> Iterator for HtmlExtractor<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(js) = self.in_js.as_mut() {
            if let Some(value) = js.next() {
                return Some(value);
            } else {
                self.in_js = None;
            }
        }

        if self.cursor.is_eof() {
            return None;
        }

        if self.in_start_tag() {
            match self.consume_attrs() {
                Some(CandidateValue::Plain(s)) => {
                    let res = Some(s);
                    self.consume(|s| {
                        s.eat_while(|c| c.is_whitespace());
                    });
                    return res;
                }
                Some(CandidateValue::Ecma) => {
                    if let Some(js) = self.in_js.as_mut() {
                        if let Some(value) = js.next() {
                            return Some(value);
                        } else {
                            self.in_js = None;
                            return self.next();
                        }
                    }
                }
                None => {
                    return self.next();
                }
            }
        } else {
            self.consume_until_value();
            return self.next();
        }

        None
    }
}

fn is_name_start_char(c: char) -> bool {
    matches!(
        c,
        'a'..='z' | 'A'..='Z' | ':' | '_' | '\u{C0}'..='\u{D6}' | '\u{D8}'..='\u{F6}' | '\u{F8}'..='\u{2FF}' | '\u{370}'..='\u{37D}' | '\u{37F}'..='\u{1FFF}' | '\u{200C}'..='\u{200D}' | '\u{2070}'..='\u{218F}' | '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}' | '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'..='\u{EFFFF}'
    )
}

fn is_name_char(c: char) -> bool {
    matches!(
        c,
        'a'..='z' | 'A'..='Z' | ':' | '_' | '-' | '.' | '0'..='9' | '\u{B7}' | '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}'
    ) || is_name_start_char(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        let input = r#"
            <script lang="ts">
                console.log('hello');
            </script>
            <div ref="container" :class="containerClass">
                <a href="https://google.com" class="flex" :class="['bg-red-500']" />
            </div>
            <style>
                .a {
                    background-image: url('https://google.com');
                }
            </style>
            "#;
        let extractor = HtmlExtractor::new(input).with_options(HtmlExtractOptions {
            class_only: false,
            file_type: FileType::Vue,
        });

        let expected = [
            "hello",
            "container",
            "https://google.com",
            "flex",
            "bg-red-500",
        ];

        assert_eq!(extractor.collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_svelte() {
        let input = r#"
            <div ref="container" class:foo={ a ? 'red' : 'blue' }>
                <a href="https://google.com" class="flex" />
            </div>
            "#;
        let extractor = HtmlExtractor::new(input).with_options(HtmlExtractOptions {
            class_only: false,
            file_type: FileType::Svelte,
        });

        let expected = [
            "container",
            "foo",
            "red",
            "blue",
            "https://google.com",
            "flex",
        ];

        assert_eq!(extractor.collect::<Vec<_>>(), expected);
    }
}
