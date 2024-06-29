pub mod de;
#[cfg(feature = "json_schema")]
pub mod schema;
// pub mod user_theme;

use std::{io, str::FromStr};

use config::Config;
use rustc_hash::FxHashMap as HashMap;
use serde::Deserialize;
use smol_str::SmolStr;
use thiserror::Error;
use tracing::{debug, info, instrument};

use crate::parsing::UtilityBuilder;

pub static DEFAULT_CONFIG_PATH: &str = "rswind.config.json";

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

// This `wbg_shim` exist because of the following issue:
// https://github.com/rustwasm/wasm-bindgen/pull/3946
// TODO: remove this shim when `wasm-bindgen` releases 0.2.93
// Also, see https://github.com/rust-lang/rust-analyzer/issues/8747
#[allow(non_snake_case, clippy::empty_docs)]
mod wbg_shim {
    use rswind_theme::ThemeConfig;

    use super::*;
    #[derive(Debug, Default, Deserialize)]
    #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(from_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    pub struct GeneratorConfig {
        /// The glob pattern to match input files
        pub content: Vec<String>,

        /// User define themes, will be merged with the default theme
        pub theme: ThemeConfig,

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
}

pub use wbg_shim::GeneratorConfig;

#[derive(Debug, Error)]
pub enum GeneratorConfigError {
    // #[error("Failed to deserialize configuration: {0}")]
    // DeserializeError(#[from] serde_json::Error),
    #[error("Failed to read configuration file: {0}")]
    ConfigError(#[from] config::ConfigError),
}

#[cfg(feature = "napi")]
impl From<GeneratorConfigError> for napi::Error {
    fn from(err: GeneratorConfigError) -> Self {
        napi::Error::new(napi::Status::GenericFailure, err.to_string())
    }
}

impl GeneratorConfig {
    #[instrument]
    pub fn from_file(name: &str) -> Result<Self, GeneratorConfigError> {
        let config_result = Config::builder().add_source(config::File::with_name(name)).build();

        let config = match config_result {
            Ok(config) => config.try_deserialize::<GeneratorConfig>(),
            // If the file is not found, use the default configuration
            Err(config::ConfigError::Foreign(err))
                if err
                    .downcast_ref::<io::Error>()
                    .map_or(false, |io_err| io_err.kind() == io::ErrorKind::NotFound) =>
            {
                info!("No configuration file found, using default configuration");
                Ok(GeneratorConfig::default())
            }
            Err(e) => Err(e),
        };

        debug!(config = ?config,"Loaded configuration");

        Ok(config?)
    }

    pub fn from_value(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

impl FromStr for GeneratorConfig {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
