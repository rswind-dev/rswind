use rswind::{
    generator::GeneratorBuilder, preset::preset_tailwind, processor::GeneratorProcessor,
    GeneratorConfig,
};
use rswind_extractor::{Extractable, Extractor};
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Generator {
    processor: GeneratorProcessor,
}

#[wasm_bindgen(js_name = createGenerator)]
pub fn create_generator(config: Option<GeneratorConfig>) -> Result<Generator, JsError> {
    Generator::new(config)
}

#[wasm_bindgen]
impl Generator {
    #[wasm_bindgen(constructor)]
    pub fn new(config: Option<GeneratorConfig>) -> Result<Generator, JsError> {
        Ok(Generator {
            processor: GeneratorBuilder::new()
                .with_parallel(false)
                .with_preset(preset_tailwind)
                .with_config(config.unwrap_or_default())
                .build_processor()?,
        })
    }

    #[wasm_bindgen]
    pub fn generate(&mut self, css: String, typ: String) -> String {
        self.processor.run_with(Extractor::new(&css, &*typ).extract()).css.as_str().to_owned()
    }

    #[wasm_bindgen(js_name = generateWith)]
    pub fn generate_with(&mut self, candidates: Vec<String>) -> String {
        self.processor.run_with(candidates).css.as_str().to_owned()
    }
}
