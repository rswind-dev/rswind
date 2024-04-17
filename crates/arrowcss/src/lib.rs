#![feature(pattern)]

use app::Application;

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
pub mod types;
pub mod variant;
pub mod ordering;
pub mod writer;

pub fn create_app() -> Application<'static> {
    let mut app = Application::new().unwrap();
    app.init();
    app
}
