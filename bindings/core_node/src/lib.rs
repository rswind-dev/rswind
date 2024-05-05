use arrowcss::{app::Application, create_app as _create_app, extract::SourceInput};

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
        self.app.run(SourceInput::new(&css, &*typ))
    }
}

#[napi]
pub fn create_app() -> ArrowCss {
    ArrowCss::new()
}
