pub use self::{
    decl::{Decl, DeclList},
    rule::Rule,
};
use crate::writer::Writer;

pub mod de;
pub mod decl;
pub mod rule;
pub mod writer;

pub trait ToCss {
    fn to_css<W>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error>
    where
        W: std::fmt::Write;
}

pub trait ToCssString {
    fn to_css_string(self) -> String;
    fn to_css_minified(self) -> String;
}

impl<T: ToCss> ToCssString for T {
    fn to_css_string(self) -> String {
        let mut s = String::new();
        let mut writer = Writer::new(&mut s);
        let _ = self.to_css(&mut writer);
        s
    }

    fn to_css_minified(self) -> String {
        let mut s = String::new();
        let mut writer = Writer::minify(&mut s);
        let _ = self.to_css(&mut writer);
        s
    }
}
