use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    cache::{CacheState, GeneratorCache},
    common::{LoadPreset, LoadTheme},
    config::{GeneratorConfig, GeneratorConfigError},
    glob::{BuildGlobError, GlobMatcher, MaybeParallelGlobFilter},
    io::{walk, FileInput},
    process::ThemeParseError,
    processor::{GenOptions, GenerateResult, GeneratorProcessor, ParGenerateWith},
    DesignSystem,
};
use rswind_common::iter::prelude::*;
use rswind_extractor::{Extractor, MaybeParCollectExtracted};

use rswind_theme::Theme;
use thiserror::Error;
use tracing::instrument;

pub struct Generator {
    pub processor: GeneratorProcessor,
    pub glob: GlobMatcher,
}

#[derive(Default)]
pub struct GeneratorBuilder {
    pub(crate) config: Option<GeneratorConfig>,
    pub(crate) design: DesignSystem,
    pub(crate) presets: Vec<Box<dyn LoadPreset>>,
    pub(crate) themes: Vec<Box<dyn LoadTheme>>,
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
    #[error("{0}")]
    ConfigError(#[from] GeneratorConfigError),
}

#[cfg(feature = "napi")]
impl From<AppBuildError> for napi::Error {
    fn from(err: AppBuildError) -> Self {
        napi::Error::new(napi::Status::GenericFailure, err.to_string())
    }
}

impl GeneratorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[instrument(skip_all)]
    pub fn with_config(mut self, config: GeneratorConfig) -> Self {
        self.config = Some(config);
        self
    }

    #[instrument(skip_all)]
    pub fn with_preset(mut self, preset: impl LoadPreset + 'static) -> Self {
        self.presets.push(Box::new(preset));
        self
    }

    #[instrument(skip_all)]
    pub fn with_theme(mut self, preset: impl LoadTheme + 'static) -> Self {
        self.themes.push(Box::new(preset));
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
    pub fn build_processor(mut self) -> Result<GeneratorProcessor, AppBuildError> {
        // Step 1: load theme
        for theme in self.themes.drain(..) {
            theme.load_theme(&mut self.design);
        }

        if let Some(ref mut config) = self.config {
            self.design.theme.merge(&mut config.theme);
            for utility in config.utilities.drain(..) {
                utility
                    .parse(&self.design.theme)
                    .map(|(key, utility)| self.design.utilities.add(key, utility))?;
            }

            for (key, value) in config.static_utilities.drain() {
                self.design.add_static(key, value);
            }
        }

        for preset in self.presets.drain(..) {
            preset.load_preset(&mut self.design);
        }

        Ok(GeneratorProcessor {
            design: Arc::new(self.design),
            cache: GeneratorCache::new(match self.options.watch {
                true => CacheState::FirstRun,
                false => CacheState::OneShot,
            }),
            options: self.options,
        })
    }

    #[instrument(skip_all)]
    pub fn build(mut self) -> Result<Generator, AppBuildError> {
        let base = self.base.take().map_or(env::current_dir()?, PathBuf::from);

        let glob = match self.config {
            Some(ref mut config) if !config.content.is_empty() => {
                GlobMatcher::new(config.content.drain(..), base)?
            }
            _ => GlobMatcher::default_glob(base)?,
        };

        let processor = self.build_processor()?;

        Ok(Generator { processor, glob })
    }
}

pub struct GeneratorInput<'a> {
    path: &'a str,
    content: &'a str,
}

impl AsRef<Path> for GeneratorInput<'_> {
    fn as_ref(&self) -> &Path {
        Path::new(self.path)
    }
}

impl<'a> GeneratorInput<'a> {
    pub fn new(path: &'a str, content: &'a str) -> Self {
        Self { path, content }
    }
}

impl<'a> From<(&'a str, &'a str)> for GeneratorInput<'a> {
    fn from((path, content): (&'a str, &'a str)) -> Self {
        Self { path, content }
    }
}

impl<'a> From<&'a (String, String)> for GeneratorInput<'a> {
    fn from(it: &'a (String, String)) -> Self {
        Self { path: it.0.as_str(), content: it.1.as_str() }
    }
}

impl<'a> From<GeneratorInput<'a>> for Extractor<'a> {
    fn from(input: GeneratorInput<'a>) -> Self {
        Extractor::new(input.content, get_extension(input.path))
    }
}

impl Generator {
    pub fn builder() -> GeneratorBuilder {
        GeneratorBuilder::new()
    }

    pub fn theme(&self) -> &Theme {
        &self.processor.design.theme
    }

    pub fn base(&self) -> &Path {
        self.glob.base()
    }

    pub fn generate_contents(&mut self) -> GenerateResult {
        walk(self.base())
            .into_iter_with(IntoIterKind::Parallel)
            .glob_filter(&self.glob)
            .map(FileInput::from_file)
            .collect::<Vec<_>>()
            .iter_with(IntoIterKind::Parallel)
            .map(GeneratorInput::from)
            .collect_extracted()
            .par_generate_with(&mut self.processor)
    }
}

fn get_extension(path: &str) -> &str {
    Path::new(path).extension().unwrap_or_default().to_str().unwrap_or_default()
}
