use std::ops::{Deref, DerefMut};
use std::{fmt, sync::Arc};

use fxhash::FxHashMap as HashMap;
use lightningcss::values::string::CowArcStr;
use phf::Map;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum ThemeValue<'c> {
    Dynamic(Arc<HashMap<String, CowArcStr<'c>>>),
    Static(Arc<&'static Map<&'static str, &'static str>>),
}

impl<'c> ThemeValue<'c> {
    pub fn get(&self, key: &str) -> Option<CowArcStr<'c>> {
        match self {
            Self::Dynamic(map) => map.get(key).cloned(),
            Self::Static(map) => map.get(key).map(|s| CowArcStr::from(*s)),
        }
    }

    pub fn for_each(&self, f: impl Fn((&str, &CowArcStr<'c>))) {
        match self {
            Self::Dynamic(map) => map.iter().for_each(|(k, v)| f((k, v))),
            Self::Static(map) => map
                .into_iter()
                .for_each(|(k, v)| f((k, &CowArcStr::from(*v)))),
        }
    }

    pub fn iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (&str, CowArcStr<'c>)> + 'a> {
        match self {
            Self::Dynamic(map) => {
                Box::new(map.iter().map(|(k, v)| (k.as_str(), v.clone())))
            }
            Self::Static(map) => Box::new(
                map.clone()
                    .into_iter()
                    .map(|(k, v)| (*k, CowArcStr::from(*v))),
            ),
        }
    }
}

impl<'a> From<HashMap<String, CowArcStr<'a>>> for ThemeValue<'a> {
    fn from(map: HashMap<String, CowArcStr<'a>>) -> Self {
        Self::Dynamic(Arc::new(map))
    }
}

impl<'a> From<&'static Map<&'static str, &'static str>> for ThemeValue<'a> {
    fn from(map: &'static Map<&'static str, &'static str>) -> Self {
        Self::Static(Arc::new(map))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Theme<'c>(pub HashMap<String, ThemeValue<'c>>);

impl<'c> Theme<'c> {
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

impl<'c> Deref for Theme<'c> {
    type Target = HashMap<String, ThemeValue<'c>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'c> DerefMut for Theme<'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'c> From<Theme<'c>> for HashMap<String, ThemeValue<'c>> {
    fn from(map: Theme<'c>) -> Self {
        map.0
    }
}

impl<'c> From<HashMap<String, ThemeValue<'c>>> for Theme<'c> {
    fn from(map: HashMap<String, ThemeValue<'c>>) -> Self {
        Theme(map)
    }
}

struct ThemeVisitor;

impl<'de> Visitor<'de> for ThemeVisitor {
    type Value = Theme<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of themes")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Theme<'de>, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut themes = HashMap::default();
        while let Some(key) = map.next_key::<String>()? {
            match map.next_value::<serde_json::Value>()? {
                value @ Value::Object(_) => {
                    let mut theme_map: HashMap<String, CowArcStr<'de>> =
                        HashMap::default();
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
                                theme_map.insert(
                                    k.to_string(),
                                    s.to_string().into(),
                                );
                            }
                        }
                    }
                    themes
                        .insert(key, ThemeValue::Dynamic(Arc::new(theme_map)));
                }
                _ => {
                    return Err(de::Error::custom(
                        "theme only accepts object value",
                    ))
                }
            }
        }
        Ok(Theme(themes))
    }
}

impl<'de> Deserialize<'de> for Theme<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Theme<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ThemeVisitor)
    }
}

#[derive(Debug, Default)]
pub struct FlattenedColors<'c>(pub HashMap<String, CowArcStr<'c>>);

impl<'c> Deref for FlattenedColors<'c> {
    type Target = HashMap<String, CowArcStr<'c>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'c> DerefMut for FlattenedColors<'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'c> From<FlattenedColors<'c>> for HashMap<String, CowArcStr<'c>> {
    fn from(map: FlattenedColors<'c>) -> Self {
        map.0
    }
}

struct FlattenedColorsVisitor;

impl<'de> Visitor<'de> for FlattenedColorsVisitor {
    type Value = FlattenedColors<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of colors")
    }

    fn visit_map<V>(self, mut map: V) -> Result<FlattenedColors<'de>, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut colors: HashMap<String, CowArcStr> = HashMap::default();
        while let Some(key) = map.next_key::<String>()? {
            match map.next_value::<serde_json::Value>()? {
                Value::String(s) => {
                    colors.insert(key, s.into());
                }
                Value::Object(nested) => {
                    for (nested_key, nested_value) in nested {
                        let flat_key = format!("{}-{}", key, nested_key);
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

impl<'de> Deserialize<'de> for FlattenedColors<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(FlattenedColorsVisitor)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::map;

//     use super::*;

//     #[test]
//     fn test_theme_deserialization() {
//         let json_str = r##"{
//             "spacing": {
//                 "1": "0.25rem"
//             },
//             "colors": {
//                 "inherit": "inherit",
//                 "slate": {
//                     "50": "#f8fafc"
//                 }
//             }
//         }"##;

//         let theme = serde_json::from_str::<Theme>(json_str).unwrap();

//         assert_eq!(
//             theme.get("colors").unwrap().get("inherit"),
//             Some(&"inherit".to_string().into())
//         );
//         assert_eq!(
//             theme.get("colors").unwrap().get("slate-50"),
//             Some(&"#f8fafc".to_string().into())
//         );
//         assert_eq!(
//             theme.get("spacing").unwrap().get("1"),
//             Some(&"0.25rem".to_string().into())
//         );
//     }

//     #[test]
//     fn test_flattened_colors_deserialization() {
//         let json_str = r##"{
//             "inherit": "inherit",
//             "slate": {
//                 "50": "#f8fafc"
//             }
//         }"##;

//         let flattened_colors: FlattenedColors =
//             serde_json::from_str(json_str).unwrap();

//         assert_eq!(
//             flattened_colors.get("inherit"),
//             Some(&"inherit".to_string().into())
//         );
//         assert_eq!(
//             flattened_colors.get("slate-50"),
//             Some(&"#f8fafc".to_string().into())
//         );
//     }

//     #[test]
//     fn test_theme_merge() {
//         let mut theme1 = Theme::default();
//         let mut theme2 = Theme::default();

//         theme1.insert(
//             "colors".to_string(),
//             Arc::new(map! {
//                 "inherit" => "inherit".to_string(),
//                 "slate-50" => "#f8fafc".to_string()
//             }),
//         );

//         theme2.insert(
//             "spacing".to_string(),
//             Arc::new(map! {
//                 "1" => "0.25rem".to_string()
//             }),
//         );

//         theme2.insert(
//             "colors".to_string(),
//             Arc::new(map! {
//                 "inherit" => "inherit-merged".to_string()
//             }),
//         );

//         let theme1 = theme1.merge(theme2);

//         assert_eq!(
//             theme1
//                 .get("colors")
//                 .unwrap()
//                 .get("slate-50")
//                 .map(|s| s.to_string()),
//             Some("#f8fafc".to_string())
//         );
//         assert_eq!(
//             theme1
//                 .get("spacing")
//                 .unwrap()
//                 .get("1")
//                 .map(|s| s.to_string()),
//             Some("0.25rem".to_string())
//         );
//         assert_eq!(
//             theme1
//                 .get("colors")
//                 .unwrap()
//                 .get("inherit")
//                 .map(|s| s.to_string()),
//             Some("inherit-merged".to_string())
//         );
//     }
// }
