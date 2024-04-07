#![feature(trait_alias)]
#![feature(control_flow_enum)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(pattern)]

pub mod config;
pub mod context;
pub mod css;
pub mod macros;
pub mod parser;
pub mod rule;
pub mod rules;
pub mod theme;
pub mod themes;
pub mod utility;
pub mod utils;
// pub mod variant;
pub mod app;
pub mod common;
pub mod types;
pub mod variant;
pub mod writer;

pub fn generate(_input: String) -> String {
    _input
}
