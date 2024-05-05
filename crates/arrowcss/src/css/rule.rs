use std::{
    fmt::Write,
    ops::{Deref, DerefMut},
};

use smol_str::SmolStr;

use super::{Decl, ToCss};
use crate::writer::Writer;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rule {
    pub selector: SmolStr,
    pub decls: Vec<Decl>,
    pub rules: RuleList,
}

impl Rule {
    pub fn new_with_decls(selector: impl Into<SmolStr>, decls: Vec<Decl>) -> Self {
        Self {
            selector: selector.into(),
            decls,
            rules: RuleList::default(),
        }
    }

    pub fn modify_with(self, modifier: impl Fn(SmolStr) -> SmolStr) -> Self {
        Self {
            selector: modifier(self.selector),
            decls: self.decls,
            rules: self.rules,
        }
    }

    pub fn wrap(self, wrapper: SmolStr) -> Self {
        Self {
            selector: wrapper,
            decls: vec![],
            rules: RuleList::new(self),
        }
    }

    pub fn to_rule_list(self) -> RuleList {
        RuleList::new(self)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RuleList(pub Vec<Rule>);

impl RuleList {
    pub fn new(rule: Rule) -> Self {
        Self(vec![rule])
    }

    pub fn wrap(self, wrapper: SmolStr) -> Rule {
        Rule {
            selector: wrapper,
            decls: vec![],
            rules: self,
        }
    }

    pub fn modify_with(self, modifier: impl Fn(SmolStr) -> SmolStr + Clone) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|r| r.modify_with(modifier.clone()))
                .collect(),
        )
    }

    pub fn as_single(self) -> Option<Rule> {
        self.0.into_iter().next()
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

impl RuleList {
    pub fn extend(&mut self, other: RuleList) {
        self.0.extend(other.0);
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
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error>
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
    T: IntoIterator<Item = &'b Rule>,
{
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error>
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

impl ToCss for &RuleList {
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error>
    where
        W: Write,
    {
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
