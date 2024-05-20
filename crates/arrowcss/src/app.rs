use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    sync::Arc,
};

use cssparser::serialize_name;
use rayon::{iter::IntoParallelIterator, prelude::*};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;
use tracing::{debug, info, instrument};

use crate::{
    config::ArrowConfig,
    context::{Context, GenerateResult},
    css::{Rule, ToCss},
    preset::{preset_tailwind, Preset},
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

type GenResult<'a> = Vec<(SmolStr, GenerateResult<'a>)>;

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
        let res: GenResult = input
            .into_iter()
            .filter_map(|token| {
                self.ctx.generate(token.as_ref()).map(|rule| (SmolStr::from(token.as_ref()), rule))
            })
            .collect();

        info!("Generated {} utilities", res.len());

        Self::run_inner(&mut self.seen_variants, res)
    }

    pub fn run_parallel_with<I>(&mut self, input: I) -> String
    where
        I: IntoParallelIterator,
        I::Item: AsRef<str>,
    {
        let res = input
            .into_par_iter()
            .filter_map(|s| {
                self.ctx.generate(s.as_ref()).map(|rule| (SmolStr::from(s.as_ref()), rule))
            })
            .collect();

        Self::run_inner(&mut self.seen_variants, res)
    }

    pub fn run_inner(seen_variants: &mut BTreeSet<u64>, mut res: GenResult) -> String {
        let mut writer = Writer::default(String::with_capacity(1024));
        let mut groups = HashMap::default();
        for (name, v) in res.iter() {
            seen_variants.extend(v.variants.clone());
            if let Some(group) = &v.group {
                groups.entry(*group).or_insert_with(Vec::new).push(name.to_owned());
            }
        }

        let get_key = |r: &GenerateResult| {
            r.variants
                .iter()
                .map(|v| seen_variants.iter().position(|x| x == v).unwrap())
                .fold(0u128, |order, o| order | (1 << o))
        };

        res.sort_by_cached_key(|(k, v)| {
            // variant first > ordering key > name
            (get_key(v), v.ordering, k.clone())
        });

        for (_, r) in res.iter() {
            let mut w = Writer::default(String::with_capacity(100));
            let _ = r.rule.to_css(&mut w);
            let _ = writer.write_str(&w.dest);
        }

        let unique_rules = res
            .iter()
            .filter_map(|(_, r)| r.additional_css.as_ref())
            .flat_map(|r| r.iter())
            .map(|r| (&r.selector, r))
            .collect::<BTreeMap<_, _>>();

        for (_, css) in unique_rules {
            let _ = css.to_css(&mut writer);
        }

        for (group, names) in groups {
            let rule = Rule::new_with_decls(
                names
                    .iter()
                    .map(|s| {
                        let mut w = String::from(".");
                        serialize_name(s, &mut w).unwrap();
                        w
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                group.as_decls().into_vec(),
            );
            let _ = rule.to_css(&mut writer);
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
