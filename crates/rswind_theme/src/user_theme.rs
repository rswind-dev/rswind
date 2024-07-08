use instance_code::InstanceCode;
use itertools::put_back_n;
use rswind_css::rule::RuleList;
use rustc_hash::FxHashMap as HashMap;
use serde::Deserialize;
use smol_str::SmolStr;

use crate::{flatten::FlattenedColors, FontFamily, FontSize, ThemeMap};

#[derive(Debug, Default, Clone, Deserialize, InstanceCode)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ThemeOptions {
    #[serde(default)]
    pub colors: Option<FlattenedColors>,
    #[serde(default)]
    pub keyframes: Option<HashMap<SmolStr, RuleList>>,
    #[serde(default)]
    pub font_family: Option<HashMap<SmolStr, FontFamily>>,
    #[serde(default)]
    pub font_size: Option<HashMap<SmolStr, FontSize>>,

    #[serde(flatten)]
    pub normal: HashMap<SmolStr, HashMap<SmolStr, SmolStr>>,
}

impl ThemeOptions {
    #[allow(clippy::should_implement_trait)] // TODO: can't describe the type of `Iterator`, too complex
    pub fn into_iter(self) -> impl Iterator<Item = (SmolStr, ThemeMap)> {
        let mut iter = put_back_n(self.normal.into_iter().map(|(k, v)| (k, ThemeMap::Dynamic(v))));

        if let Some(v) = self.colors {
            iter.put_back(("colors".into(), ThemeMap::Dynamic(v.0)))
        }
        if let Some(v) = self.keyframes {
            iter.put_back(("keyframes".into(), ThemeMap::KeyFrames(v)))
        }
        if let Some(v) = self.font_family {
            iter.put_back(("fontFamily".into(), ThemeMap::FontFamily(v)))
        }
        if let Some(v) = self.font_size {
            iter.put_back(("fontSize".into(), ThemeMap::FontSize(v)))
        }

        iter
    }
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct ThemeConfig {
    #[serde(default)]
    pub extend: ThemeOptions,
    #[serde(flatten)]
    pub replace: ThemeOptions,
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::Theme;

    use super::*;
    use rswind_css_macros::rule_list;
    use serde_json::json;

    #[test]
    fn test_user_theme() {
        let json = json!({
            "backgroundSize": {
                "...": "$spacing",
                "auto": "auto",
                "cover": "cover",
                "contain": "contain"
            },
            "foo": {
                "...": "$colors",
            },
            "extend": {
                "spacing": {
                    "big": "0.25rem",
                },
                "colors": {
                    "red": "#ff0000"
                },
                "keyframes": {
                    "wiggle": {
                        "0%, 100%": { "transform": "rotate(-3deg)" },
                        "50%": { "transform": "rotate(3deg)" }
                    }
                }
            }
        });

        let mut user_theme: ThemeConfig = serde_json::from_value(json).unwrap();

        let mut default_theme = Theme::default();

        default_theme.merge(&mut user_theme);

        // assert_eq!(default_theme.get_value("backgroundSize", "1").unwrap(), "0.25rem");
        // assert_eq!(default_theme.get_value("backgroundSize", "big").unwrap(), "0.25rem");
        assert_eq!(default_theme.get_value("colors", "red").unwrap(), "#ff0000");
        // assert_eq!(default_theme.get_value("foo", "red").unwrap(), "#ff0000");
        assert_eq!(
            default_theme.get("keyframes").unwrap().deref().get_rule_list("wiggle").unwrap(),
            &rule_list! {
                "0%, 100%" { "transform": "rotate(-3deg)" }
                "50%" { "transform": "rotate(3deg)" }
            }
        );
    }
}
