pub mod cache;
pub mod common;
pub mod config;
pub mod design;
pub mod generator;
pub mod glob;
pub mod io;
pub mod ordering;
pub mod parse;
pub mod process;
pub mod processor;
pub mod types;

pub use config::GeneratorConfig;
pub use generator::Generator;

pub use design::DesignSystem;

pub mod css {
    pub use rswind_css::*;
    pub use rswind_css_macros::*;
}

pub mod theme {
    pub use rswind_theme::*;
}

pub mod extract {
    pub use rswind_extractor::*;
}

#[cfg(feature = "build")]
mod build_impl;

#[cfg(feature = "build")]
pub mod build {
    pub use crate::config::StaticUtilityConfig;
    pub use crate::parse::UtilityInput;
}
