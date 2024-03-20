use std::fmt::Write;

use anyhow::Error;
use smallvec::smallvec;
use smallvec::SmallVec;

use crate::writer::Writer;

use super::CSSRule;
use super::ToCss;

#[derive(Debug, Clone)]
pub struct Container {
    pub nodes: SmallVec<[CSSRule; 1]>,
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
        Self { nodes: smallvec![] }
    }
}
