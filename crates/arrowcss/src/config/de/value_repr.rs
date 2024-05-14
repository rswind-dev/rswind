use std::borrow::Cow;

use lightningcss::{properties::PropertyId, traits::IntoOwned, values::string::CowArcStr};
use serde::{de::Error, Deserialize, Deserializer};

use crate::types::{CssDataType, CssProperty, TypeValidator};

impl<'de> Deserialize<'de> for Box<dyn TypeValidator> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Deserialize::deserialize(deserializer)?;

        CssDataType::parse_string(s.as_ref())
            .map(|typ| Box::new(typ) as Box<dyn TypeValidator>)
            .or_else(
                |_| match CssProperty::from(CowArcStr::from(s).into_owned()) {
                    PropertyId::Custom(prop) => Err(Error::custom(format!(
                        "expect css data type or property id, found `{}`",
                        prop.as_ref()
                    ))),
                    prop => Ok(Box::new(prop) as Box<dyn TypeValidator>),
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::process::RawValueRepr;

    #[test]
    fn test_deserialize() {
        let input = json!({
          "type": "color",
          "theme": "colors"
        });

        let res = RawValueRepr::deserialize(&input).unwrap();

        assert_eq!(res.theme_key, Some("colors".into()));
        // TODO: impl PartialEq for Box<dyn TypeValidator>
    }
}
