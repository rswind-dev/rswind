use std::ops::{Deref, DerefMut};

use cssparser::match_byte;

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
    in_js: Option<Box<dyn Iterator<Item = &'a str> + Send + Sync + 'a>>,
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

impl<'a> Deref for HtmlExtractor<'a> {
    type Target = Cursor<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl<'a> DerefMut for HtmlExtractor<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cursor
    }
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

    pub fn apply_options<R>(mut self, opt_fn: impl Fn(&mut HtmlExtractOptions) -> R) -> Self {
        opt_fn(&mut self.options);
        self
    }

    pub fn str_from(&self, start: usize) -> &'a str {
        &self.input[start..self.pos()]
    }

    fn consume<R>(&mut self, f: impl FnOnce(&mut Cursor<'a>) -> R) -> &'a str {
        let start = self.pos();
        f(&mut self.cursor);
        self.str_from(start)
    }

    fn consume_tag_name(&mut self) -> &'a str {
        self.consume(|s| s.eat_until(move |c| c.is_ascii_whitespace() || c == '>'))
    }

    fn consume_whitespace(&mut self) {
        self.eat_while(|c| c.is_ascii_whitespace());
    }

    fn in_start_tag(&mut self) -> bool {
        self.in_start_tag
    }

    fn extend_js_extractor(&mut self, iter: impl Iterator<Item = &'a str> + Send + Sync + 'a) {
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
        loop {
            self.eat_until_after_char(b'<');
            if self.first() != '/' && !self.first().is_ascii_whitespace() {
                break;
            }
        }
        self.in_start_tag = true;

        // eat tag name
        match self.consume_tag_name() {
            // use `EcmaExtractor` to extract JS str lit
            "script" => {
                // skip start tag
                // TODO: handle class name here?
                if self.bump() != '>' {
                    self.eat_until_after_char(b'>');
                }

                // read js content
                let js = self.consume(|c| loop {
                    c.eat_until_after_char(b'<');
                    if c.eat_str("/script>") {
                        break;
                    }
                });

                self.in_js = Some(Box::new(EcmaExtractor::new(js)));
                return Some(CandidateValue::Ecma);
            }
            // skip
            "style" => {
                self.eat_until_after_char(b'>');

                loop {
                    self.eat_until_after_char(b'<');
                    if self.eat_str("/style>") {
                        break;
                    }
                }

                return None;
            }
            _ => (),
        }

        None
    }

    fn consume_attrs(&mut self) -> Option<CandidateValue<'a>> {
        self.consume_whitespace();
        if self.is_eof() {
            return None;
        }

        // <div class="foo"> <div class> <div class >
        let name = self.consume(|s| {
            s.eat_while(|c| {
                match_byte! {c,
                    b'=' | b'>' | b'/' | b'\n' | b'\r' | b'\t' | b' ' | b'\x0C' => false,
                    _ => true
                }
            })
        });

        match self.bump() {
            '=' => {
                // jump the `"` or `{` (svelte)
                let start = self.bump();

                // filter out invalid start
                if start != '"' && start != '{' {
                    return None;
                }

                let end = if start == '{' { b'}' } else { b'"' };

                let value = self.consume(|c| c.eat_until_char(end));
                self.bump();

                // TODO: determine these functions at init, prevent runtime check
                if self.options.class_only && !name.starts_with("class") && !name.starts_with(':') {
                    return None;
                }

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
            '>' => {
                self.in_start_tag = false;
            }
            '/' if self.first() == '>' => {
                // self closed tag
                self.bump();
                self.in_start_tag = false;
            }
            _ => {
                self.consume_whitespace();
            }
        }
        None
    }

    fn next_js(&mut self) -> Option<&'a str> {
        if let Some(js) = self.in_js.as_mut() {
            if let Some(value) = js.next() {
                return Some(value);
            } else {
                self.in_js = None;
            }
        }
        None
    }
}

impl<'a> Iterator for HtmlExtractor<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(js_lit) = self.next_js() {
                return Some(js_lit);
            }

            if self.is_eof() {
                return None;
            }

            if self.in_start_tag() {
                if let Some(CandidateValue::Plain(s)) = self.consume_attrs() {
                    return Some(s);
                }
            } else {
                self.consume_until_value();
            }
        }
    }
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
        let extractor = HtmlExtractor::new(input).apply_options(|o| {
            // o.class_only = true;
            o.file_type = FileType::Vue;
        });

        let expected = ["hello", "container", "https://google.com", "flex", "bg-red-500"];

        assert_eq!(extractor.collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_svelte() {
        let input = r#"
            <div ref="container" class:foo={ a ? 'red' : 'blue' }>
                <a href="https://google.com" class="flex" />
            </div>
            "#;
        let extractor = HtmlExtractor::new(input).apply_options(|o| {
            o.file_type = FileType::Svelte;
        });

        let expected = ["container", "foo", "red", "blue", "https://google.com", "flex"];

        assert_eq!(extractor.collect::<Vec<_>>(), expected);
    }
}
