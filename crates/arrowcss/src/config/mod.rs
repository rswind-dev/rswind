pub mod de;
#[cfg(feature = "json_schema")]
pub mod schema;

use rustc_hash::FxHashMap as HashMap;
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
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
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
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum StaticUtilityConfig {
    DeclList(HashMap<SmolStr, SmolStr>),
    WithSelector((SmolStr, HashMap<SmolStr, SmolStr>)),
}

fn default_dark_mode() -> SmolStr {
    "media".into()
}

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Features {
    pub strict_mode: bool,
}
