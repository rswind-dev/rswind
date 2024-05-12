use std::{collections::BTreeSet, fmt::Write as _, sync::Arc};

use cssparser::serialize_name;
use rayon::{iter::IntoParallelIterator, prelude::*};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;

use crate::{
    config::ArrowConfig,
    context::{Context, GenerateResult},
    css::{Rule, ToCss},
    ordering::{create_ordering, OrderingItem, OrderingMap},
    preset::load_preset,
    writer::Writer,
};

pub struct Application {
    pub ctx: Arc<Context>,
    // TODO: this is not right, it should store variants' order
    pub seen_variants: BTreeSet<u64>,
    pub ordering: OrderingMap,
    pub cache: HashMap<SmolStr, Option<String>>,
    pub strict_mode: bool,
}

pub struct UninitializedApp {
    ctx: Context,
    seen_variants: BTreeSet<u64>,
    strict_mode: bool,
}

impl UninitializedApp {
    pub fn init(mut self) -> Application {
        load_preset(&mut self.ctx);
        Application {
            ctx: Arc::new(self.ctx),
            seen_variants: self.seen_variants,
            cache: HashMap::default(),
            ordering: OrderingMap::new(create_ordering()),
            strict_mode: self.strict_mode,
        }
    }
}

type GenResult = HashMap<SmolStr, GenerateResult>;

impl Application {
    pub fn builder(config: ArrowConfig) -> UninitializedApp {
        UninitializedApp {
            // TODO: add theme back
            ctx: Context::new(),
            seen_variants: BTreeSet::new(),
            strict_mode: config.features.strict_mode,
        }
    }

    pub fn run_with(&mut self, input: impl IntoIterator<Item: AsRef<str>>) -> String {
        let res = input
            .into_iter()
            .filter_map(|token| {
                self.ctx
                    .generate(token.as_ref())
                    .map(|rule| (SmolStr::from(token.as_ref()), rule))
            })
            .collect();
        self.run_inner(res)
    }

    pub fn run_parallel_with(
        &mut self,
        input: impl IntoParallelIterator<Item: AsRef<str>>,
    ) -> String {
        let res = input
            .into_par_iter()
            .filter_map(|s| {
                self.ctx
                    .generate(s.as_ref())
                    .map(|rule| (SmolStr::from(s.as_ref()), rule))
            })
            .collect();
        self.run_inner(res)
    }

    pub fn run_inner(&mut self, res: GenResult) -> String {
        let mut writer = Writer::default(String::with_capacity(1024));
        let mut groups = HashMap::default();
        for (name, v) in res.iter() {
            self.seen_variants.extend(v.variants.clone());
            if let Some(group) = &v.group {
                groups
                    .entry(*group)
                    .or_insert_with(Vec::new)
                    .push(name.to_owned());
            }
        }

        let get_key = |r: &GenerateResult| {
            r.variants
                .iter()
                .map(|v| self.seen_variants.iter().position(|x| x == v).unwrap())
                .fold(0u128, |order, o| order | (1 << o))
        };

        self.ordering.insert_many(res.into_iter().map(|r| {
            let key = get_key(&r.1);
            OrderingItem::new(r.0, r.1, key)
        }));

        for r in self.ordering.get_ordered() {
            let mut w = Writer::default(String::with_capacity(100));
            let _ = r.item.rule.to_css(&mut w);
            let _ = writer.write_str(&w.dest);
            // self.ctx.cache.insert(r.name.clone(), Some(w.dest));
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
                group.as_decls(),
            );
            let _ = rule.to_css(&mut writer);
        }

        writer.dest
    }
}

pub fn create_app() -> Application {
    let config = ArrowConfig::default();
    let app = Application::builder(config);
    app.init()
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
