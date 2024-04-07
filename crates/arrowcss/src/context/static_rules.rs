use fxhash::FxHashMap as HashMap;

use crate::css::DeclList;

#[derive(Default, Clone)]
pub struct StaticRuleStorage(
    pub HashMap<String, DeclList<'static>>,
);

impl StaticRuleStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: String, value: DeclList<'static>) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<DeclList<'static>> {
        self.0.get(key).cloned()
    }
}
