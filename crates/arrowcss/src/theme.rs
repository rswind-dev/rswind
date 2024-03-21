use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;
use std::fmt;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default)]
pub struct Theme(pub HashMap<String, HashMap<String, String>>);

impl Deref for Theme {
    type Target = HashMap<String, HashMap<String, String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
                    themes.insert(key, theme_map);
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
}
