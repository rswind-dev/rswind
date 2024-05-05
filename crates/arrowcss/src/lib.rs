#![feature(pattern)]
#![recursion_limit = "256"]

use app::Application;
use config::ArrowConfig;

pub mod config;
pub mod context;
pub mod css;
pub mod macros;
pub mod parser;
pub mod parsing;
pub mod process;
pub mod rules;
pub mod theme;
pub mod themes;
pub mod utils;
// pub mod variant;
pub mod app;
pub mod common;
pub mod extract;
pub mod ordering;
pub mod types;
pub mod variant;
pub mod writer;

pub fn create_app() -> Application {
    let config = ArrowConfig::default();
    let mut app = Application::new(config);
    app.init();
    app
}
