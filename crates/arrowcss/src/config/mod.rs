pub mod de;

use std::collections::HashMap;

use serde::Deserialize;
use smol_str::SmolStr;

use crate::{parsing::UtilityBuilder, theme::Theme};

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CorePlugins {
    pub preflight: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ArrowConfig {
    #[serde(default)]
    pub theme: Theme,
    #[serde(default = "default_dark_mode")]
    pub dark_mode: SmolStr,
    #[serde(default)]
    pub features: Features,
    #[serde(default)]
    pub utilities: Vec<UtilityBuilder>,
    #[serde(default)]
    pub static_utilities: HashMap<SmolStr, StaticUtilityConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StaticUtilityConfig {
    DeclList(HashMap<SmolStr, SmolStr>),
    WithSelector((SmolStr, HashMap<SmolStr, SmolStr>)),
}

fn default_dark_mode() -> SmolStr {
    "media".into()
}

#[derive(Debug, Deserialize, Default)]
pub struct Features {
    pub strict_mode: bool,
}
