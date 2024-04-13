use arrowcss::{
    app::Application, create_app as _create_app, parser::to_css_rule,
    writer::write_to_string,
};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct ArrowCss {
    app: Application<'static>,
}

#[napi]
impl ArrowCss {
    pub fn new() -> Self {
        ArrowCss { app: _create_app() }
    }

    #[napi]
    pub fn generate(&self, css: String) -> String {
        let rule = to_css_rule(&css, &self.app.ctx).unwrap();
        write_to_string(rule)
    }
}

#[napi]
pub fn create_app() -> ArrowCss {
    ArrowCss::new()
}
