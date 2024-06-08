use std::{fmt::Write, sync::Arc};

use either::Either::{Left, Right};
use rayon::{iter::IntoParallelIterator, prelude::*};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;
use tracing::{info, instrument};

use crate::{
    app::{Generator, GeneratorBuilder},
    cache::{Cache, CacheState, GeneratorCache},
    context::{CacheKey, Context, GenerateResult},
    css::{Rule, ToCss, ToCssString},
    preset::preset_tailwind,
    process::build_group_selector,
    writer::Writer,
};

pub struct GeneratorProcessor {
    pub ctx: Arc<Context>,
    pub cache: GeneratorCache,
    pub options: GenOptions,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GenOptions {
    pub parallel: bool,
    pub watch: bool,
}

pub type GenResultList = Vec<GenerateResult>;

impl GeneratorProcessor {
    pub fn builder() -> GeneratorBuilder {
        GeneratorBuilder {
            presets: Vec::new(),
            config: None,
            ctx: Context::default(),
            options: GenOptions::default(),
            base: None,
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
                if self.cache.has_seen(s) {
                    return None;
                }
                let res = self.ctx.generate(s);
                if res.is_none() {
                    self.cache.mark_invalid(SmolStr::from(s));
                }
                res
            })
            .collect::<GenResultList>();

        self.generate_css(res)
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

        self.generate_css(valid)
    }

    pub fn generate_css(&mut self, mut res: GenResultList) -> String {
        info!("{} new utilities generated", res.len());

        if !self.cache.state.is_cached() {
            match self.options.parallel {
                true => res.par_sort_unstable(),
                false => res.sort_unstable(),
            }
        }

        let mut writer = Writer::new(String::with_capacity(1024));
        process_result(res, &mut self.cache, &mut writer);

        // During the first run and one shot run, grouped css and additional css will be written here
        // and all css during cached run will be written here
        match self.cache.state {
            CacheState::Cached => {
                Left(self.cache.css().iter().chain(self.cache.extra_css().iter()))
            }
            _ => Right(self.cache.extra_css().iter()),
        }
        .for_each(|(_, css)| {
            let _ = writer.write_str(css);
        });

        self.cache.state.mark_cached();

        writer.dest
    }
}

fn process_result(res: GenResultList, cache: &mut GeneratorCache, writer: &mut Writer<impl Write>) {
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

pub fn create_generator() -> GeneratorProcessor {
    GeneratorBuilder::new().with_preset(preset_tailwind).build_processor().unwrap()
}

pub fn create_app() -> Generator {
    GeneratorBuilder::new().with_preset(preset_tailwind).build().unwrap()
}

pub trait GeneratorWith {
    fn generate_with(self, generator: &mut GeneratorProcessor) -> String;
}

impl<'a, T> GeneratorWith for T
where
    T: IntoIterator + 'a,
    T::Item: AsRef<str>,
{
    fn generate_with(self, generator: &mut GeneratorProcessor) -> String {
        generator.run_with(self)
    }
}

pub trait ParGenerateWith {
    fn par_generate_with(self, generator: &mut GeneratorProcessor) -> String;
}

impl<'a, T> ParGenerateWith for T
where
    T: IntoParallelIterator + 'a,
    T::Item: AsRef<str>,
{
    fn par_generate_with(self, generator: &mut GeneratorProcessor) -> String {
        generator.run_parallel_with(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application() {
        let mut app = create_generator();

        println!("{}", app.run_with(["flex", "flex-col"]));
    }
}
