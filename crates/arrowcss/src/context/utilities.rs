use enum_dispatch::enum_dispatch;
use fxhash::FxHashMap as HashMap;

use crate::css::Rule;
use crate::parsing::UtilityCandidate;
use crate::process::UtilityProcessor;

#[enum_dispatch]
pub trait UtilityStorage<'c>: Sync + Send {
    fn insert(&mut self, key: String, value: UtilityProcessor<'c>);
    fn get(&self, key: &str) -> Option<&Vec<UtilityProcessor<'c>>>;
    fn try_apply<'a>(&self, input: UtilityCandidate<'a>) -> Option<Rule<'c>>;
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
    utilities: HashMap<String, Vec<UtilityProcessor<'c>>>,
}

impl<'c> UtilityStorage<'c> for HashMapUtilityStorage<'c> {
    fn insert(&mut self, key: String, value: UtilityProcessor<'c>) {
        self.utilities.entry(key).or_default().push(value);
    }

    fn get(&self, key: &str) -> Option<&Vec<UtilityProcessor<'c>>> {
        self.utilities.get(key)
    }

    fn try_apply<'a>(
        &self,
        candidate: UtilityCandidate<'a>,
    ) -> Option<Rule<'c>> {
        self.get(candidate.key)?
            .iter()
            .find_map(|rule| rule.apply_to(candidate))
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
