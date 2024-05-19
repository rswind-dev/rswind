use std::{
    fmt::Write,
    ops::{Deref, DerefMut},
};

use smol_str::SmolStr;

use super::{Decl, ToCss};
use crate::writer::Writer;

#[derive(Debug, Clone, PartialEq, Default, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Rule {
    pub selector: SmolStr,
    pub decls: Vec<Decl>,
    pub rules: RuleList,
}

impl Rule {
    pub fn new_empty(decls: impl IntoIterator<Item = Decl>) -> Self {
        Self {
            selector: "&".into(),
            decls: decls.into_iter().collect(),
            rules: RuleList::default(),
        }
    }

    pub fn new_with_decls(selector: impl Into<SmolStr>, decls: Vec<Decl>) -> Self {
        Self { selector: selector.into(), decls, rules: RuleList::default() }
    }

    pub fn new_with_rules(selector: impl Into<SmolStr>, rules: RuleList) -> Self {
        Self { selector: selector.into(), decls: Default::default(), rules }
    }

    pub fn modify_with<T: Into<SmolStr>>(mut self, modifier: impl Fn(&str) -> T) -> Self {
        self.selector = modifier(&self.selector).into();
        self
    }

    pub fn modify_mut_with<T: Into<SmolStr>>(&mut self, modifier: impl Fn(&str) -> T) -> &mut Self {
        self.selector = modifier(&self.selector).into();
        self
    }

    pub fn wrap(self, wrapper: SmolStr) -> Self {
        Self { selector: wrapper, decls: vec![], rules: RuleList::new(self) }
    }

    pub fn to_rule_list(self) -> RuleList {
        RuleList::new(self)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleList(pub Vec<Rule>);

impl RuleList {
    pub fn new(rule: Rule) -> Self {
        Self(vec![rule])
    }

    pub fn wrap(self, wrapper: SmolStr) -> Rule {
        Rule { selector: wrapper, decls: vec![], rules: self }
    }

    pub fn modify_with<T: Into<SmolStr>>(mut self, modifier: impl Fn(&str) -> T + Clone) -> Self {
        self.0.iter_mut().for_each(|r| {
            r.modify_mut_with(&modifier);
        });
        self
    }

    pub fn as_single(self) -> Option<Rule> {
        self.0.into_iter().next()
    }

    pub fn from_list<'a>(i: impl IntoIterator<Item = &'a Self>) -> Self {
        Self(i.into_iter().flat_map(|r| &r.0).cloned().collect())
    }
}

impl IntoIterator for RuleList {
    type Item = Rule;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Rule> for RuleList {
    fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a Rule> for &'a RuleList {
    fn from_iter<T: IntoIterator<Item = &'a Rule>>(iter: T) -> Self {
        iter.into_iter().collect()
    }
}

impl RuleList {
    pub fn extend(&mut self, other: RuleList) {
        self.0.extend(other.0);
    }

    pub fn extended(mut self, other: RuleList) -> Self {
        self.0.extend(other.0);
        self
    }
}

impl Rule {
    pub fn is_at_rule(&self) -> bool {
        self.selector.starts_with('@')
    }

    pub fn extend(&mut self, other: Rule) {
        self.decls.extend(other.decls);
        self.rules.extend(other.rules);
    }
}

impl ToCss for &Rule {
    fn to_css<W: Write>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error> {
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

impl<'a, T: IntoIterator<Item = &'a Rule>> ToCss for T {
    fn to_css<W: Write>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error> {
        let mut iter = self.into_iter();
        if let Some(first) = iter.next() {
            first.to_css(writer)?;
            for node in iter {
                node.to_css(writer)?;
            }
        }
        Ok(())
    }
}

impl ToCss for &RuleList {
    fn to_css<W: Write>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error> {
        self.iter().to_css(writer)
    }
}

// region: impl Traits for RuleList

impl Deref for RuleList {
    type Target = Vec<Rule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RuleList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Rule> for RuleList {
    fn from(rule: Rule) -> Self {
        Self(vec![rule])
    }
}

impl From<Vec<Rule>> for RuleList {
    fn from(rule: Vec<Rule>) -> Self {
        Self(rule)
    }
}

impl<const N: usize> From<[Rule; N]> for RuleList {
    fn from(s: [Rule; N]) -> RuleList {
        RuleList(s.into())
    }
}

// endregion
