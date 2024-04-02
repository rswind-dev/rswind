use std::fmt::Write;

use anyhow::Error;

use crate::css::rule::AstNode;
use crate::writer::Writer;

use super::ToCss;

// #[derive(Debug, Default)]
pub type NodeList<'a> = Vec<AstNode<'a>>;

// impl<'a> Clone for NodeList<'a> {
//     fn clone(&self) -> Self {
//         Self {
//             nodes: self.nodes.clone(),
//         }
//     }
// }

// impl<'a> FromIterator<AstNode<'a>> for NodeList<'a> {
//     fn from_iter<T: IntoIterator<Item = AstNode<'a>>>(iter: T) -> Self {
//         Self {
//             nodes: iter.into_iter().collect(),
//         }
//     }
// }

// impl<'a> FromIterator<NodeList<'a>> for NodeList<'a> {
//     fn from_iter<T: IntoIterator<Item = NodeList<'a>>>(iter: T) -> Self {
//         Self {
//             nodes: iter.into_iter().flat_map(|c| c.nodes).collect(),
//         }
//     }
// }

// impl From<T> for some base type for CssRuleList
// impl<'a> From<DeclList<'a>> for NodeList<'a> {
//     fn from(decls: DeclList<'a>) -> Self {
//         decls.0.into_iter().map(AstNode::Decl).collect()
//     }
// }

// impl<'a> From<Decl<'a>> for NodeList<'a> {
//     fn from(decl: Decl<'a>) -> Self {
//         AstNode::Decl(decl).into()
//     }
// }

// impl<'a, const N: usize> From<[Decl<'a>; N]> for NodeList<'a> {
//     fn from(decls: [Decl<'a>; N]) -> Self {
//         decls.iter().map(|d| AstNode::Decl(d.clone())).collect()
//     }
// }

// impl<'a> From<AstNode<'a>> for NodeList<'a> {
//     fn from(rule: AstNode<'a>) -> Self {
//         Self { nodes: vec![rule] }
//     }
// }

// impl<'a> From<Rule<'a>> for NodeList<'a> {
//     fn from(rule: Rule<'a>) -> Self {
//         AstNode::Rule(rule).into()
//     }
// }

impl<'a> ToCss for NodeList<'a> {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        let mut iter = self.iter();
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
