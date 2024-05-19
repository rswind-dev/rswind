use either::Either::{self, Left, Right};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;

use crate::{
    config::StaticUtilityConfig,
    css::{DeclList, Rule},
    ordering::OrderingKey,
    parsing::UtilityCandidate,
    process::{Utility, UtilityApplyResult},
};

#[derive(Debug)]
pub struct StaticUtility {
    selector: Option<SmolStr>,
    decls: DeclList,
}

impl StaticUtility {
    pub fn new(selector: SmolStr, decls: DeclList) -> Self {
        Self {
            selector: Some(selector),
            decls,
        }
    }
}

impl From<DeclList> for StaticUtility {
    fn from(value: DeclList) -> Self {
        Self {
            selector: None,
            decls: value,
        }
    }
}

impl From<(SmolStr, DeclList)> for StaticUtility {
    fn from((selector, decl_list): (SmolStr, DeclList)) -> Self {
        Self {
            selector: Some(selector),
            decls: decl_list,
        }
    }
}

impl From<StaticUtilityConfig> for StaticUtility {
    fn from(value: StaticUtilityConfig) -> Self {
        match value {
            StaticUtilityConfig::DeclList(decl_list) => Self {
                selector: None,
                decls: decl_list.into_iter().collect(),
            },
            StaticUtilityConfig::WithSelector(value) => Self {
                selector: Some(value.0),
                decls: value.1.into_iter().collect(),
            },
        }
    }
}

pub type UtilityValue = Either<StaticUtility, Utility>;

#[derive(Default)]
pub struct UtilityStorage {
    utilities: HashMap<SmolStr, Vec<UtilityValue>>,
}

impl UtilityStorage {
    pub fn add(&mut self, key: SmolStr, value: Utility) {
        self.utilities
            .entry(key)
            .or_default()
            .push(Either::Right(value));
    }

    pub fn reserve(&mut self, additional: usize) {
        self.utilities.reserve(additional);
    }

    pub fn add_static(&mut self, key: SmolStr, value: StaticUtility) {
        self.utilities
            .entry(key)
            .or_default()
            .push(Either::Left(value));
    }

    pub fn get(&self, key: &str) -> Option<&Vec<UtilityValue>> {
        self.utilities.get(key)
    }

    pub fn len(&self) -> usize {
        self.utilities.len()
    }

    pub fn try_apply(&self, candidate: UtilityCandidate<'_>) -> Option<UtilityApplyResult> {
        self.get(candidate.key)?.iter().find_map(|rule| match rule {
            Left(value) => Some(UtilityApplyResult {
                rule: Rule::new_with_decls(
                    value.selector.as_ref().map(|s| s.as_str()).unwrap_or("&"),
                    value.decls.0.clone(),
                ),
                ordering: OrderingKey::Disorder,
                group: None,
                additional_css: None,
            }),
            Right(handler) => handler.apply_to(candidate),
        })
    }
}
