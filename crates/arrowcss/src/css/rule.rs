use std::fmt::Write;

use anyhow::Error;

use crate::writer::Writer;

use super::{CssDecl, ToCss};

#[derive(Debug, Clone)]
pub struct StyleRule<'a> {
    pub selector: String,
    pub nodes: Vec<CssRule<'a>>,
}

#[derive(Debug, Clone)]
pub struct AtRule<'a> {
    pub name: String,
    pub params: String,
    pub nodes: Vec<CssRule<'a>>,
}

#[derive(Debug, Clone)]
pub enum CssRule<'a> {
    Style(StyleRule<'a>),
    AtRule(AtRule<'a>),
    Decl(CssDecl<'a>),
}

impl<'a> ToCss for AtRule<'a> {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writer.write_str("@")?;
        writer.write_str(&self.name)?;
        writer.write_str(&self.params)?;
        writer.write_str(" {")?;
        writer.indent();
        for node in &self.nodes {
            writer.newline()?;
            node.to_css(writer)?;
        }
        writer.dedent();
        writer.write_str("}")?;
        writer.newline()?;
        Ok(())
    }
}

impl<'a> ToCss for CssRule<'a> {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            Self::Style(rule) => rule.to_css(writer),
            Self::AtRule(rule) => rule.to_css(writer),
            Self::Decl(decl) => decl.to_css(writer),
        }
    }
}

impl<'a> ToCss for StyleRule<'a> {
    fn to_css<W: std::fmt::Write>(
        &self,
        writer: &mut Writer<W>,
    ) -> Result<(), Error> {
        writer.write_char('.')?;
        writer.write_str(&self.selector)?;
        writer.whitespace()?;
        writer.write_char('{')?;
        writer.indent();
        for node in &self.nodes {
            writer.newline()?;
            node.to_css(writer)?;
        }
        writer.dedent();
        writer.newline()?;
        writer.write_char('}')?;
        writer.newline()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_css_macro() {
        // let container = css! {
        //     ".class" => {
        //          "color" => "red",
        //          "font-size" => "1rem"
        //     }
        //     ".class2" => {
        //         "color" => "blue"
        //     }
        // };
        // println!("{:?}", container);
    }
}
