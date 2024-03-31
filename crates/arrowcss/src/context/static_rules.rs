use std::{cell::RefCell, sync::Arc};
use hashbrown::HashMap;

use crate::css::CssDecls;

#[derive(Default, Clone)]
pub struct StaticRuleStorage(
    pub Arc<RefCell<HashMap<String, CssDecls<'static>>>>,
);

impl StaticRuleStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, key: String, value: CssDecls<'static>) {
        self.0.borrow_mut().insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<CssDecls<'static>> {
        self.0.borrow().get(key).cloned()
    }
}
