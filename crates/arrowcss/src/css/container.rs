use std::fmt::Write;

use anyhow::Error;
use smallvec::smallvec;
use smallvec::SmallVec;

use crate::css::rule::CssRule;
use crate::writer::Writer;

use super::ToCss;

#[derive(Debug, Clone)]
pub struct CssRuleList {
    pub nodes: SmallVec<[CssRule; 1]>,
}

impl From<CssRule> for CssRuleList {
    fn from(rule: CssRule) -> Self {
        Self {
            nodes: smallvec![rule],
        }
    }
}

impl FromIterator<CssRule> for CssRuleList {
    fn from_iter<T: IntoIterator<Item = CssRule>>(iter: T) -> Self {
        Self {
            nodes: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<CssRuleList> for CssRuleList {
    fn from_iter<T: IntoIterator<Item = CssRuleList>>(iter: T) -> Self {
        Self {
            nodes: iter.into_iter().flat_map(|c| c.nodes).collect(),
        }
    }
}

impl ToCss for CssRuleList {
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

impl Default for CssRuleList {
    fn default() -> Self {
        Self::new()
    }
}

impl CssRuleList {
    pub fn new() -> Self {
        Self { nodes: smallvec![] }
    }
}
