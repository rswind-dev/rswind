use std::ops::DerefMut;
use std::{fmt::Write, ops::Deref};

use anyhow::Error;
use lightningcss::traits::IntoOwned;
use lightningcss::values::string::CowArcStr;
use smallvec::smallvec;
use smallvec::SmallVec;

use crate::writer::Writer;

use super::{AstNode, ToCss};

#[derive(Clone, Debug, PartialEq)]
pub struct Decl<'a> {
    pub name: CowArcStr<'a>,
    pub value: CowArcStr<'a>,
}

impl<'a> Decl<'a> {
    pub fn new<S: Into<CowArcStr<'a>>, SS: Into<CowArcStr<'a>>>(
        name: S,
        value: SS,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl<'a> IntoOwned<'a> for Decl<'a> {
    type Owned = Decl<'static>;

    fn into_owned(self) -> Self::Owned {
        Decl {
            name: self.name.into_owned(),
            value: self.value.into_owned(),
        }
    }
}

// pub fn decl<'a, S: Into<CowArcStr<'a>>>(name: S, value: S) -> CssDecl<'a> {
//     CssDecl::new(name, value)
// }

impl<'a, A: Into<CowArcStr<'a>>, B: Into<CowArcStr<'a>>> From<(A, B)>
    for Decl<'a>
{
    fn from(val: (A, B)) -> Self {
        Decl::new(val.0.into(), val.1.into())
    }
}

impl<'a, A: Into<CowArcStr<'a>>, B: Into<CowArcStr<'a>>> FromIterator<(A, B)>
    for DeclList<'a>
{
    fn from_iter<T: IntoIterator<Item = (A, B)>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeclList<'a>(pub SmallVec<[Decl<'a>; 1]>);

impl<'a> IntoOwned<'a> for DeclList<'a> {
    type Owned = DeclList<'static>;

    fn into_owned(self) -> Self::Owned {
        DeclList(self.0.into_iter().map(IntoOwned::into_owned).collect())
    }
}

impl<'c> IntoIterator for DeclList<'c> {
    type Item = Decl<'c>;
    type IntoIter = smallvec::IntoIter<[Decl<'c>; 1]>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> Into<Vec<AstNode<'a>>> for DeclList<'a> {
    fn into(self) -> Vec<AstNode<'a>> {
        self.0.into_iter().map(AstNode::Decl).collect()
    }
}

impl<'a, const N: usize> From<[Decl<'a>; N]> for DeclList<'a> {
    fn from(decls: [Decl<'a>; N]) -> Self {
        Self(decls.into_iter().collect())
    }
}

impl<'a> Deref for DeclList<'a> {
    type Target = [Decl<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for DeclList<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Decl<'a>> for DeclList<'a> {
    fn from(decl: Decl<'a>) -> Self {
        Self(smallvec![decl])
    }
}

impl<'a> From<Vec<Decl<'a>>> for DeclList<'a> {
    fn from(decl: Vec<Decl<'a>>) -> Self {
        Self(decl.into())
    }
}

impl<'a> FromIterator<Decl<'a>> for DeclList<'a> {
    fn from_iter<T: IntoIterator<Item = Decl<'a>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> DeclList<'a> {
    pub fn new() -> Self {
        Self(smallvec![])
    }

    pub fn multi<D: Into<Decl<'a>>, I: IntoIterator<Item = D>>(
        decls: I,
    ) -> Self {
        Self(decls.into_iter().map(Into::into).collect())
    }
}

impl<'a> ToCss for Decl<'a> {
    fn to_css<W>(&self, writer: &mut Writer<W>) -> Result<(), Error>
    where
        W: Write,
    {
        writer.write_str(&self.name)?;
        writer.write_str(":")?;
        writer.whitespace()?;
        writer.write_str(&self.value)?;
        writer.write_str(";")?;
        writer.newline()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::css::NodeList;

    use super::*;
    use arrowcss_css_macro::css;
    use smallvec::smallvec;

    #[test]
    fn test_css_decl_macro() {
        let decls: NodeList = css! {
            "color": "red";
            // "background-color": "blue";
        };

        assert_eq!(
            decls,
            vec![
                AstNode::Decl(Decl::new("color", "red")),
                // AstNode::Decl(Decl::new("background-color", "blue")),
            ]
        );
    }
}
