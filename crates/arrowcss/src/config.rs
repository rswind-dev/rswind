use serde::Deserialize;

use crate::theme::Theme;

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CorePlugins {
    pub preflight: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArrowConfig {
    #[serde(default)]
    pub theme: Theme,
    #[serde(flatten, default)]
    pub config: Config,
    pub features: Features,
}

#[derive(Debug, Deserialize, Default)]
pub struct Features {
    pub strict_mode: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Config {
    #[serde(default)]
    pub dark_mode: String,
    #[serde(default)]
    pub core_plugins: CorePlugins,
}
