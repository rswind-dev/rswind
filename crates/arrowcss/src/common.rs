use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
    ops::Deref,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MaybeArbitrary<'a> {
    Arbitrary(&'a str),
    Named(&'a str),
}

impl MaybeArbitrary<'_> {
    pub fn take_arbitrary(&self) -> Option<&str> {
        match self {
            MaybeArbitrary::Arbitrary(s) => Some(s),
            _ => None,
        }
    }

    pub fn take_named(&self) -> Option<&str> {
        match self {
            MaybeArbitrary::Named(s) => Some(s),
            _ => None,
        }
    }
}

impl Default for MaybeArbitrary<'_> {
    fn default() -> Self {
        MaybeArbitrary::Named("")
    }
}

impl<'a> Deref for MaybeArbitrary<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeArbitrary::Arbitrary(s) => s,
            MaybeArbitrary::Named(s) => s,
        }
    }
}

pub trait MapExtendedExt<A> {
    fn extended<T: IntoIterator<Item = A>>(self, other: T) -> Self;
}

impl<K: Hash + Eq, V, S: BuildHasher> MapExtendedExt<(K, V)>
    for HashMap<K, V, S>
{
    fn extended<T: IntoIterator<Item = (K, V)>>(mut self, other: T) -> Self {
        self.extend(other);
        self
    }
}
