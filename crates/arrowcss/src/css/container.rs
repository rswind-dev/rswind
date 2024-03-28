use std::fmt::Write;

use anyhow::Error;
use smallvec::smallvec;
use smallvec::SmallVec;

use crate::css::rule::CssRule;
use crate::writer::Writer;

use super::ToCss;

#[derive(Debug, Default)]
pub struct CssRuleList<'a> {
    pub nodes: SmallVec<[CssRule<'a>; 1]>,
}

impl<'a> Clone for CssRuleList<'a> {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
        }
    }
}

impl<'a> From<CssRule<'a>> for CssRuleList<'a> {
    fn from(rule: CssRule<'a>) -> Self {
        Self {
            nodes: smallvec![rule],
        }
    }
}

impl<'a> FromIterator<CssRule<'a>> for CssRuleList<'a> {
    fn from_iter<T: IntoIterator<Item = CssRule<'a>>>(iter: T) -> Self {
        Self {
            nodes: iter.into_iter().collect(),
        }
    }
}

impl<'a> FromIterator<CssRuleList<'a>> for CssRuleList<'a> {
    fn from_iter<T: IntoIterator<Item = CssRuleList<'a>>>(iter: T) -> Self {
        Self {
            nodes: iter.into_iter().flat_map(|c| c.nodes).collect(),
        }
    }
}

impl<'a> ToCss for CssRuleList<'a> {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        let mut iter = self.nodes.iter();
        let first = iter.next();
        if let Some(first) = first {
            first.to_css(writer)?;
            for node in iter {
                writer.newline()?;
                node.to_css(writer)?;
            }
        }
        Ok(())
    }
}
