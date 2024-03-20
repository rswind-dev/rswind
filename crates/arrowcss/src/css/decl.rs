use std::{fmt::Write, ops::Deref};

use anyhow::Error;
use smallvec::smallvec;
use smallvec::SmallVec;

use crate::writer::Writer;

use super::ToCss;

#[derive(Debug, Clone, PartialEq)]
pub struct CSSDecl {
    pub name: String,
    pub value: String,
}

impl CSSDecl {
    pub fn new<S: Into<String>>(name: S, value: S) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl<A: Into<String>, B: Into<String>> From<(A, B)> for CSSDecl {
    fn from(val: (A, B)) -> Self {
        CSSDecl::new(val.0.into(), val.1.into())
    }
}

impl<A: Into<String>, B: Into<String>> FromIterator<(A, B)> for CSSDecls {
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CSSDecls(SmallVec<[CSSDecl; 1]>);

impl Deref for CSSDecls {
    type Target = [CSSDecl];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<CSSDecl> for CSSDecls {
    fn from(decl: CSSDecl) -> Self {
        Self(smallvec![decl])
    }
}

impl From<Vec<CSSDecl>> for CSSDecls {
    fn from(decl: Vec<CSSDecl>) -> Self {
        Self(decl.into())
    }
}

impl CSSDecls {
    pub fn new(decl: CSSDecl) -> Self {
        Self(smallvec![decl])
    }

    pub fn multi<D: Into<CSSDecl>, I: IntoIterator<Item = D>>(
        decls: I,
    ) -> Self {
        Self(decls.into_iter().map(Into::into).collect())
    }

    pub fn from_pair<S: Into<String>>(pair: (S, S)) -> Self {
        Self::new(pair.into())
    }
}

impl ToCss for CSSDecl {
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
