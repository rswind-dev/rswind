use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
use std::{fmt, sync::Arc};

#[derive(Debug, Default)]
pub struct Theme(pub HashMap<String, Arc<HashMap<String, String>>>);

impl Deref for Theme {
    type Target = HashMap<String, Arc<HashMap<String, String>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Theme> for HashMap<String, Arc<HashMap<String, String>>> {
    fn from(map: Theme) -> Self {
        map.0
    }
}

impl From<HashMap<String, Arc<HashMap<String, String>>>> for Theme {
    fn from(map: HashMap<String, Arc<HashMap<String, String>>>) -> Self {
        Theme(map)
    }
}

impl Theme {
    pub fn merge(mut self, other: Theme) -> Self {
        for (key, value) in other.0 {
            self.0
                .entry(key.clone())
                .and_modify(|inner_map| {
                    let mut_arc = Arc::make_mut(inner_map);
                    for (inner_key, inner_value) in value.iter() {
                        mut_arc.insert(inner_key.clone(), inner_value.clone());
                    }
                })
                .or_insert(value.clone());
        }
        self
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
        let mut themes = HashMap::new();
        while let Some(key) = map.next_key::<String>()? {
            match map.next_value::<serde_json::Value>()? {
                ref value @ Value::Object(ref theme) => {
                    let mut theme_map = HashMap::new();
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
                        for (k, v) in theme {
                            if let Value::String(s) = v {
                                theme_map.insert(k.to_string(), s.to_string());
                            }
                        }
                    }
                    themes.insert(key, Arc::new(theme_map));
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

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Theme, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ThemeVisitor)
    }
}

#[derive(Debug, Default)]
pub struct FlattenedColors(pub HashMap<String, String>);

impl Deref for FlattenedColors {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FlattenedColors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FlattenedColors> for HashMap<String, String> {
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
        let mut colors = HashMap::new();
        while let Some(key) = map.next_key::<String>()? {
            match map.next_value::<serde_json::Value>()? {
                Value::String(s) => {
                    colors.insert(key, s);
                }
                Value::Object(nested) => {
                    for (nested_key, nested_value) in nested {
                        let flat_key = format!("{}-{}", key, nested_key);
                        if let serde_json::Value::String(color) = nested_value {
                            colors.insert(flat_key, color);
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
#[cfg(test)]
mod tests {
    use crate::map;

    use super::*;

    #[test]
    fn test_theme_deserialization() {
        let json_str = r##"{
            "spacing": {
                "1": "0.25rem"
            },
            "colors": {
                "inherit": "inherit",
                "slate": {
                    "50": "#f8fafc"
                }
            }
        }"##;

        let theme = serde_json::from_str::<Theme>(json_str).unwrap();

        assert_eq!(
            theme.get("colors").unwrap().get("inherit"),
            Some(&"inherit".to_string())
        );
        assert_eq!(
            theme.get("colors").unwrap().get("slate-50"),
            Some(&"#f8fafc".to_string())
        );
        assert_eq!(
            theme.get("spacing").unwrap().get("1"),
            Some(&"0.25rem".to_string())
        );
    }

    #[test]
    fn test_flattened_colors_deserialization() {
        let json_str = r##"{
            "inherit": "inherit",
            "slate": {
                "50": "#f8fafc"
            }
        }"##;

        let flattened_colors: FlattenedColors =
            serde_json::from_str(json_str).unwrap();

        assert_eq!(
            flattened_colors.get("inherit"),
            Some(&"inherit".to_string())
        );
        assert_eq!(
            flattened_colors.get("slate-50"),
            Some(&"#f8fafc".to_string())
        );
    }

    #[test]
    fn test_theme_merge() {
        let mut theme1 = Theme::default();
        let mut theme2 = Theme::default();

        theme1.insert(
            "colors".to_string(),
            Arc::new(map! {
                "inherit" => "inherit".to_string(),
                "slate-50" => "#f8fafc".to_string()
            }),
        );

        theme2.insert(
            "spacing".to_string(),
            Arc::new(map! {
                "1" => "0.25rem".to_string()
            }),
        );

        theme2.insert(
            "colors".to_string(),
            Arc::new(map! {
                "inherit" => "inherit-merged".to_string()
            }),
        );

        let theme1 = theme1.merge(theme2);

        assert_eq!(
            theme1.get("colors").unwrap().get("slate-50"),
            Some(&"#f8fafc".to_string())
        );
        assert_eq!(
            theme1.get("spacing").unwrap().get("1"),
            Some(&"0.25rem".to_string())
        );
        assert_eq!(
            theme1.get("colors").unwrap().get("inherit"),
            Some(&"inherit-merged".to_string())
        );
    }
}
