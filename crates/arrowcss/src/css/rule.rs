use std::{
    fmt::Write,
    ops::{Deref, DerefMut},
};

use anyhow::Error;

use super::{Decl, ToCss};
use crate::writer::Writer;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rule<'a> {
    pub selector: String,
    pub decls: Vec<Decl<'a>>,
    pub rules: RuleList<'a>,
}

impl<'a> Rule<'a> {
    pub fn new_with_decls(selector: impl Into<String>, decls: Vec<Decl<'a>>) -> Self {
        Self {
            selector: selector.into(),
            decls,
            rules: RuleList::default(),
        }
    }

    pub fn modify_with(self, modifier: impl Fn(String) -> String) -> Self {
        Self {
            selector: modifier(self.selector),
            decls: self.decls,
            rules: self.rules,
        }
    }

    pub fn wrap(self, wrapper: String) -> Self {
        Self {
            selector: wrapper,
            decls: vec![],
            rules: RuleList::new(self),
        }
    }

    pub fn to_rule_list(self) -> RuleList<'a> {
        RuleList::new(self)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuleList<'a>(pub Vec<Rule<'a>>);

impl<'a> RuleList<'a> {
    pub fn new(rule: Rule<'a>) -> Self {
        Self(vec![rule])
    }

    pub fn wrap(self, wrapper: String) -> Rule<'a> {
        Rule {
            selector: wrapper,
            decls: vec![],
            rules: self,
        }
    }

    pub fn modify_with(self, modifier: impl Fn(String) -> String + Clone) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|r| r.modify_with(modifier.clone()))
                .collect(),
        )
    }

    pub fn as_single(self) -> Option<Rule<'a>> {
        self.0.into_iter().next()
    }
}

impl<'a> IntoIterator for RuleList<'a> {
    type Item = Rule<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> FromIterator<Rule<'a>> for RuleList<'a> {
    fn from_iter<T: IntoIterator<Item = Rule<'a>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> RuleList<'a> {
    pub fn extend(&mut self, other: RuleList<'a>) {
        self.0.extend(other.0);
    }
}

impl<'a> Rule<'a> {
    pub fn is_at_rule(&self) -> bool {
        self.selector.starts_with('@')
    }

    pub fn extend(&mut self, other: Rule<'a>) {
        self.decls.extend(other.decls);
        self.rules.extend(other.rules);
    }
}

impl<'a> ToCss for &Rule<'a> {
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writer.write_str(&self.selector)?;
        writer.whitespace()?;
        writer.write_char('{')?;
        writer.indent();
        writer.newline()?;
        for node in self.decls.iter() {
            node.to_css(writer)?;
        }
        for rule in self.rules.iter() {
            rule.to_css(writer)?;
        }
        writer.dedent();
        writer.write_char('}')?;
        writer.newline()?;
        Ok(())
    }
}

impl<'a, 'b, T> ToCss for T
where
    'a: 'b,
    T: IntoIterator<Item = &'b Rule<'a>>,
{
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        let mut iter = self.into_iter();
        if let Some(first) = iter.next() {
            first.to_css(writer)?;
            for node in iter {
                writer.newline()?;
                node.to_css(writer)?;
            }
        }
        Ok(())
    }
}

impl ToCss for &RuleList<'_> {
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        self.iter().to_css(writer)
    }
}

// region: impl Traits for RuleList

impl<'a> Deref for RuleList<'a> {
    type Target = Vec<Rule<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RuleList<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Rule<'a>> for RuleList<'a> {
    fn from(rule: Rule<'a>) -> Self {
        Self(vec![rule])
    }
}

impl<'a> From<Vec<Rule<'a>>> for RuleList<'a> {
    fn from(rule: Vec<Rule<'a>>) -> Self {
        Self(rule)
    }
}

// endregion

#[cfg(test)]
mod tests {

    #[test]
    fn test_rule_to_css() {
        // let nodes = css!(
        //     "@media (min-width: 768px)" {
        //         "color": "red";
        //         "background-color": "blue";
        //     }
        // );
        // let mut w = String::new();
        // let mut writer = Writer::default(&mut w);
        // nodes.to_css(&mut writer).unwrap();
        // assert_eq!(writer.dest, "@media (min-width: 768px) {\n  color: red;\n  background-color: blue;\n}\n");
    }
}
