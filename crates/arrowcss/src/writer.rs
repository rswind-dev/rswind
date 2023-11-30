use anyhow::Error;
use std::fmt::Write;

pub enum LineFeed {
    LF,
    CRLF,
}

pub enum IndentType {
    Space,
    Tab,
}

pub struct WriterConfig {
    pub linefeed: LineFeed,
    pub indent_width: u32,
    pub indent_type: IndentType,
    pub minify: bool,
}

pub struct Writer<'a, W: Write> {
    pub dest: W,
    pub minify: bool,
    pub line: usize,
    pub col: usize,
    pub linefeed: &'a str,
    pub indent: &'a str,
    pub indent_width: u32,
    pub indent_level: usize,
}

impl<'a, W> Writer<'a, W>
where
    W: Write,
{
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

    pub fn whitespace(&mut self) -> Result<(), Error> {
        if self.minify {
            return Ok(());
        }
        self.write_char(' ')?;

        Ok(())
    }

    pub fn newline(&mut self) -> Result<(), Error> {
        if self.minify {
            return Ok(());
        }

        self.write_char('\n')?;
        if self.indent_level > 0 {
            self.write_str(self.indent.repeat(self.indent_level).as_str())?;
        }

        Ok(())
    }

    pub fn indent(&mut self) {
        self.indent_level += 2;
    }

    pub fn dedent(&mut self) {
        self.indent_level -= 2;
    }
}

impl<'a, W: std::fmt::Write + Sized> Write for Writer<'a, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.dest.write_str(s)
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.dest.write_char(c)
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.dest.write_fmt(args)
    }
}
