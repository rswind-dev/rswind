use rswind::{
    app::{App, AppInput},
    config::{AppConfig, DEFAULT_CONFIG_PATH},
    generator::{GeneratorWith, ParGenerateWith},
    glob::GlobFilter,
    preset::preset_tailwind,
};
use rswind_extractor::{CollectExtracted, Extractable, Extractor};
use serde::Deserialize;
use serde_json::{from_value, Value};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct RsWindApp(App);

#[napi]
impl RsWindApp {
    #[napi]
    pub fn generate_with(&mut self, candidates: Vec<(String, String)>) -> String {
        candidates
            .iter()
            .map(AppInput::from)
            .glob_filter(&self.0.glob)
            .collect_extracted()
            .par_generate_with(&mut self.0.generator)
    }

    #[napi]
    pub fn generate_contents(&mut self) -> String {
        self.0.generate_contents()
    }

    #[napi]
    pub fn generate_string(
        &mut self,
        input: String,
        #[napi(ts_arg_type = "'html' | 'ecma' | 'unknown'")] kind: Option<String>,
    ) -> String {
        Extractor::new(&input, kind.as_deref().unwrap_or("unknown"))
            .extract()
            .par_generate_with(&mut self.0.generator)
    }

    #[napi]
    pub fn generate_candidate(&mut self, input: Vec<String>) -> String {
        input.generate_with(&mut self.0.generator)
    }
}

#[napi::module_init]
fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RSWIND_LOG"))
        .init();
}

#[napi(object)]
pub struct GeneratorOptions {
    pub base: Option<String>,
    #[napi(ts_type = "string | false | AppConfig")]
    pub config: Option<Value>,
    pub watch: Option<bool>,
    pub parallel: Option<bool>,
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        Self { base: None, config: None, watch: Some(false), parallel: Some(true) }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RswindConfig {
    Path(String),
    Object(AppConfig),
}

#[napi]
pub fn create_app(options: Option<GeneratorOptions>) -> RsWindApp {
    let options = options.unwrap_or_default();
    let config = match options.config {
        Some(Value::String(path)) => AppConfig::from_file(&path).unwrap(),
        Some(obj @ Value::Object(_)) => from_value(obj).unwrap(),
        Some(Value::Bool(false)) => AppConfig::default(),
        _ => AppConfig::from_file(DEFAULT_CONFIG_PATH).unwrap(),
    };

    RsWindApp(
        App::builder()
            .with_preset(preset_tailwind)
            .with_config(config)
            .with_watch(options.watch.unwrap_or(false))
            .with_parallel(options.parallel.unwrap_or(true))
            .with_base(options.base)
            .build()
            .unwrap(),
    )
}
