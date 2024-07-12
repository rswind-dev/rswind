use std::{
    fmt::Debug,
    mem,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use instance_code::InstanceCode;
use phf::{phf_map, Map};
use rswind_css::rule::RuleList;
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;
use values::{FontFamily, FontSize};

pub mod codegen;
mod flatten;
mod theme_value;
mod user_theme;
pub mod values;

pub use flatten::FlattenedColors;
pub use theme_value::ThemeValue;
pub use user_theme::{ThemeConfig, ThemeOptions};

#[derive(Clone, Debug, InstanceCode)]
pub enum ThemeMap {
    Dynamic(HashMap<SmolStr, SmolStr>),
    Static(&'static Map<&'static str, &'static str>),
    // Utility values
    FontSize(HashMap<SmolStr, FontSize>),
    FontFamily(HashMap<SmolStr, FontFamily>),

    KeyFrames(HashMap<SmolStr, RuleList>),
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
            _ => None,
        }
    }

    pub fn get_ref(&self, key: &str) -> Option<&str> {
        match self {
            Self::Static(map) => map.get(key).copied(),
            Self::Dynamic(map) => map.get(key).map(|s| s.as_str()),
            _ => None,
        }
    }

    pub fn get_value(&self, key: &str) -> Option<ThemeValue> {
        match self {
            Self::KeyFrames(map) => map.get(key).map(ThemeValue::KeyFrames),
            Self::FontSize(map) => map.get(key).map(ThemeValue::FontSize),
            Self::FontFamily(map) => map.get(key).map(ThemeValue::FontFamily),
            Self::Dynamic(map) => map.get(key).cloned().map(ThemeValue::Plain),
            Self::Static(map) => map.get(key).map(|v| ThemeValue::Plain(SmolStr::from(*v))),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Static(map) => map.len(),
            Self::Dynamic(map) => map.len(),
            Self::KeyFrames(map) => map.len(),
            Self::FontSize(map) => map.len(),
            Self::FontFamily(map) => map.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_rule_list(&self, key: &str) -> Option<&RuleList> {
        match self {
            Self::KeyFrames(map) => map.get(key),
            _ => None,
        }
    }

    pub fn merge(&mut self, other: Self) {
        match (self, other) {
            (s @ Self::Static(_), Self::Dynamic(d)) => s.extend(d),
            (s @ Self::Static(_), Self::Static(d)) => {
                s.extend(d.into_iter().map(|(k, v)| (SmolStr::from(*k), SmolStr::from(*v))))
            }
            (Self::Dynamic(s), Self::Dynamic(d)) => s.extend(d),
            (Self::KeyFrames(s), Self::KeyFrames(d)) => s.extend(d),
            (Self::FontSize(s), Self::FontSize(d)) => s.extend(d),
            (Self::FontFamily(s), Self::FontFamily(d)) => s.extend(d),
            _ => unreachable!(),
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&str, SmolStr)> + 'a> {
        match self {
            Self::Static(map) => Box::new(map.into_iter().map(|(k, v)| (*k, SmolStr::from(*v)))),
            Self::Dynamic(map) => Box::new(map.iter().map(|(k, v)| (k.as_str(), v.clone()))),
            _ => Box::new(std::iter::empty()),
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
        if let Self::KeyFrames(map) = self {
            map.extend(iter)
        }
    }
}

#[derive(Debug, Default, Clone, InstanceCode)]
pub struct Theme(pub HashMap<SmolStr, Arc<ThemeMap>>);

impl Theme {
    pub fn get_value(&self, key: &str, inner_key: &str) -> Option<SmolStr> {
        self.get(key).and_then(|v| v.get(inner_key))
    }

    pub fn merge(&mut self, user_theme: &mut ThemeConfig) {
        for (key, value) in mem::take(&mut user_theme.replace).into_iter() {
            self.insert(key, Arc::new(value));
        }

        for (key, value) in mem::take(&mut user_theme.extend).into_iter() {
            if let Some(entry) = self.get_mut(&key) {
                Arc::make_mut(entry).merge(value);
            } else {
                self.insert(key, Arc::new(value));
            }
        }
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
