use std::fmt::Write;

use anyhow::Error;
use cssparser::serialize_identifier;

use crate::writer::Writer;

use super::{CSSDecl, Container, ToCss};

#[derive(Debug, Clone)]
pub struct CSSStyleRule {
    pub selector: String,
    pub nodes: Vec<CSSRule>,
}

#[derive(Debug, Clone)]
pub struct CSSAtRule {
    pub name: String,
    pub params: String,
    pub nodes: Vec<Container>,
}

#[derive(Debug, Clone)]
pub enum CSSRule {
    Style(CSSStyleRule),
    AtRule(CSSAtRule),
    Decl(CSSDecl),
}

impl ToCss for CSSAtRule {
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
        // writer.newline()?;
        writer.write_str("}")?;
        writer.newline()?;
        Ok(())
    }
}

impl ToCss for CSSRule {
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

impl ToCss for CSSStyleRule {
    fn to_css<W: std::fmt::Write>(
        &self,
        writer: &mut Writer<W>,
    ) -> Result<(), Error> {
        writer.write_char('.')?;
        serialize_identifier(&self.selector, writer)?;
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
