use std::str::FromStr;

use either::Either::{self, Left, Right};
use rswind_css::{Decl, DeclList, Rule};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;

use crate::{
    config::StaticUtilityValue,
    ordering::OrderingKey,
    parse::UtilityCandidate,
    process::{Utility, UtilityApplyResult},
};

#[derive(Debug)]
pub struct StaticUtility {
    pub selector: Option<SmolStr>,
    pub decls: DeclList,
}

impl StaticUtility {
    pub fn new(selector: SmolStr, decls: DeclList) -> Self {
        Self { selector: Some(selector), decls }
    }
}

impl From<DeclList> for StaticUtility {
    fn from(value: DeclList) -> Self {
        Self { selector: None, decls: value }
    }
}

impl From<(SmolStr, DeclList)> for StaticUtility {
    fn from((selector, decl_list): (SmolStr, DeclList)) -> Self {
        Self { selector: Some(selector), decls: decl_list }
    }
}

impl From<StaticUtilityValue> for StaticUtility {
    fn from(value: StaticUtilityValue) -> Self {
        match value {
            StaticUtilityValue::DeclList(decl_list) => Self { selector: None, decls: decl_list },
            StaticUtilityValue::WithSelector(value) => {
                Self { selector: Some(value.0), decls: value.1 }
            }
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
        self.utilities.entry(key).or_default().push(Either::Right(value));
    }

    pub fn reserve(&mut self, additional: usize) {
        self.utilities.reserve(additional);
    }

    pub fn add_static(&mut self, key: SmolStr, value: StaticUtility) {
        self.utilities.entry(key).or_default().push(Either::Left(value));
    }

    pub fn get(&self, key: &str) -> Option<&Vec<UtilityValue>> {
        self.utilities.get(key)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.utilities.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SmolStr, &Vec<UtilityValue>)> {
        self.utilities.iter()
    }

    pub fn try_apply(&self, candidate: UtilityCandidate<'_>) -> Option<UtilityApplyResult> {
        if candidate.arbitrary {
            return Some(UtilityApplyResult {
                rule: Rule::new([Decl::new(
                    candidate.key,
                    candidate.value.unwrap_or_default().as_str(),
                )]),
                ordering: OrderingKey::from_str(candidate.key).unwrap_or(OrderingKey::Disorder),
                group: None,
                extra_css: None,
            });
        }
        self.get(candidate.key)?.iter().find_map(|rule| match rule {
            Left(value) => Some(UtilityApplyResult {
                rule: Rule::new_with_decls(
                    value.selector.as_deref().unwrap_or("&"),
                    value.decls.0.clone(),
                ),
                ordering: OrderingKey::Disorder,
                group: None,
                extra_css: None,
            }),
            Right(handler) => handler.apply_to(candidate),
        })
    }
}

impl Extend<(SmolStr, Utility)> for UtilityStorage {
    fn extend<T: IntoIterator<Item = (SmolStr, Utility)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.add(key, value);
        }
    }
}

impl Extend<(SmolStr, StaticUtilityValue)> for UtilityStorage {
    fn extend<T: IntoIterator<Item = (SmolStr, StaticUtilityValue)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.add_static(key, value.into());
        }
    }
}
