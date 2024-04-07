use anyhow::Error;

use crate::writer::Writer;

pub use self::container::NodeList;
pub use self::decl::Decl;
pub use self::decl::DeclList;
pub use self::rule::AstNode;
pub use self::rule::Rule;

pub mod container;
pub mod decl;
pub mod rule;

pub trait ToCss {
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::fmt::Write;
}
