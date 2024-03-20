use std::{fmt::Write, ops::Deref};

use anyhow::{Error, Ok};
use cssparser::serialize_identifier;
use smallvec::{smallvec, SmallVec};

use crate::writer::Writer;

// dark:text-red -> modifier=[dark],
#[derive(Default)]
pub struct Rule<'a> {
    pub raw: &'a str,
    pub rule: &'a str,
    pub modifier: Vec<&'a str>,
    pub css_cache: Option<String>,
}

pub trait ToCss {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::fmt::Write;
}

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

#[derive(Debug, Clone)]
pub enum CSSRule {
    Style(CSSStyleRule),
    AtRule(CSSAtRule),
    Decl(CSSDecl),
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

#[derive(Debug, Clone)]
pub struct Container {
    pub nodes: SmallVec<[CSSRule; 1]>
}

impl From<CSSRule> for Container {
    fn from(rule: CSSRule) -> Self {
        Self {
            nodes: smallvec![rule],
        }
    }
}

impl FromIterator<CSSRule> for Container {
    fn from_iter<T: IntoIterator<Item = CSSRule>>(iter: T) -> Self {
        Self {
            nodes: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<Container> for Container {
    fn from_iter<T: IntoIterator<Item = Container>>(iter: T) -> Self {
        Self {
            nodes: iter.into_iter().flat_map(|c| c.nodes).collect(),
        }
    }
}

impl ToCss for Container {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        for node in &self.nodes {
            node.to_css(writer)?;
        }
        Ok(())
    }
}

impl Container {
    pub fn new() -> Self {
        Self {
            nodes: smallvec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CSSDecl {
    pub name: String,
    pub value: String,
}

impl CSSDecl {
    pub fn new<S: Into<String>>(name: S, value: S) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl<A: Into<String>, B: Into<String>> From<(A, B)> for CSSDecl {
    fn from(val: (A, B)) -> Self {
        CSSDecl::new(val.0.into(), val.1.into())
    }
}

impl<A: Into<String>, B: Into<String>> FromIterator<(A, B)> for CSSDecls {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CSSDecls(SmallVec<[CSSDecl; 1]>);

impl Deref for CSSDecls {
    type Target = [CSSDecl];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<CSSDecl> for CSSDecls {
    fn from(decl: CSSDecl) -> Self {
        Self(smallvec![decl])
    }
}

impl From<Vec<CSSDecl>> for CSSDecls {
    fn from(decl: Vec<CSSDecl>) -> Self {
        Self(decl.into())
    }
}

impl CSSDecls {
    pub fn new(decl: CSSDecl) -> Self {
        Self(smallvec![decl])
    }

    pub fn multi<D: Into<CSSDecl>, I: IntoIterator<Item = D>>(
        decls: I,
    ) -> Self {
        Self(decls.into_iter().map(Into::into).collect())
    }

    pub fn from_pair<S: Into<String>>(pair: (S, S)) -> Self {
        Self::new(pair.into())
    }
}

impl ToCss for CSSDecl {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writer.write_str(&self.name)?;
        writer.write_str(":")?;
        writer.whitespace()?;
        writer.write_str(&self.value)?;
        writer.write_str(";")?;

        Ok(())
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
