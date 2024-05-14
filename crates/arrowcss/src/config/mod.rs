pub mod de;

use serde::Deserialize;

use crate::{parsing::UtilityBuilder, theme::Theme};

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CorePlugins {
    pub preflight: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct ArrowConfig {
    #[serde(default)]
    pub theme: Theme,
    #[serde(default = "default_dark_mode")]
    pub dark_mode: String,
    #[serde(default)]
    pub features: Features,
    #[serde(default)]
    pub utilities: Vec<UtilityBuilder>,
}

fn default_dark_mode() -> String {
    "media".into()
}

#[derive(Debug, Deserialize, Default)]
pub struct Features {
    pub strict_mode: bool,
}
