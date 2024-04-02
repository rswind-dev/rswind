use fxhash::FxHashMap as HashMap;
use std::{cell::RefCell, sync::Arc};

use crate::css::DeclList;

#[derive(Default, Clone)]
pub struct StaticRuleStorage(
    pub Arc<RefCell<HashMap<String, DeclList<'static>>>>,
);

impl StaticRuleStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, key: String, value: DeclList<'static>) {
        self.0.borrow_mut().insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<DeclList<'static>> {
        self.0.borrow().get(key).cloned()
    }
}
