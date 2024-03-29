use std::{cell::RefCell, collections::HashMap, sync::Arc};

use crate::css::CssDecls;

use crate::rule::{MetaData, Rule};

#[derive(Clone, Default)]
pub struct RuleStorage<'c>(
    pub Arc<RefCell<HashMap<String, Vec<Arc<Rule<'c>>>>>>,
);

impl<'c> RuleStorage<'c> {
    pub fn new() -> Self {
        Self(Arc::new(RefCell::new(HashMap::new())))
    }

    pub fn insert(&self, key: String, value: Rule<'c>) {
        self.0.borrow_mut().entry(key).or_default().push(value.into());
    }

    pub fn get(&self, key: &str) -> Option<Vec<Arc<Rule<'c>>>> {
        self.0.borrow().get(key).cloned()
    }

    pub fn try_apply<'a>(
        &self,
        key: &str,
        input: &'a str,
    ) -> Option<CssDecls<'a>> {
        let meta = MetaData::new(input);
        self.get(key)?
            .into_iter()
            .find_map(|rule| (rule.handler)(meta.clone(), input.into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::css::decl::decl;

    use super::*;

    #[test]
    fn test_rule_storage() {
        let storage = RuleStorage::new();
        let rule = Rule::new(|_, input| {
            Some(decl("color".into(), input).into())
        });
        storage.insert("text".into(), rule);

        assert_eq!(
            storage.try_apply("text", "red"),
            Some(decl("color", "red").into())
        );
    }
}
