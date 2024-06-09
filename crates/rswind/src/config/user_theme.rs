use std::{mem, sync::Arc};

use either::Either::{Left, Right};
use rustc_hash::FxHashMap as HashMap;
use serde::Deserialize;
use smol_str::SmolStr;

use crate::{
    css::rule::RuleList,
    theme::{Theme, ThemeValue},
};

use super::de::theme::FlattenedColors;

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct UserThemeBase {
    #[serde(default)]
    pub(crate) colors: Option<FlattenedColors>,
    #[serde(default)]
    pub(crate) keyframes: Option<HashMap<SmolStr, RuleList>>,
    #[serde(flatten)]
    pub(crate) normal: HashMap<SmolStr, HashMap<SmolStr, SmolStr>>,
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct UserTheme {
    #[serde(default)]
    pub(crate) extend: UserThemeBase,
    #[serde(flatten)]
    pub(crate) replace: UserThemeBase,
}

impl UserTheme {
    pub fn drain(&mut self) -> Self {
        Self {
            extend: UserThemeBase {
                colors: self.extend.colors.take(),
                keyframes: self.extend.keyframes.take(),
                normal: mem::take(&mut self.extend.normal),
            },
            replace: UserThemeBase {
                colors: self.replace.colors.take(),
                keyframes: self.replace.keyframes.take(),
                normal: mem::take(&mut self.replace.normal),
            },
        }
    }
}

fn expand_spread(map: &mut HashMap<SmolStr, SmolStr>, theme: &Theme) {
    if let Some(v) = map.get("...").and_then(|v| v.strip_prefix('$')).map(SmolStr::from) {
        map.remove("...");

        // TODO: error handling
        let expand = theme.get(&v).unwrap();
        map.extend(match expand.as_ref() {
            ThemeValue::Dynamic(map) => Left(map.clone().into_iter()),
            ThemeValue::Static(map) => {
                Right(map.into_iter().map(|(k, v)| (SmolStr::from(*k), SmolStr::from(*v))))
            }
            _ => {
                // warn_once
                Left(HashMap::default().into_iter())
            }
        });
    }
}

impl Theme {
    pub fn merge(&mut self, user_theme: UserTheme) {
        for (key, mut value) in user_theme.replace.normal {
            expand_spread(&mut value, self);
            self.insert(key, Arc::new(ThemeValue::Dynamic(value)));
        }
        if let Some(colors) = user_theme.replace.colors {
            self.insert("colors".into(), Arc::new(ThemeValue::Dynamic(colors.0)));
        }
        if let Some(keyframes) = user_theme.replace.keyframes {
            self.insert("keyframes".into(), Arc::new(ThemeValue::RuleList(keyframes)));
        }

        for (key, mut value) in user_theme.extend.normal {
            expand_spread(&mut value, self);
            if let Some(entry) = self.get_mut(&key) {
                Arc::make_mut(entry).extend(value);
            } else {
                self.insert(key, Arc::new(ThemeValue::Dynamic(value)));
            }
        }

        if let Some(colors) = user_theme.extend.colors {
            if let Some(entry) = self.get_mut("colors") {
                Arc::make_mut(entry).extend(colors.0);
            } else {
                self.insert("colors".into(), Arc::new(ThemeValue::Dynamic(colors.0)));
            }
        }

        if let Some(keyframes) = user_theme.extend.keyframes {
            if let Some(entry) = self.get_mut("keyframes") {
                Arc::make_mut(entry).extend(keyframes);
            } else {
                self.insert("keyframes".into(), Arc::new(ThemeValue::RuleList(keyframes)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use crate::preset::theme::theme;
    use serde_json::json;

    #[test]
    fn test_user_theme() {
        let json = json!({
            "spacing": {
                "1": "0.25rem",
            },
            "backgroundSize": {
                "...": "$spacing",
                "auto": "auto",
                "cover": "cover",
                "contain": "contain"
            },
            "extend": {
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

        let user_theme: UserTheme = serde_json::from_value(json).unwrap();

        let mut default_theme = theme();

        default_theme.merge(user_theme);

        assert_eq!(default_theme.get("spacing").unwrap().len(), 1);

        assert_eq!(default_theme.get_value("backgroundSize", "1").unwrap(), "0.25rem");
        assert_eq!(default_theme.get_value("backgroundSize", "2").unwrap(), "0.5rem");
        assert_eq!(default_theme.get_value("colors", "red").unwrap(), "#ff0000");
        assert_eq!(
            default_theme.get("keyframes").unwrap().deref().get_rule_list("wiggle").unwrap(),
            &serde_json::from_value::<RuleList>(json!({
                "0%, 100%": { "transform": "rotate(-3deg)" },
                "50%": { "transform": "rotate(3deg)" }
            }))
            .unwrap()
        );
    }
}
