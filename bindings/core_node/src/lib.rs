use rswind::{app::Application, create_app as _create_app};
use rswind_extractor::{Extractable, Extractor};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct RsWindApp {
    app: Application,
}

impl Default for RsWindApp {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl RsWindApp {
    pub fn new() -> Self {
        RsWindApp { app: _create_app() }
    }

    #[napi]
    pub fn generate(&mut self, css: String, typ: String) -> String {
        self.app.run_with(Extractor::new(&css, &*typ).extract())
    }
}

#[napi]
pub fn create_app() -> RsWindApp {
    RsWindApp::new()
}
