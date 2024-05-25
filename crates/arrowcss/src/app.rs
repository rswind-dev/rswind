use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    sync::Arc,
};

use rayon::{iter::IntoParallelIterator, prelude::*};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;
use tracing::{debug, info, instrument};

use crate::{
    config::ArrowConfig,
    context::{Context, GenerateResult},
    css::{Rule, ToCss},
    preset::{preset_tailwind, Preset},
    process::build_group_selector,
    writer::Writer,
};

pub struct Application {
    pub ctx: Arc<Context>,
    // TODO: this is not right, it should store variants' order
    pub seen_variants: BTreeSet<u64>,
    // pub ordering: OrderingMap,
    pub cache: HashMap<SmolStr, Option<String>>,
}

pub struct ApplicationBuilder {
    config: Option<ArrowConfig>,
    ctx: Context,
    presets: Vec<Box<dyn Preset>>,
}

impl ApplicationBuilder {
    #[instrument(skip_all)]
    pub fn with_config(mut self, config: ArrowConfig) -> Self {
        debug!(
            utilities = ?config.utilities.len(),
            static_utilities = ?config.static_utilities.len(),
            dark_mode = ?config.dark_mode,
            theme = ?config.theme,
            "Loaded config"
        );
        self.config = Some(config);
        self
    }

    #[instrument(skip_all)]
    pub fn with_preset(mut self, preset: impl Preset + 'static) -> Self {
        self.presets.push(Box::new(preset));
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
            seen_variants: BTreeSet::default(),
            cache: HashMap::default(),
            // ordering: OrderingMap::new(create_ordering()),
        }
    }
}

type GenResultList<'a> = Vec<GenerateResult<'a>>;

impl Application {
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder { presets: Vec::new(), config: None, ctx: Context::default() }
    }

    #[instrument(skip_all)]
    pub fn run_with<I>(&mut self, input: I) -> String
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let res: GenResultList =
            input.into_iter().filter_map(|token| self.ctx.generate(token.as_ref())).collect();

        info!("Generated {} utilities", res.len());

        Self::run_inner(&mut self.seen_variants, res)
    }

    pub fn run_parallel_with<I>(&mut self, input: I) -> String
    where
        I: IntoParallelIterator,
        I::Item: AsRef<str>,
    {
        let res = input.into_par_iter().filter_map(|s| self.ctx.generate(s.as_ref())).collect();

        Self::run_inner(&mut self.seen_variants, res)
    }

    pub fn run_inner(seen_variants: &mut BTreeSet<u64>, mut res: GenResultList) -> String {
        res.sort_unstable();

        let mut writer = Writer::new(String::with_capacity(1024));

        let mut groups = HashMap::default();
        let mut unique_rules = BTreeMap::default();

        for r in res.iter() {
            if let Some(group) = &r.group {
                groups.entry(*group).or_insert_with(Vec::new).push(r.raw.to_owned());
            }
            if let Some(add) = r.additional_css.as_ref() {
                for css in add.iter() {
                    unique_rules.insert(&css.selector, css);
                }
            }
            let mut w = Writer::new(String::with_capacity(100));
            let _ = r.rule.to_css(&mut w);
            let _ = writer.write_str(&w.dest);
        }

        for (_, css) in unique_rules {
            let _ = css.to_css(&mut writer);
        }

        for (group, names) in groups {
            let _ = Rule::new_with_decls(build_group_selector(names), group.as_decls().into_vec())
                .to_css(&mut writer);
        }

        writer.dest
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
