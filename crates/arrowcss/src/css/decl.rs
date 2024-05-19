use std::{
    fmt::Write,
    ops::{Deref, DerefMut},
};

use smol_str::SmolStr;

use super::ToCss;
use crate::writer::Writer;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Decl {
    pub name: SmolStr,
    pub value: SmolStr,
}

impl Decl {
    pub fn new(name: impl Into<SmolStr>, value: impl Into<SmolStr>) -> Self {
        Self { name: name.into(), value: value.into() }
    }
}

impl<A: Into<SmolStr>, B: Into<SmolStr>> From<(A, B)> for Decl {
    fn from(val: (A, B)) -> Self {
        Decl::new(val.0.into(), val.1.into())
    }
}

impl<A: Into<SmolStr>, B: Into<SmolStr>> FromIterator<(A, B)> for DeclList {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeclList(pub Vec<Decl>);

impl IntoIterator for DeclList {
    type Item = Decl;
    type IntoIter = std::vec::IntoIter<Decl>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const N: usize> From<[Decl; N]> for DeclList {
    fn from(decls: [Decl; N]) -> Self {
        Self(decls.into_iter().collect())
    }
}

impl Deref for DeclList {
    type Target = [Decl];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DeclList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Decl> for DeclList {
    fn from(decl: Decl) -> Self {
        Self(vec![decl])
    }
}

impl From<Vec<Decl>> for DeclList {
    fn from(decl: Vec<Decl>) -> Self {
        Self(decl)
    }
}

impl FromIterator<Decl> for DeclList {
    fn from_iter<T: IntoIterator<Item = Decl>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl DeclList {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn multi<D: Into<Decl>, I: IntoIterator<Item = D>>(decls: I) -> Self {
        Self(decls.into_iter().map(Into::into).collect())
    }
}

impl ToCss for &Decl {
    fn to_css<W: Write>(self, writer: &mut Writer<W>) -> Result<(), std::fmt::Error> {
        writer.write_str(&self.name)?;
        writer.write_str(":")?;
        writer.whitespace()?;
        writer.write_str(&self.value)?;
        writer.write_str(";")?;
        writer.newline()?;

        Ok(())
    }
}
