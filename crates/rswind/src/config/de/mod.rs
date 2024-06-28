pub mod utility_handler;
pub mod value_repr;

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use crate::parsing::UtilityBuilder;

    #[test]
    fn test_deserialize() {
        let input = json!({
          "type": "color",
          "theme": "colors",
          "key": "text",
          "css": {
            "color": "$1:color",
            "opacity": "$2"
          },
          "modifier": {
            "type": "number",
            "theme": "opacity"
          }
        });

        let res = UtilityBuilder::deserialize(&input).unwrap();

        assert_eq!(res.key, "text");
        assert_eq!(res.theme_key, Some("colors".into()));
        assert_eq!(res.modifier.unwrap().theme_key, Some("opacity".into()));

        // TODO: impl PartialEq for UtilityHandler
        // assert_eq!(res.value_repr.validator, Some(CssDataType::Color.into()));
        // assert_eq!(res.modifier.unwrap().validator, Some(CssDataType::Number.into());
    }
}
