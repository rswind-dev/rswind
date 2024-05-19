use std::fmt::Write;

#[allow(clippy::upper_case_acronyms, unused)]
pub enum LineFeed {
    LF,
    CRLF,
}

#[allow(unused)]
pub enum IndentType {
    Space,
    Tab,
}

pub struct WriterConfig {
    pub linefeed: LineFeed,
    pub indent_width: usize,
    pub indent_type: IndentType,
    pub minify: bool,
}

impl Default for WriterConfig {
    fn default() -> Self {
        Self {
            linefeed: LineFeed::LF,
            indent_width: 2,
            indent_type: IndentType::Space,
            minify: false,
        }
    }
}

pub struct Writer<'a, W: Write> {
    pub dest: W,
    pub minify: bool,
    pub line: usize,
    pub col: usize,
    pub linefeed: &'a str,
    pub indent: &'a str,
    pub indent_width: usize,
    pub indent_level: usize,
}

impl<'a, W: Write> Writer<'a, W> {
    pub fn default(dest: W) -> Self {
        Self::new(
            dest,
            WriterConfig {
                minify: false,
                linefeed: LineFeed::LF,
                indent_width: 2,
                indent_type: IndentType::Space,
            },
        )
    }

    pub fn minify(dest: W) -> Self {
        Self::new(dest, WriterConfig { minify: true, ..Default::default() })
    }

    pub fn new(dest: W, config: WriterConfig) -> Self {
        let indent = match config.indent_type {
            IndentType::Tab => "\t",
            IndentType::Space => " ",
        };
        let linefeed = match config.linefeed {
            LineFeed::LF => "\n",
            LineFeed::CRLF => "\r\n",
        };

        Self {
            dest,
            minify: config.minify,
            line: 0,
            col: 0,
            linefeed,
            indent_width: config.indent_width,
            indent,
            indent_level: 0,
        }
    }

    pub fn whitespace(&mut self) -> Result<(), std::fmt::Error> {
        if self.minify {
            return Ok(());
        }
        self.write_str(self.indent)?;
        self.col += 1;

        Ok(())
    }

    pub fn newline(&mut self) -> Result<(), std::fmt::Error> {
        if self.minify {
            return Ok(());
        }

        self.write_str(self.linefeed)?;
        self.line += 1;
        self.col = 0;

        Ok(())
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dedent(&mut self) {
        self.indent_level -= 1;
    }

    fn ensure_ident(&mut self) -> Result<(), std::fmt::Error> {
        if self.minify {
            return Ok(());
        }
        if self.col == 0 && self.indent_level > 0 {
            for _ in 0..(self.indent_level * self.indent_width) {
                self.dest.write_str(self.indent)?;
            }
        }
        Ok(())
    }
}

impl<'a, W: std::fmt::Write + Sized> Write for Writer<'a, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.ensure_ident()?;
        self.col += s.len();
        self.dest.write_str(s)
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.ensure_ident()?;
        self.col += 1;
        self.dest.write_char(c)
    }
}
