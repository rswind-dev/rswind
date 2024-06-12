use std::rc::Rc;

use rswind::{
    config::{GeneratorConfig, DEFAULT_CONFIG_PATH},
    generator::{self, GeneratorInput},
    glob::GlobFilter,
    preset::preset_tailwind,
    processor::{self, GeneratorWith, ParGenerateWith},
};
use rswind_extractor::{CollectExtracted, Extractable, Extractor};
use serde::Deserialize;
use serde_json::{from_value, Value};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Generator(generator::Generator);

#[napi(object)]
pub struct GenerateResult {
    pub css: Rc<String>,
    pub kind: ResultKind,
}

#[napi(string_enum)]
pub enum ResultKind {
    Cached,
    Generated,
}

impl From<processor::GenerateResult> for GenerateResult {
    fn from(result: processor::GenerateResult) -> Self {
        Self { css: result.css, kind: result.kind.into() }
    }
}

impl From<processor::ResultKind> for ResultKind {
    fn from(kind: processor::ResultKind) -> Self {
        match kind {
            processor::ResultKind::Cached => Self::Cached,
            processor::ResultKind::Generated => Self::Generated,
        }
    }
}

#[napi]
impl Generator {
    #[napi]
    pub fn generate_with(&mut self, candidates: Vec<(String, String)>) -> GenerateResult {
        candidates
            .iter()
            .map(GeneratorInput::from)
            .glob_filter(&self.0.glob)
            .collect_extracted()
            .par_generate_with(&mut self.0.processor)
            .into()
    }

    #[napi]
    pub fn generate(&mut self) -> Rc<String> {
        self.0.generate_contents().css
    }

    #[napi]
    pub fn generate_string(
        &mut self,
        input: String,
        #[napi(ts_arg_type = "'html' | 'ecma' | 'unknown'")] kind: Option<String>,
    ) -> GenerateResult {
        Extractor::new(&input, kind.as_deref().unwrap_or("unknown"))
            .extract()
            .par_generate_with(&mut self.0.processor)
            .into()
    }

    #[napi]
    pub fn generate_candidate(&mut self, input: Vec<String>) -> GenerateResult {
        input.generate_with(&mut self.0.processor).into()
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
    #[napi(ts_type = "string | false | GeneratorConfig")]
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
    Object(GeneratorConfig),
}

#[napi]
pub fn create_generator(options: Option<GeneratorOptions>) -> Generator {
    let options = options.unwrap_or_default();
    let config = match options.config {
        Some(Value::String(path)) => GeneratorConfig::from_file(&path).unwrap(),
        Some(obj @ Value::Object(_)) => from_value(obj).unwrap(),
        Some(Value::Bool(false)) => GeneratorConfig::default(),
        _ => GeneratorConfig::from_file(DEFAULT_CONFIG_PATH).unwrap(),
    };

    Generator(
        generator::Generator::builder()
            .with_preset(preset_tailwind)
            .with_config(config)
            .with_watch(options.watch.unwrap_or(false))
            .with_parallel(options.parallel.unwrap_or(true))
            .with_base(options.base)
            .build()
            .unwrap(),
    )
}
