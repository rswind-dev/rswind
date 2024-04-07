use fxhash::FxHashMap as HashMap;

use crate::css::NodeList;
use crate::rule::UtilityProcessor;
use crate::utility::UtilityCandidate;

pub trait UtilityStorage<'c>: Sync + Send {
    fn insert(&mut self, key: String, value: UtilityProcessor<'c>);
    fn get(&self, key: &str) -> Option<&Vec<UtilityProcessor<'c>>>;
    fn try_apply<'a>(
        &self,
        input: UtilityCandidate<'a>,
    ) -> Option<NodeList<'c>>;
}

#[derive(Default)]
pub struct HashMapUtilityStorage<'c> {
    pub utilities: HashMap<String, Vec<UtilityProcessor<'c>>>,
    // pub theme: Arc<RefCell<Theme<'static>>>,
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
    ) -> Option<NodeList<'c>> {
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
