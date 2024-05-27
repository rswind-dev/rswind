use std::{fmt::Write, sync::Arc};

use either::Either::{Left, Right};
use rayon::{iter::IntoParallelIterator, prelude::*};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;
use tracing::{info, instrument};

use crate::{
    cache::{AppCache, Cache, CacheState},
    config::ArrowConfig,
    context::{CacheKey, Context, GenerateResult},
    css::{Rule, ToCss, ToCssString},
    preset::{preset_tailwind, Preset},
    process::build_group_selector,
    writer::Writer,
};

pub struct Application {
    pub ctx: Arc<Context>,
    pub cache: AppCache,
    pub options: AppOptions,
}

#[derive(Debug, Clone, Default)]
pub struct AppOptions {
    pub parallel: bool,
    pub watch: bool,
}

pub struct ApplicationBuilder {
    config: Option<ArrowConfig>,
    ctx: Context,
    presets: Vec<Box<dyn Preset>>,
    options: AppOptions,
}

impl ApplicationBuilder {
    #[instrument(skip_all)]
    pub fn with_config(mut self, config: ArrowConfig) -> Self {
        self.config = Some(config);
        self
    }

    #[instrument(skip_all)]
    pub fn with_preset(mut self, preset: impl Preset + 'static) -> Self {
        self.presets.push(Box::new(preset));
        self
    }

    pub fn watch(mut self) -> Self {
        self.options.watch = true;
        self
    }

    pub fn parallel(mut self) -> Self {
        self.options.parallel = true;
        self
    }

    #[instrument(skip_all)]
    pub fn build(mut self) -> Application {
        for preset in self.presets {
            preset.load_preset(&mut self.ctx);
        }

        if let Some(config) = self.config.take() {
            for utility in config.utilities {
                match utility.parse(&self.ctx.theme) {
                    Ok((key, utility)) => {
                        self.ctx.utilities.add(key, utility);
                    }
                    Err(e) => {
                        eprintln!("Error parsing utility: {:?}", e);
                    }
                }
            }

            for (key, value) in config.static_utilities {
                self.ctx.add_static(key, value);
            }
        }

        Application {
            ctx: Arc::new(self.ctx),
            cache: AppCache::new(match self.options.watch {
                true => CacheState::FirstRun,
                false => CacheState::OneShot,
            }),
            options: self.options,
        }
    }
}

pub type GenResultList = Vec<GenerateResult>;

impl Application {
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder {
            presets: Vec::new(),
            config: None,
            ctx: Context::default(),
            options: AppOptions::default(),
        }
    }

    #[instrument(skip_all)]
    pub fn run_with<I>(&mut self, input: I) -> String
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let res = input
            .into_iter()
            .filter_map(|s| {
                let s = s.as_ref();
                if !self.cache.has_seen(s) {
                    return None;
                }
                let res = self.ctx.generate(s);
                if res.is_none() {
                    self.cache.mark_invalid(SmolStr::from(s));
                }
                res
            })
            .collect::<GenResultList>();

        Self::generate_css(&mut self.cache, res)
    }

    pub fn run_parallel_with<I>(&mut self, input: I) -> String
    where
        I: IntoParallelIterator,
        I::Item: AsRef<str>,
    {
        let (invalid, valid): (Vec<_>, Vec<_>) =
            input.into_par_iter().filter(|s| !self.cache.has_seen(s.as_ref())).partition_map(|s| {
                let s = s.as_ref();
                match self.ctx.generate(s) {
                    Some(r) => Right(r),
                    None => Left(SmolStr::from(s)),
                }
            });
        self.cache.mark_invalid_many(invalid);

        Self::generate_css(&mut self.cache, valid)
    }

    pub fn generate_css(cache: &mut AppCache, mut res: GenResultList) -> String {
        info!("{} new utilities generated", res.len());

        if !cache.state.is_cached() {
            res.par_sort_unstable();
        }

        let mut writer = Writer::new(String::with_capacity(1024));
        process_result(res, cache, &mut writer);

        // During the first run and one shot run, grouped css and additional css will be written here
        // and all css during cached run will be written here
        match cache.state {
            CacheState::Cached => Left(cache.css().iter().chain(cache.extra_css().iter())),
            _ => Right(cache.extra_css().iter()),
        }
        .for_each(|(_, css)| {
            let _ = writer.write_str(css);
        });

        cache.state.mark_cached();

        writer.dest
    }
}

fn process_result(res: GenResultList, cache: &mut AppCache, writer: &mut Writer<impl Write>) {
    let mut groups = HashMap::default();

    for mut r in res.into_iter() {
        if let Some(group) = &r.group {
            groups.entry(*group).or_insert_with(Vec::new).push(r.raw.to_owned());
        }

        if let Some(add) = r.additional_css.take() {
            // Even oneshot run, we still need to write additional css to "cache",
            // for remove duplicates and sort them
            for css in add.iter() {
                cache.store_extra_css(
                    CacheKey::new_property(css.selector.clone()),
                    css.to_css_string(),
                );
            }
        }

        cache.mark_valid(r.raw.clone());

        match cache.state {
            CacheState::FirstRun | CacheState::Cached => {
                let mut w = Writer::new(String::with_capacity(64));
                let _ = r.rule.to_css(&mut w);

                // If it's first run, we need directly write to writer
                // so we can avoid iterate the BTreeMap again
                if cache.state == CacheState::FirstRun {
                    let _ = writer.write_str(&w.dest);
                }

                cache.store_css(CacheKey::from(r), w.dest);
            }
            CacheState::OneShot => {
                // one shot run, we don't need to cache the css, just write to writer
                let _ = r.rule.to_css(writer);
            }
        }
    }

    for (group, names) in groups {
        let selector = build_group_selector(names);
        cache.store_extra_css(
            CacheKey::new_grouped(SmolStr::from(&selector)),
            Rule::new_with_decls(selector, group.as_decls().into_vec()).to_css_string(),
        );
    }
}

pub fn create_app() -> Application {
    Application::builder().with_preset(preset_tailwind).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application() {
        let mut app = create_app();

        println!("{}", app.run_with(["flex", "flex-col"]));
    }
}
