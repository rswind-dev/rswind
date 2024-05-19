use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use phf::{phf_map, Map};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;

use crate::css::rule::RuleList;

#[derive(Clone)]
pub enum ThemeValue {
    Dynamic(Arc<HashMap<SmolStr, SmolStr>>),
    Static(Arc<&'static Map<&'static str, &'static str>>),
    RuleList(Arc<HashMap<SmolStr, RuleList>>),
}

impl Debug for ThemeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dynamic(map) => write!(f, "ThemeValue::Dynamic(len: {:?})", map.len()),
            Self::Static(map) => write!(f, "ThemeValue::Static(len: {:?})", map.len()),
            Self::RuleList(map) => write!(f, "ThemeValue::RuleList(len: {:?})", map.len()),
        }
    }
}

impl Default for ThemeValue {
    fn default() -> Self {
        Self::Static(Arc::new(&phf_map! {}))
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
            Self::Static(map) => Box::new(
                map.clone()
                    .into_iter()
                    .map(|(k, v)| (*k, SmolStr::from(*v))),
            ),
            Self::Dynamic(map) => Box::new(map.iter().map(|(k, v)| (k.as_str(), v.clone()))),
            Self::RuleList(_) => Box::new(std::iter::empty()),
        }
    }
}

impl From<HashMap<SmolStr, SmolStr>> for ThemeValue {
    fn from(map: HashMap<SmolStr, SmolStr>) -> Self {
        Self::Dynamic(Arc::new(map))
    }
}

impl From<&'static Map<&'static str, &'static str>> for ThemeValue {
    fn from(map: &'static Map<&'static str, &'static str>) -> Self {
        Self::Static(Arc::new(map))
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Theme(pub HashMap<SmolStr, ThemeValue>);

impl Theme {
    pub fn merge(&mut self, key: SmolStr, mut value: HashMap<SmolStr, SmolStr>) {
        if let Some(entry) = self.get_mut(key.as_str()) {
            match entry {
                ThemeValue::Dynamic(d) => {
                    let inner_map = Arc::make_mut(d);
                    inner_map.reserve(value.len());
                    inner_map.extend(value);
                }
                ThemeValue::Static(s) => {
                    value.reserve(s.len());
                    value.extend(
                        s.into_iter()
                            .map(|(k, v)| (SmolStr::from(*k), SmolStr::from(*v))),
                    );
                    *entry = ThemeValue::Dynamic(Arc::new(value));
                }
                ThemeValue::RuleList(_r) => {
                    todo!()
                    // let inner_map = Arc::make_mut(r);
                    // inner_map.reserve(value.len());
                    // inner_map.extend(value);
                }
            }
            return;
        }

        self.insert(key, ThemeValue::Dynamic(Arc::new(value)));
    }
}

impl Deref for Theme {
    type Target = HashMap<SmolStr, ThemeValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Theme> for HashMap<SmolStr, ThemeValue> {
    fn from(map: Theme) -> Self {
        map.0
    }
}

impl From<HashMap<SmolStr, ThemeValue>> for Theme {
    fn from(map: HashMap<SmolStr, ThemeValue>) -> Self {
        Theme(map)
    }
}
