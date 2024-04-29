use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{BuildHasher, Hash},
    ops::Deref,
};

pub trait BasicParser {
    fn advance(&mut self, n: usize);
    fn is_eof(&self) -> bool;
    fn next_byte(&self) -> u8;
}

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

    pub fn as_str(&self) -> &str {
        match self {
            MaybeArbitrary::Arbitrary(s) => s,
            MaybeArbitrary::Named(s) => s,
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

impl<K: Hash + Eq, V, S: BuildHasher> MapExtendedExt<(K, V)> for HashMap<K, V, S> {
    fn extended<T: IntoIterator<Item = (K, V)>>(mut self, other: T) -> Self {
        self.extend(other);
        self
    }
}

pub trait Inspector {
    fn dbg(self) -> Self;
    fn also(self, f: impl FnOnce(&Self)) -> Self;
}

impl<T> Inspector for T
where
    T: Debug,
{
    fn dbg(self) -> Self {
        dbg!(&self);
        self
    }

    fn also(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }
}

pub trait ScopeFunctions: Sized {
    fn run_if<B: FnOnce(Self) -> Self + 'static>(self, predictor: bool, block: B) -> Self;
    fn run_unless<B: FnOnce(Self) -> Self + 'static>(self, predictor: bool, block: B) -> Self {
        self.run_if(!predictor, block)
    }
}

impl<T> ScopeFunctions for T {
    fn run_if<B: FnOnce(Self) -> Self + 'static>(self, predictor: bool, block: B) -> Self {
        if predictor {
            block(self)
        } else {
            self
        }
    }
}
