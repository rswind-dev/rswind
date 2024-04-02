use fxhash::FxHashMap as HashMap;
use std::{cell::RefCell, sync::Arc};

use crate::css::{DeclList, NodeList};
use crate::rule::Utility;
use crate::theme::Theme;

#[derive(Clone, Default)]
pub struct UtilityStorage<'c> {
    pub utilities: Arc<RefCell<HashMap<String, Vec<Arc<Utility<'c>>>>>>,
    pub theme: Arc<RefCell<Theme<'static>>>,
    pub cache: HashMap<String, Option<NodeList<'c>>>,
}

impl<'c> UtilityStorage<'c> {
    pub fn new() -> Self {
        Self {
            utilities: Arc::new(RefCell::new(HashMap::default())),
            theme: Arc::new(RefCell::new(Theme::default())),
            cache: HashMap::default(),
        }
    }

    pub fn insert(&self, key: String, value: Utility<'c>) {
        self.utilities
            .borrow_mut()
            .entry(key)
            .or_default()
            .push(value.into());
    }

    pub fn get(&self, key: &str) -> Option<Vec<Arc<Utility<'c>>>> {
        self.utilities.borrow().get(key).cloned()
    }

    pub fn try_apply<'a>(
        &mut self,
        key: &str,
        input: &'a str,
    ) -> Option<NodeList<'c>> {
        let k = self.get(key)?;
        self.cache
            .entry(format!("{}{}", key, input))
            .or_insert_with(|| {
                k.into_iter().find_map(|rule| rule.apply_to(input))
            })
            .clone()
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
