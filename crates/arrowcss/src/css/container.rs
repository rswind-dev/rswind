use std::fmt::Write;

use anyhow::Error;

use crate::css::rule::AstNode;
use crate::writer::Writer;

use super::ToCss;

// #[derive(Debug, Default)]
pub type NodeList<'a> = Vec<AstNode<'a>>;

impl<'a, 'b, T> ToCss for T
where
    'a: 'b,
    T: IntoIterator<Item = &'b AstNode<'a>>,
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
