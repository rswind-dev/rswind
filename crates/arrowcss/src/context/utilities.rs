use fxhash::FxHashMap as HashMap;

use crate::css::NodeList;
use crate::rule::Utility;

#[derive(Default)]
pub struct UtilityStorage<'c> {
    pub utilities: HashMap<String, Vec<Utility<'c>>>,
    // pub theme: Arc<RefCell<Theme<'static>>>,
    // pub cache: HashMap<String, Option<NodeList<'c>>>,
}

impl<'c> UtilityStorage<'c> {
    pub fn new() -> Self {
        Self {
            utilities: HashMap::default(),
            // theme: Arc::new(RefCell::new(Theme::default())),
            // cache: HashMap::default(),
        }
    }

    pub fn insert(&mut self, key: String, value: Utility<'c>) {
        self.utilities.entry(key).or_default().push(value.into());
    }

    pub fn get(&self, key: &str) -> Option<&Vec<Utility<'c>>> {
        self.utilities.get(key)
    }

    pub fn try_apply<'a>(
        &self,
        key: &str,
        input: &'a str,
    ) -> Option<NodeList<'c>> {
        let k = self.get(key)?;
        k.into_iter().find_map(|rule| rule.apply_to(input))
        // self.cache
        //     .entry(format!("{}{}", key, input))
        //     .or_insert_with(|| {
        //     })
        //     .clone()
    }
}

#[cfg(test)]
mod tests {

    use crate::css;

    use super::*;

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
