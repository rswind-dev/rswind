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

pub trait ToCssString {
    fn to_css_string(self) -> Result<String, Error>;
}

impl<T: ToCss> ToCssString for T {
    fn to_css_string(self) -> Result<String, Error> {
        let mut s = String::new();
        let mut writer = Writer::default(&mut s);
        self.to_css(&mut writer)?;
        Ok(s)
    }
}
