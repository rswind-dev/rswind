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

#[derive(Debug, Deserialize, Default)]
pub struct Theme {
    pub colors: FlattenedColors,
    pub spacing: HashMap<String, String>,
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
    fn test_flattened_colors_deserialization() {
        let json_str = r##"{
            "inherit": "inherit",
            "slate": {
                "50": "#f8fafc"
            }
        }"##;

        let flattened_colors: FlattenedColors = serde_json::from_str(json_str).unwrap();

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
