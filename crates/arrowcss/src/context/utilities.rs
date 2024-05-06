use either::Either::{self, Left, Right};
use enum_dispatch::enum_dispatch;
use fxhash::FxHashMap as HashMap;
use smol_str::SmolStr;

use crate::{
    css::{DeclList, Rule},
    ordering::OrderingKey,
    parsing::UtilityCandidate,
    process::{Utility, UtilityGroup},
};

pub type UtilityValue = Either<DeclList, Utility>;

#[enum_dispatch]
pub trait UtilityStorage: Sync + Send {
    fn add(&mut self, key: SmolStr, value: Utility);
    fn reserve(&mut self, additional: usize);
    fn add_static(&mut self, key: SmolStr, value: DeclList);
    fn get(&self, key: &str) -> Option<&Vec<UtilityValue>>;
    fn try_apply(
        &self,
        input: UtilityCandidate<'_>,
    ) -> Option<(Rule, OrderingKey, Option<UtilityGroup>)>;
}

#[enum_dispatch(UtilityStorage)]
pub enum UtilityStorageImpl {
    HashMap(HashMapUtilityStorage),
}

impl Default for UtilityStorageImpl {
    fn default() -> Self {
        Self::HashMap(HashMapUtilityStorage::default())
    }
}

#[derive(Default)]
pub struct HashMapUtilityStorage {
    utilities: HashMap<SmolStr, Vec<UtilityValue>>,
}

impl UtilityStorage for HashMapUtilityStorage {
    fn add(&mut self, key: SmolStr, value: Utility) {
        self.utilities
            .entry(key)
            .or_default()
            .push(Either::Right(value));
    }

    fn reserve(&mut self, additional: usize) {
        self.utilities.reserve(additional);
    }

    fn add_static(&mut self, key: SmolStr, value: DeclList) {
        self.utilities
            .entry(key)
            .or_default()
            .push(Either::Left(value));
    }

    fn get(&self, key: &str) -> Option<&Vec<UtilityValue>> {
        self.utilities.get(key)
    }

    fn try_apply(
        &self,
        candidate: UtilityCandidate<'_>,
    ) -> Option<(Rule, OrderingKey, Option<UtilityGroup>)> {
        self.get(candidate.key)?.iter().find_map(|rule| match rule {
            Left(decls) => Some((
                Rule::new_with_decls("&", decls.clone().0.into_vec()),
                OrderingKey::Disorder,
                None,
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
