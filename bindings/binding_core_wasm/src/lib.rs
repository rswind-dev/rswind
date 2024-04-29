use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
};

use arrowcss::{app::Application, create_app, extract::SourceType};
use wasm_bindgen::prelude::*;

mod utils;
extern crate console_error_panic_hook;

#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

lazy_static::lazy_static! {
    static ref APP: Arc<RwLock<Application<'static>>> = Arc::new(RwLock::new(create_app()));
}

#[wasm_bindgen]
pub fn generate(css: String, typ: String) -> String {
    console_error_panic_hook::set_once();

    APP.write()
        .unwrap()
        .run_parallel_with([SourceType::new(css, &typ)], None)
}
