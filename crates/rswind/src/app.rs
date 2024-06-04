use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    cache::{CacheState, GeneratorCache},
    config::AppConfig,
    generator::{GenOptions, Generator, ParGenerateWith},
    glob::{BuildGlobError, GlobMatcher, MaybeParallelGlobFilter},
    io::{walk, FileInput},
    preset::Preset,
    process::ThemeParseError,
    Context,
};
use rswind_common::iter::prelude::*;
use rswind_extractor::{Extractor, MaybeParCollectExtracted};

use thiserror::Error;
use tracing::instrument;

pub struct App {
    pub generator: Generator,
    pub glob: GlobMatcher,
}

#[derive(Default)]
pub struct AppBuilder {
    pub(crate) config: Option<AppConfig>,
    pub(crate) ctx: Context,
    pub(crate) presets: Vec<Box<dyn Preset>>,
    pub(crate) options: GenOptions,
    pub(crate) base: Option<String>,
}

#[derive(Debug, Error)]
pub enum AppBuildError {
    #[error("Failed to build glob: {0}")]
    GlobError(#[from] BuildGlobError),
    #[error("Failed to parse utility: {0}")]
    UtilityParingError(#[from] ThemeParseError),
    #[error("Io Error during building app: {0}")]
    IoError(#[from] std::io::Error),
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[instrument(skip_all)]
    pub fn with_config(mut self, config: AppConfig) -> Self {
        self.config = Some(config);
        self
    }

    #[instrument(skip_all)]
    pub fn with_preset(mut self, preset: impl Preset + 'static) -> Self {
        self.presets.push(Box::new(preset));
        self
    }

    pub fn with_base(mut self, base: Option<String>) -> Self {
        self.base = base;
        self
    }

    pub fn with_watch(mut self, watch: bool) -> Self {
        self.options.watch = watch;
        self
    }

    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.options.parallel = parallel;
        self
    }

    #[instrument(skip_all)]
    pub fn build_generator(mut self) -> Result<Generator, AppBuildError> {
        for preset in self.presets.drain(..) {
            preset.load_preset(&mut self.ctx);
        }

        if let Some(ref mut config) = self.config {
            for utility in config.utilities.drain(..) {
                utility
                    .parse(&self.ctx.theme)
                    .map(|(key, utility)| self.ctx.utilities.add(key, utility))?;
            }

            for (key, value) in config.static_utilities.drain() {
                self.ctx.add_static(key, value);
            }
        }

        Ok(Generator {
            ctx: Arc::new(self.ctx),
            cache: GeneratorCache::new(match self.options.watch {
                true => CacheState::FirstRun,
                false => CacheState::OneShot,
            }),
            options: self.options,
        })
    }

    #[instrument(skip_all)]
    pub fn build(mut self) -> Result<App, AppBuildError> {
        let base = self.base.take().map_or(env::current_dir()?, PathBuf::from);

        let glob = match self.config {
            Some(ref mut config) if !config.content.is_empty() => {
                GlobMatcher::new(config.content.drain(..), base)?
            }
            _ => GlobMatcher::default_glob(base)?,
        };

        let generator = self.build_generator()?;

        Ok(App { generator, glob })
    }
}

pub struct AppInput<'a> {
    path: &'a str,
    content: &'a str,
}

impl AsRef<Path> for AppInput<'_> {
    fn as_ref(&self) -> &Path {
        Path::new(self.path)
    }
}

impl<'a> AppInput<'a> {
    pub fn new(path: &'a str, content: &'a str) -> Self {
        Self { path, content }
    }
}

impl<'a> From<(&'a str, &'a str)> for AppInput<'a> {
    fn from((path, content): (&'a str, &'a str)) -> Self {
        Self { path, content }
    }
}

impl<'a> From<&'a (String, String)> for AppInput<'a> {
    fn from(it: &'a (String, String)) -> Self {
        Self { path: it.0.as_str(), content: it.1.as_str() }
    }
}

impl<'a> From<AppInput<'a>> for Extractor<'a> {
    fn from(input: AppInput<'a>) -> Self {
        Extractor::new(input.content, get_extension(input.path))
    }
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn base(&self) -> &Path {
        self.glob.base()
    }

    pub fn generate_contents(&mut self) -> String {
        walk(self.base())
            .into_iter_with(IntoIterKind::Parallel)
            .glob_filter(&self.glob)
            .map(FileInput::from_file)
            .collect::<Vec<_>>()
            .iter_with(IntoIterKind::Parallel)
            .map(AppInput::from)
            .collect_extracted()
            .par_generate_with(&mut self.generator)
    }
}

fn get_extension(path: &str) -> &str {
    Path::new(path).extension().unwrap_or_default().to_str().unwrap_or_default()
}
