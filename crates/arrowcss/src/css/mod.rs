use anyhow::Error;

use crate::writer::Writer;

pub use self::decl::Decl;
pub use self::decl::DeclList;
pub use self::rule::Rule;

pub mod decl;
pub mod rule;

pub trait ToCss {
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::fmt::Write;
}
