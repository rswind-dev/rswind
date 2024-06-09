use core::fmt;
use std::ops::{Deref, DerefMut};

use rustc_hash::FxHashMap as HashMap;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;
use smol_str::{format_smolstr, SmolStr};

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
