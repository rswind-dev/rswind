use either::Either::{self, Left, Right};
use enum_dispatch::enum_dispatch;
use fxhash::FxHashMap as HashMap;

use crate::{
    css::{DeclList, Rule},
    ordering::OrderingKey,
    parsing::UtilityCandidate,
    process::Utility,
};

pub type UtilityValue<'c> = Either<DeclList<'static>, Utility<'c>>;

#[enum_dispatch]
pub trait UtilityStorage<'c>: Sync + Send {
    fn add(&mut self, key: String, value: Utility<'c>);
    fn add_static(&mut self, key: String, value: DeclList<'static>);
    fn get(&self, key: &str) -> Option<&Vec<UtilityValue<'c>>>;
    fn try_apply<'a>(&self, input: UtilityCandidate<'a>) -> Option<(Rule<'c>, OrderingKey)>;
}

#[enum_dispatch(UtilityStorage)]
pub enum UtilityStorageImpl<'c> {
    HashMap(HashMapUtilityStorage<'c>),
}

impl Default for UtilityStorageImpl<'_> {
    fn default() -> Self {
        Self::HashMap(HashMapUtilityStorage::default())
    }
}

#[derive(Default)]
pub struct HashMapUtilityStorage<'c> {
    utilities: HashMap<String, Vec<UtilityValue<'c>>>,
}

impl<'c> UtilityStorage<'c> for HashMapUtilityStorage<'c> {
    fn add(&mut self, key: String, value: Utility<'c>) {
        self.utilities
            .entry(key)
            .or_default()
            .push(Either::Right(value));
    }

    fn add_static(&mut self, key: String, value: DeclList<'static>) {
        self.utilities
            .entry(key)
            .or_default()
            .push(Either::Left(value));
    }

    fn get(&self, key: &str) -> Option<&Vec<UtilityValue<'c>>> {
        self.utilities.get(key)
    }

    fn try_apply<'a>(&self, candidate: UtilityCandidate<'a>) -> Option<(Rule<'c>, OrderingKey)> {
        self.get(candidate.key)?.iter().find_map(|rule| match rule {
            Left(decls) => Some((
                Rule::new_with_decls("&", decls.clone().0.into_vec()),
                OrderingKey::Disorder,
            )),
            Right(handler) => handler.apply_to(candidate),
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_utility_storage() {
        // let mut storage = UtilityStorage::new();
        // storage.insert(
        //     "text".into(),
        //     Rule::new(|_, input| decl!("color".into(), input).into()),
        // );

        // assert_eq!(
        //     storage.try_apply("text", "red"),
        //     Some(decl!("color": "red").into())
        // );
    }
}
