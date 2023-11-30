use std::{fmt::Write, ops::Not};

use anyhow::{Error, Ok};
use cssparser::serialize_identifier;

use crate::{context::Context, writer::Writer};

pub trait ToCssRule<'a> {
    fn to_css_rule(&self, ctx: &Context<'a>) -> Option<CSSStyleRule>;
}

// dark:text-red -> modifier=[dark],
pub struct Rule<'a> {
    pub raw: &'a str,
    pub rule: &'a str,
    pub modifier: Vec<&'a str>,
    pub css_cache: Option<String>,
}

impl Default for Rule<'_> {
    fn default() -> Self {
        Self {
            raw: "",
            rule: "",
            modifier: vec![],
            css_cache: None,
        }
    }
}

impl<'a> ToCssRule<'a> for Rule<'a> {
    fn to_css_rule(&self, ctx: &Context<'a>) -> Option<CSSStyleRule> {
        // Step 1(todo): split the rules by `:`, get [...modifier, rule]
        // Step 2: try static match
        let mut decls: Vec<CSSDecl> = vec![];
        if let Some(static_rule) = ctx.static_rules.get(self.rule) {
            decls = static_rule.to_vec();
        } else {
            // Step 3: get all index of `-`
            for (i, _) in self.rule.match_indices("-") {
                let key = self.rule.get(..i).unwrap();
                if let Some(func) = ctx.rules.get(key) {
                    if let Some(v) = func(self.rule.get((i + 1)..).unwrap()) {
                        decls.append(&mut v.to_vec());
                    }
                    break;
                }
            }
        }
        decls.is_empty().not().then(|| CSSStyleRule {
            selector: format!("{}", self.raw),
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
    pub nodes: Vec<CSSDecl>,
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

impl<A: Into<String>, B: Into<String>> Into<CSSDecl> for (A, B) {
    fn into(self) -> CSSDecl {
        CSSDecl::new(self.0.into(), self.1.into())
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
