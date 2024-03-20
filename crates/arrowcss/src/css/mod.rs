use anyhow::Error;

use crate::writer::Writer;

pub mod container;
pub mod decl;
pub mod rule;

pub use self::container::Container;
pub use self::decl::CSSDecl;
pub use self::decl::CSSDecls;

pub use self::rule::CSSAtRule;
pub use self::rule::CSSRule;
pub use self::rule::CSSStyleRule;

pub trait ToCss {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: std::fmt::Write;
}
