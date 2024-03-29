use std::ops::DerefMut;
use std::{fmt::Write, ops::Deref};

use anyhow::Error;
use lightningcss::traits::IntoOwned;
use lightningcss::values::string::CowArcStr;
use smallvec::smallvec;
use smallvec::SmallVec;

use crate::writer::Writer;

use super::{CssRule, ToCss};

#[derive(Clone, Debug, PartialEq)]
pub struct CssDecl<'a> {
    pub name: CowArcStr<'a>,
    pub value: CowArcStr<'a>,
}

impl<'a> CssDecl<'a> {
    pub fn new<S: Into<CowArcStr<'a>>>(name: S, value: S) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl<'a> IntoOwned<'a> for CssDecl<'a> {
    type Owned = CssDecl<'static>;

    fn into_owned(self) -> Self::Owned {
        CssDecl {
            name: self.name.into_owned(),
            value: self.value.into_owned(),
        }
    }
}

pub fn decl<'a, S: Into<CowArcStr<'a>>>(name: S, value: S) -> CssDecl<'a> {
    CssDecl::new(name, value)
}

impl<'a, A: Into<CowArcStr<'a>>, B: Into<CowArcStr<'a>>> From<(A, B)>
    for CssDecl<'a>
{
    fn from(val: (A, B)) -> Self {
        CssDecl::new(val.0.into(), val.1.into())
    }
}

impl<'a, A: Into<CowArcStr<'a>>, B: Into<CowArcStr<'a>>> FromIterator<(A, B)>
    for CssDecls<'a>
{
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CssDecls<'a>(pub SmallVec<[CssDecl<'a>; 1]>);

impl<'a> IntoOwned<'a> for CssDecls<'a> {
    type Owned = CssDecls<'static>;

    fn into_owned(self) -> Self::Owned {
        CssDecls(self.0.into_iter().map(IntoOwned::into_owned).collect())
    }
}

impl<'c> IntoIterator for CssDecls<'c> {
    type Item = CssDecl<'c>;
    type IntoIter = smallvec::IntoIter<[CssDecl<'c>; 1]>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


impl<'a> Into<Vec<CssRule<'a>>> for CssDecls<'a> {
    fn into(self) -> Vec<CssRule<'a>> {
        self.0.into_iter().map(CssRule::Decl).collect()
    }
}

#[macro_export]
macro_rules! decls {
    ($($name:expr => $value:expr),* $(,)?) => {
        // $value ant be Option<&str> or &str, filter out None
        {
            let mut d = $crate::css::CssDecls::new();
            $(

                if let Some(value) = Option::from(lightningcss::values::string::CowArcStr::from($value)) {
                    d.0.push($crate::css::CssDecl::new($name.into(), value));
                }
            )*
            d
        }
    };
}

// impl<'a> From<&&'a str> for OptionOrStr<'a> {
//     fn from(val: &&'a str) -> Self {
//         Self::Str(*val)
//     }
// }

// impl<'a> From<&'a std::string::String> for OptionOrStr<'a> {
//     fn from(val: &'a std::string::String) -> Self {
//         Self::Str(val.as_str())
//     }
// }

impl<'a> Deref for CssDecls<'a> {
    type Target = [CssDecl<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for CssDecls<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<CssDecl<'a>> for CssDecls<'a> {
    fn from(decl: CssDecl<'a>) -> Self {
        Self(smallvec![decl])
    }
}

impl<'a> From<Vec<CssDecl<'a>>> for CssDecls<'a> {
    fn from(decl: Vec<CssDecl<'a>>) -> Self {
        Self(decl.into())
    }
}

impl<'a> FromIterator<CssDecl<'a>> for CssDecls<'a> {
    fn from_iter<T: IntoIterator<Item = CssDecl<'a>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> CssDecls<'a> {
    pub fn new() -> Self {
        Self(smallvec![])
    }

    pub fn multi<D: Into<CssDecl<'a>>, I: IntoIterator<Item = D>>(
        decls: I,
    ) -> Self {
        Self(decls.into_iter().map(Into::into).collect())
    }
}

impl<'a> ToCss for CssDecl<'a> {
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
            CssDecls::multi([
                CssDecl::new("color", "red"),
                CssDecl::new("background-color", "blue"),
            ])
        );
    }
}
