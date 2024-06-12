use std::sync::{Arc, RwLock};

use rswind::{create_processor, processor::GeneratorProcessor};
use rswind_extractor::{Extractable, Extractor};
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

lazy_static::lazy_static! {
    static ref APP: Arc<RwLock<GeneratorProcessor>> = Arc::new(RwLock::new(create_processor()));
}

#[wasm_bindgen]
pub fn generate(css: String, typ: String) -> String {
    APP.write().unwrap().run_with(Extractor::new(&css, &*typ).extract()).css.as_str().to_string()
}

#[wasm_bindgen(js_name = generateWith)]
pub fn generate_with(candidates: Vec<String>) -> String {
    APP.write().unwrap().run_with(candidates).css.as_str().to_string()
}
