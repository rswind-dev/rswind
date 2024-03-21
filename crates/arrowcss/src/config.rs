use serde::Deserialize;

use crate::theme::Theme;

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CorePlugins {
    pub preflight: bool,
    pub text_opacity: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArrowConfig {
    pub theme: Theme,
    #[serde(flatten)]
    pub config: Config,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub dark_mode: String,
    pub core_plugins: CorePlugins,
}
