use std::{fmt::Write, ops::Not};

use anyhow::{Error, Ok};
use cssparser::serialize_identifier;

use crate::{context::Context, writer::Writer};

pub trait ToCssRule<'a> {
    fn to_css_rule(&self, ctx: &Context<'a>) -> Option<CSSStyleRule>;
}

// dark:text-red -> modifier=[dark],
#[derive(Default)]
pub struct Rule<'a> {
    pub raw: &'a str,
    pub rule: &'a str,
    pub modifier: Vec<&'a str>,
    pub css_cache: Option<String>,
}



impl<'a> ToCssRule<'a> for Rule<'a> {
    fn to_css_rule(&self, ctx: &Context<'a>) -> Option<CSSStyleRule> {
        // Step 1(todo): split the rules by `:`, get [...modifier, rule]
        // Step 2: try static match
        let mut decls: Vec<CSSRule> = vec![];
        if let Some(static_rule) = ctx.static_rules.get(self.rule) {
            decls = static_rule.to_vec().into_iter().map(CSSRule::Decl).collect();
        } else {
            // Step 3: get all index of `-`
            for (i, _) in self.rule.match_indices('-') {
                let key = self.rule.get(..i).unwrap();
                if let Some(func) = ctx.rules.get(key) {
                    if let Some(v) = func(self.rule.get((i + 1)..).unwrap().to_string()) {
                        decls.append(&mut v.to_vec().into_iter().map(CSSRule::Decl).collect());
                    }
                    break;
                }
            }
        }
        decls.is_empty().not().then(|| CSSStyleRule {
            selector: self.raw.to_string(),
            nodes: decls,
        })
    }
}

pub trait ToCss {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::fmt::Write;
}

#[derive(Debug)]
pub struct CSSStyleRule {
    pub selector: String,
    pub nodes: Vec<CSSRule>,
}

#[derive(Debug)]
pub struct CSSAtRule {
    pub name: String,
    pub params: String,
    pub nodes: Vec<CSSRule>,
}

impl ToCss for CSSAtRule {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writer.write_str("@")?;
        writer.write_str(&self.name)?;
        writer.write_str(" ")?;
        writer.write_str(&self.params)?;
        writer.write_str(" {")?;
        writer.indent();
        for node in &self.nodes {
            writer.newline()?;
            node.to_css(writer)?;
        }
        writer.dedent();
        writer.newline()?;
        writer.write_str("}")?;
        writer.newline()?;
        Ok(())
    }
}


#[derive(Debug)]
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

// one or multiple CSS
#[derive(Clone)]
pub enum CSSDecls {
    One(CSSDecl),
    Multi(Vec<CSSDecl>),
}

impl CSSDecls {
    pub fn one<S: Into<String>>(name: S, value: S) -> Self {
        Self::One(CSSDecl {
            name: name.into(),
            value: value.into(),
        })
    }
    pub fn multi<I: IntoIterator<Item = D>, D: Into<CSSDecl>>(decls: I) -> Self {
        Self::Multi(decls.into_iter().map(|it| it.into()).collect())
    }
    pub fn to_vec(&self) -> Vec<CSSDecl> {
        match self {
            Self::One(decl) => vec![decl.clone()],
            Self::Multi(decls) => decls.clone(),
        }
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
    fn to_css<W: std::fmt::Write>(&self, writer: &mut Writer<W>) -> Result<(), Error> {
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
