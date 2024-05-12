use arrowcss::{app::Application, create_app as _create_app};
use arrowcss_extractor::{Extractable, Extractor};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct ArrowCss {
    app: Application,
}

impl Default for ArrowCss {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl ArrowCss {
    pub fn new() -> Self {
        ArrowCss { app: _create_app() }
    }

    #[napi]
    pub fn generate(&mut self, css: String, typ: String) -> String {
        self.app.run_with(Extractor::new(&css, &*typ).extract())
    }
}

#[napi]
pub fn create_app() -> ArrowCss {
    ArrowCss::new()
}
