use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use phf::{phf_map, Map};
use rswind_css::rule::RuleList;
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;

#[derive(Clone, Debug)]
pub enum ThemeMap {
    Dynamic(HashMap<SmolStr, SmolStr>),
    Static(&'static Map<&'static str, &'static str>),
    RuleList(HashMap<SmolStr, RuleList>),
}

impl Default for ThemeMap {
    fn default() -> Self {
        Self::Static(&phf_map! {})
    }
}

impl ThemeMap {
    pub fn get(&self, key: &str) -> Option<SmolStr> {
        match self {
            Self::Static(map) => map.get(key).map(|s| SmolStr::from(*s)),
            Self::Dynamic(map) => map.get(key).cloned(),
            Self::RuleList(_) => None,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Static(map) => map.len(),
            Self::Dynamic(map) => map.len(),
            Self::RuleList(map) => map.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

impl From<HashMap<SmolStr, SmolStr>> for ThemeMap {
    fn from(map: HashMap<SmolStr, SmolStr>) -> Self {
        Self::Dynamic(map)
    }
}

impl From<&'static Map<&'static str, &'static str>> for ThemeMap {
    fn from(map: &'static Map<&'static str, &'static str>) -> Self {
        Self::Static(map)
    }
}

impl Extend<(SmolStr, SmolStr)> for ThemeMap {
    fn extend<T: IntoIterator<Item = (SmolStr, SmolStr)>>(&mut self, iter: T) {
        match self {
            Self::Dynamic(map) => map.extend(iter),
            Self::Static(s) => {
                *self = Self::Dynamic(
                    iter.into_iter()
                        .chain(s.into_iter().map(|(k, v)| (SmolStr::from(*k), SmolStr::from(*v))))
                        .collect(),
                )
            }
            _ => {}
        }
    }
}

impl Extend<(SmolStr, RuleList)> for ThemeMap {
    fn extend<T: IntoIterator<Item = (SmolStr, RuleList)>>(&mut self, iter: T) {
        if let Self::RuleList(map) = self {
            map.extend(iter)
        }
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Theme(pub HashMap<SmolStr, Arc<ThemeMap>>);

impl Theme {
    pub fn get_value(&self, key: &str, inner_key: &str) -> Option<SmolStr> {
        self.get(key).and_then(|v| v.get(inner_key))
    }
}

impl Deref for Theme {
    type Target = HashMap<SmolStr, Arc<ThemeMap>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Theme> for HashMap<SmolStr, Arc<ThemeMap>> {
    fn from(map: Theme) -> Self {
        map.0
    }
}

impl From<HashMap<SmolStr, Arc<ThemeMap>>> for Theme {
    fn from(map: HashMap<SmolStr, Arc<ThemeMap>>) -> Self {
        Theme(map)
    }
}
