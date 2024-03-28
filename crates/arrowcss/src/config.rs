use serde::Deserialize;

use crate::theme::Theme;

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CorePlugins {
    pub preflight: bool,
    pub text_opacity: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArrowConfig<'d> {
    #[serde(bound(deserialize = "'d: 'de, 'de: 'd"))]
    pub theme: Theme<'d>,
    #[serde(flatten)]
    pub config: Config,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Config {
    pub dark_mode: String,
    pub core_plugins: CorePlugins,
}
