use std::sync::{Arc, RwLock};

use arrowcss::{app::Application, create_app, extract::SourceInput};
use wasm_bindgen::prelude::*;

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
    APP.write().unwrap().run(SourceInput::new(&css, &*typ))
}
