use crate::{cursor::Cursor, item::ExtractItem};

/// Extractor for CSS
///
/// dislike other extractors, this css extractor is for @apply only
/// e.g.
/// ```css
/// .foo {
///   color: red;
///   @apply bar baz;
/// }
/// ```
///
/// will be extracted as:
/// [ (bar, .foo), (baz, .foo) ]
///
pub struct CssExtractor<'a> {
    input: &'a str,
    cursor: Cursor<'a>,
    inside_apply: bool,
    inside_selector: Option<&'a str>,
}

impl<'a> CssExtractor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: Cursor::new(input),
            inside_apply: false,
            inside_selector: None,
        }
    }

    pub fn str_from(&self, start: usize) -> &'a str {
        &self.input[start..self.cursor.pos()]
    }

    fn consume(&mut self, f: impl FnOnce(&mut Cursor<'a>)) -> &'a str {
        let start = self.cursor.pos();
        f(&mut self.cursor);
        self.str_from(start)
    }
}

impl<'a> Iterator for CssExtractor<'a> {
    type Item = ExtractItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_eof() {
            return None;
        }
        if let Some(selector) = self.inside_selector {
            if self.inside_apply {
                self.cursor.eat_whitespace();
                match self.cursor.first() {
                    ';' => {
                        self.cursor.bump();
                        self.inside_apply = false;
                        return self.next();
                    }
                    '}' => {
                        self.cursor.bump();
                        self.inside_selector = None;
                        self.inside_apply = false;
                    }
                    _ => {}
                }
                let utility = self.consume(|cursor| {
                    cursor.eat_until(|c| c == ';' || c == '}' || c.is_whitespace());
                });
                return Some(ExtractItem::new(utility.trim(), selector));
            } else {
                // find @apply
                self.cursor.eat_until(|c| c == '@' || c == '}');
                match self.cursor.first() {
                    '@' if self.cursor.eat_str("@apply ") => {
                        self.inside_apply = true;
                        let utility = self.consume(|cursor| {
                            cursor.eat_until(|c| c == ';' || c == '}' || c.is_whitespace());
                        });

                        let selector = self.inside_selector.unwrap();
                        return Some(ExtractItem::new(utility.trim(), selector));
                    }
                    '}' => {
                        self.cursor.bump();
                        self.inside_selector = None;
                        self.inside_apply = false;
                        return self.next();
                    }
                    _ => {}
                }
            }
        } else {
            let selector = self.consume(|cursor| {
                cursor.eat_until(|c| c == '{');
            });
            self.cursor.bump();

            if self.cursor.is_eof() {
                return None;
            }

            self.inside_selector = Some(selector.trim());
            return self.next();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"
    .foo {
        color: red;
        @apply bar baz;
    }
    .bar {
        color: blue;
        @apply bb;
    }
"#;

    #[test]
    fn test_consume() {
        let input = INPUT;
        println!("{:?}", input);
        let e = CssExtractor::new(&input);
        println!("{:#?}", e.collect::<Vec<_>>());
    }
}
