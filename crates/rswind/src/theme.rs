use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::css::rule::RuleList;
use phf::{phf_map, Map};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;

#[derive(Clone, Debug)]
pub enum ThemeValue {
    Dynamic(HashMap<SmolStr, SmolStr>),
    Static(&'static Map<&'static str, &'static str>),
    RuleList(HashMap<SmolStr, RuleList>),
}

impl Default for ThemeValue {
    fn default() -> Self {
        Self::Static(&phf_map! {})
    }
}

impl ThemeValue {
    pub fn get(&self, key: &str) -> Option<SmolStr> {
        match self {
            Self::Static(map) => map.get(key).map(|s| SmolStr::from(*s)),
            Self::Dynamic(map) => map.get(key).cloned(),
            Self::RuleList(_) => None,
        }
    }

    pub fn get_rule_list(&self, key: &str) -> Option<&RuleList> {
        match self {
            Self::RuleList(map) => map.get(key),
            _ => None,
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&str, SmolStr)> + 'a> {
        match self {
            Self::Static(map) => Box::new(map.into_iter().map(|(k, v)| (*k, SmolStr::from(*v)))),
            Self::Dynamic(map) => Box::new(map.iter().map(|(k, v)| (k.as_str(), v.clone()))),
            Self::RuleList(_) => Box::new(std::iter::empty()),
        }
    }
}

impl From<HashMap<SmolStr, SmolStr>> for ThemeValue {
    fn from(map: HashMap<SmolStr, SmolStr>) -> Self {
        Self::Dynamic(map)
    }
}

impl From<&'static Map<&'static str, &'static str>> for ThemeValue {
    fn from(map: &'static Map<&'static str, &'static str>) -> Self {
        Self::Static(map)
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Theme(pub HashMap<SmolStr, Arc<ThemeValue>>);

impl Deref for Theme {
    type Target = HashMap<SmolStr, Arc<ThemeValue>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Theme> for HashMap<SmolStr, Arc<ThemeValue>> {
    fn from(map: Theme) -> Self {
        map.0
    }
}

impl From<HashMap<SmolStr, Arc<ThemeValue>>> for Theme {
    fn from(map: HashMap<SmolStr, Arc<ThemeValue>>) -> Self {
        Theme(map)
    }
}
