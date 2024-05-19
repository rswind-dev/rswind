pub mod de;
#[cfg(feature = "json_schema")]
pub mod schema;

use std::{io, str::FromStr};

use config::Config;
use rustc_hash::FxHashMap as HashMap;
use serde::Deserialize;
use smol_str::SmolStr;
use thiserror::Error;
use tracing::{info, instrument};

use crate::{parsing::UtilityBuilder, theme::Theme};

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CorePlugins {
    pub preflight: bool,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum StaticUtilityConfig {
    DeclList(HashMap<SmolStr, SmolStr>),
    WithSelector((SmolStr, HashMap<SmolStr, SmolStr>)),
}

#[derive(Debug, Deserialize, Default)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct Features {
    /// Use a lexer to parse candidate, default to `true`
    /// if set to `false`, the parser will use regex to parse candidate
    pub strict_mode: bool,
}

fn default_dark_mode() -> SmolStr {
    "media".into()
}

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ArrowConfig {
    /// User define themes, will be merged with the default theme
    pub theme: Theme,

    // TODO: support user defined dark mode, like
    // ['variant', '&:not(.light *)']
    // ['selector', '[data-mode="dark"]']
    // ['variant', [
    //   '@media (prefers-color-scheme: dark) { &:not(.light *) }',
    //   '&:is(.dark *)',
    // ]]
    /// How to handle `dark:` variant, can be `media` or `selector`
    #[serde(default = "default_dark_mode")]
    pub dark_mode: SmolStr,

    pub features: Features,

    /// User defined dynamic utilities, e.g. `bg-blue-500`
    pub utilities: Vec<UtilityBuilder>,

    /// User defined static utilities e.g. `flex`
    pub static_utilities: HashMap<SmolStr, StaticUtilityConfig>,
}

#[derive(Debug, Error)]
pub enum ArrowConfigError {
    // #[error("Failed to deserialize configuration: {0}")]
    // DeserializeError(#[from] serde_json::Error),
    #[error("Failed to read configuration file: {0}")]
    ConfigError(#[from] config::ConfigError),
}

impl ArrowConfig {
    #[cfg(not(feature = "wasm"))]
    #[instrument]
    pub fn from_file(name: &str) -> Result<Self, config::ConfigError> {
        let config_result = Config::builder().add_source(config::File::with_name(name)).build();

        match config_result {
            Ok(config) => config.try_deserialize::<ArrowConfig>(),
            // If the file is not found, use the default configuration
            Err(config::ConfigError::Foreign(err))
                if err
                    .downcast_ref::<io::Error>()
                    .map_or(false, |io_err| io_err.kind() == io::ErrorKind::NotFound) =>
            {
                info!("No configuration file found, using default configuration");
                Ok(ArrowConfig::default())
            }
            Err(e) => Err(e),
        }
    }

    #[cfg(feature = "wasm")]
    pub fn from_js(config: wasm_bindgen::JsValue) -> Result<Self, serde_wasm_bindgen::Error> {
        let config: ArrowConfig = serde_wasm_bindgen::from_value(config)?;
        Ok(config)
    }
}

impl FromStr for ArrowConfig {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
