use std::ops::DerefMut;
use std::{fmt::Write, ops::Deref};

use anyhow::Error;
use smallvec::smallvec;
use smallvec::SmallVec;

use crate::writer::Writer;

use super::ToCss;

#[derive(Debug, Clone, PartialEq)]
pub struct CssDecl {
    pub name: String,
    pub value: String,
}

impl CssDecl {
    pub fn new<S: Into<String>>(name: S, value: S) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl<A: Into<String>, B: Into<String>> From<(A, B)> for CssDecl {
    fn from(val: (A, B)) -> Self {
        CssDecl::new(val.0.into(), val.1.into())
    }
}

impl<A: Into<String>, B: Into<String>> FromIterator<(A, B)> for CSSDecls {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CSSDecls(pub SmallVec<[CssDecl; 1]>);

pub enum OptionOrStr<'a> {
    Option(Option<String>),
    Str(&'a str),
}

impl<'a> From<Option<&'a str>> for OptionOrStr<'a> {
    fn from(val: Option<&'a str>) -> Self {
        Self::Option(val.map(Into::into))
    }
}

impl<'a> From<&'a str> for OptionOrStr<'a> {
    fn from(val: &'a str) -> Self {
        Self::Str(val)
    }
}

impl<'a> From<Option<String>> for OptionOrStr<'a> {
    fn from(val: Option<String>) -> Self {
        Self::Option(val)
    }
}

impl<'a> From<OptionOrStr<'a>> for Option<String> {
    fn from(value: OptionOrStr<'a>) -> Self {
        match value {
            OptionOrStr::Option(Some(s)) => Some(s),
            OptionOrStr::Option(None) => None,
            OptionOrStr::Str(s) => Some(s.to_string()),
        }
    }
}

#[macro_export]
macro_rules! decls {
    ($($name:expr => $value:expr),* $(,)?) => {
        // $value ant be Option<&str> or &str, filter out None
        {
            let mut d = $crate::css::CSSDecls::new();
            $(

                if let Some(value) = Option::<String>::from($crate::css::decl::OptionOrStr::from($value)) {
                    d.0.push($crate::css::CssDecl::new($name, &value));
                }
            )*
            d
        }
    };
}

impl<'a> From<&&'a str> for OptionOrStr<'a> {
    fn from(val: &&'a str) -> Self {
        Self::Str(*val)
    }
}

impl<'a> From<&'a std::string::String> for OptionOrStr<'a> {
    fn from(val: &'a std::string::String) -> Self {
        Self::Str(val.as_str())
    }
}

impl Deref for CSSDecls {
    type Target = [CssDecl];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CSSDecls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<CssDecl> for CSSDecls {
    fn from(decl: CssDecl) -> Self {
        Self(smallvec![decl])
    }
}

impl From<Vec<CssDecl>> for CSSDecls {
    fn from(decl: Vec<CssDecl>) -> Self {
        Self(decl.into())
    }
}

impl CSSDecls {
    pub fn new() -> Self {
        Self(smallvec![])
    }

    pub fn multi<D: Into<CssDecl>, I: IntoIterator<Item = D>>(
        decls: I,
    ) -> Self {
        Self(decls.into_iter().map(Into::into).collect())
    }

    pub fn from_pair<S: Into<String>>(pair: (S, S)) -> Self {
        Self::from(Into::<CssDecl>::into(pair))
    }
}

impl ToCss for CssDecl {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writer.write_str(&self.name)?;
        writer.write_str(":")?;
        writer.whitespace()?;
        writer.write_str(&self.value)?;
        writer.write_str(";")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_decl_macro() {
        let decls = decls! {
            "color" => "red",
            "background-color" => "blue",
        };

        assert_eq!(
            decls,
            CSSDecls::multi([
                CssDecl::new("color", "red"),
                CssDecl::new("background-color", "blue"),
            ])
        );
    }
}
