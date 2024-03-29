use std::{cell::RefCell, collections::HashMap, sync::Arc};

use crate::css::CssDecls;
use crate::rule::Rule;
use crate::theme::Theme;

#[derive(Clone, Default)]
pub struct UtilityStorage<'c> {
    pub utilities: Arc<RefCell<HashMap<String, Vec<Arc<Rule<'c>>>>>>,
    pub theme: Arc<RefCell<Theme<'static>>>,
    pub cache: HashMap<String, Option<CssDecls<'c>>>,
}

impl<'c> UtilityStorage<'c> {
    pub fn new() -> Self {
        Self {
            utilities: Arc::new(RefCell::new(HashMap::new())),
            theme: Arc::new(RefCell::new(Theme::default())),
            cache: HashMap::new(),
        }
    }

    pub fn insert(&self, key: String, value: Rule<'c>) {
        self.utilities
            .borrow_mut()
            .entry(key)
            .or_default()
            .push(value.into());
    }

    pub fn get(&self, key: &str) -> Option<Vec<Arc<Rule<'c>>>> {
        self.utilities.borrow().get(key).cloned()
    }

    pub fn try_apply<'a>(
        &mut self,
        key: &str,
        input: &'a str,
    ) -> Option<CssDecls<'c>> {
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
    use crate::css::decl::decl;

    use super::*;

    #[test]
    fn test_utility_storage() {
        let mut storage = UtilityStorage::new();
        storage.insert(
            "text".into(),
            Rule::new(|_, input| Some(decl("color".into(), input).into())),
        );

        assert_eq!(
            storage.try_apply("text", "red"),
            Some(decl("color", "red").into())
        );
    }
}
