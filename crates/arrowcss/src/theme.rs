use std::{
    fmt,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use rustc_hash::FxHashMap as HashMap;
use phf::{phf_map, Map};
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;
use smol_str::{format_smolstr, SmolStr};

#[derive(Debug, Clone)]
pub enum ThemeValue {
    Dynamic(Arc<HashMap<SmolStr, SmolStr>>),
    Static(Arc<&'static Map<&'static str, &'static str>>),
}

impl Default for ThemeValue {
    fn default() -> Self {
        Self::Static(Arc::new(&phf_map! {}))
    }
}

impl ThemeValue {
    pub fn get(&self, key: &str) -> Option<SmolStr> {
        match self {
            Self::Dynamic(map) => map.get(key).cloned(),
            Self::Static(map) => map.get(key).map(|s| SmolStr::from(*s)),
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&str, SmolStr)> + 'a> {
        match self {
            Self::Dynamic(map) => Box::new(map.iter().map(|(k, v)| (k.as_str(), v.clone()))),
            Self::Static(map) => Box::new(
                map.clone()
                    .into_iter()
                    .map(|(k, v)| (*k, SmolStr::from(*v))),
            ),
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
pub struct Theme(pub HashMap<SmolStr, ThemeValue>);

impl Theme {
    pub fn merge(mut self, other: Self) -> Self {
        for (key, value) in other.0 {
            self.0
                .entry(key.clone())
                .and_modify(|_inner_map| {
                    // TODO: reopen this
                    // let inner_map = Arc::make_mut(inner_map);
                    // inner_map.reserve(value.len());
                    // inner_map.extend(value.deref().clone().into_iter());
                })
                .or_insert(value);
        }
        self
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

struct ThemeVisitor;

impl<'de> Visitor<'de> for ThemeVisitor {
    type Value = Theme;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of themes")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Theme, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut themes = HashMap::default();
        while let Some(key) = map.next_key::<SmolStr>()? {
            match map.next_value::<serde_json::Value>()? {
                value @ Value::Object(_) => {
                    let mut theme_map: HashMap<SmolStr, SmolStr> = HashMap::default();
                    if key == "colors" {
                        match FlattenedColors::deserialize(value) {
                            Ok(b) => {
                                theme_map = b.0;
                            }
                            Err(e) => {
                                return Err(de::Error::custom(e.to_string()));
                            }
                        }
                    } else {
                        for (k, v) in value.as_object().unwrap() {
                            if let Value::String(s) = v {
                                theme_map.insert(SmolStr::from(k), SmolStr::from(s));
                            }
                        }
                    }
                    themes.insert(key, ThemeValue::Dynamic(Arc::new(theme_map)));
                }
                _ => return Err(de::Error::custom("theme only accepts object value")),
            }
        }
        Ok(Theme(themes))
    }
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Theme, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ThemeVisitor)
    }
}

#[derive(Debug, Default)]
pub struct FlattenedColors(pub HashMap<SmolStr, SmolStr>);

impl Deref for FlattenedColors {
    type Target = HashMap<SmolStr, SmolStr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FlattenedColors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FlattenedColors> for HashMap<SmolStr, SmolStr> {
    fn from(map: FlattenedColors) -> Self {
        map.0
    }
}

struct FlattenedColorsVisitor;

impl<'de> Visitor<'de> for FlattenedColorsVisitor {
    type Value = FlattenedColors;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of colors")
    }

    fn visit_map<V>(self, mut map: V) -> Result<FlattenedColors, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut colors: HashMap<SmolStr, SmolStr> = HashMap::default();
        while let Some(key) = map.next_key::<SmolStr>()? {
            match map.next_value::<serde_json::Value>()? {
                Value::String(s) => {
                    colors.insert(key, s.into());
                }
                Value::Object(nested) => {
                    for (nested_key, nested_value) in nested {
                        let flat_key = format_smolstr!("{}-{}", key, nested_key);
                        if let serde_json::Value::String(color) = nested_value {
                            colors.insert(flat_key, color.into());
                        }
                    }
                }
                _ => return Err(de::Error::custom("unexpected color format")),
            }
        }
        Ok(FlattenedColors(colors))
    }
}

impl<'de> Deserialize<'de> for FlattenedColors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(FlattenedColorsVisitor)
    }
}
