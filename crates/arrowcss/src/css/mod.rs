use anyhow::Error;

use crate::writer::Writer;

pub mod container;
pub mod decl;
pub mod rule;

pub use self::container::CssRuleList;
pub use self::decl::CSSDecls;
pub use self::decl::CssDecl;

pub use self::rule::AtRule;
pub use self::rule::CssRule;
pub use self::rule::StyleRule;

pub trait ToCss {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::fmt::Write;
}
