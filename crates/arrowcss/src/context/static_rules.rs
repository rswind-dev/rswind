use fxhash::FxHashMap as HashMap;

use crate::{
    css::{DeclList, Rule},
    parsing::UtilityCandidate,
};

#[derive(Default, Clone)]
pub struct StaticRuleStorage(pub HashMap<String, DeclList<'static>>);

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

    pub fn try_apply<'a>(
        &self,
        candidate: UtilityCandidate<'a>,
    ) -> Option<Rule<'static>> {
        self.get(candidate.key)
            .map(|decls| Rule::new_with_decls("&", decls.0.into_vec()))
    }
}
