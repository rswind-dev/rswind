use arrowcss::{app::Application, create_app as _create_app};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct ArrowCss {
    app: Application<'static>,
}

#[napi]
impl Default for ArrowCss {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrowCss {
    pub fn new() -> Self {
        ArrowCss { app: _create_app() }
    }

    #[napi]
    pub fn generate(&mut self, css: String) -> String {
        self.app.run_parallel_with([css], None)
    }
}

#[napi]
pub fn create_app() -> ArrowCss {
    ArrowCss::new()
}
