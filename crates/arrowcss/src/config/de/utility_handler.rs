use rustc_hash::FxHashMap;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use smol_str::SmolStr;

use crate::{
    css::{Decl, Rule},
    preset::dynamics::as_color,
    process::UtilityHandler,
};

impl<'de> Deserialize<'de> for UtilityHandler {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UtilityHandlerVisitor;

        impl<'de> Visitor<'de> for UtilityHandlerVisitor {
            type Value = UtilityHandler;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of utility handler")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut map = map;
                let mut handlers = FxHashMap::default();

                while let Some((key, value)) = map.next_entry::<SmolStr, SmolStr>()? {
                    handlers.insert(key, value);
                }

                Ok(UtilityHandler::new(move |meta, value| {
                    Rule::new_empty(handlers.iter().filter_map(|(k, v)| {
                        Some(Decl::new(
                            k.as_str(),
                            // filter out $2 if modifier is None
                            parse_dollar_sign(v, value.as_str(), meta.modifier.as_deref())?,
                        ))
                    }))
                }))
            }
        }

        deserializer.deserialize_map(UtilityHandlerVisitor)
    }
}

fn parse_dollar_sign<'a>(v: &'a str, value: &str, modifier: Option<&str>) -> Option<SmolStr> {
    if v.contains("$2") && modifier.is_none() {
        return None;
    }

    // TODO: avoid allocation
    Some(
        v.replace("$2", &modifier.unwrap_or_default())
            .replace("$1:color", &as_color(value, modifier.as_deref()))
            .replace("$1", value)
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;
    use smol_str::SmolStr;

    use crate::{
        css::ToCssString,
        process::{MetaData, UtilityHandler},
    };

    #[test]
    fn test_css_fn_deserializer() -> anyhow::Result<()> {
        let input = json!({
            "color": "$1:color",
            "opacity": "$2"
        });

        let res = UtilityHandler::deserialize(input)?;

        let r = res.call(
            MetaData::modifier(SmolStr::from("0.5")),
            SmolStr::from("red"),
        );

        println!("{}", r.to_css_string());

        Ok(())
    }
}
