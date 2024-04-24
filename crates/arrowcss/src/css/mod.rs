
pub use self::{
    decl::{Decl, DeclList},
    rule::Rule,
};
use crate::writer::Writer;

pub mod decl;
pub mod rule;

pub trait ToCss {
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error>
    where
        W: std::fmt::Write;
}

pub trait ToCssString {
    fn to_css_string(self) -> Result<String, std::fmt::Error>;
}

impl<T: ToCss> ToCssString for T {
    fn to_css_string(self) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        let mut writer = Writer::default(&mut s);
        self.to_css(&mut writer)?;
        Ok(s)
    }
}
