use std::fmt::Write as _;

use cssparser::serialize_name;
use fxhash::FxHashMap as HashMap;
use rayon::prelude::*;
use smol_str::{format_smolstr, SmolStr};

use crate::{
    config::ArrowConfig,
    context::Context,
    css::{Rule, ToCss},
    extract::{Extractor, SourceInput},
    ordering::{create_ordering, OrderingItem, OrderingMap},
    parser::{to_css_rule, GenerateResult},
    preset::load_preset,
    writer::Writer,
};

pub struct Application {
    pub ctx: Context,
    // pub cache: String,
    pub strict_mode: bool,
}

impl Application {
    pub fn new(config: ArrowConfig) -> Self {
        Self {
            ctx: Context::new(config.theme),
            // cache: String::new(),
            strict_mode: config.features.strict_mode,
        }
    }

    pub fn init(&mut self) -> &mut Self {
        load_preset(&mut self.ctx);
        self
    }

    pub fn run<T: AsRef<str>>(&mut self, input: SourceInput<T>) -> String {
        let res = input
            .extract()
            .filter_map(|token| {
                to_css_rule(token, &self.ctx).map(|rule| (SmolStr::from(token), rule))
            })
            .collect::<HashMap<SmolStr, GenerateResult>>();
        self.run_inner(res)
    }

    pub fn run_parallel<T: AsRef<str>>(
        &mut self,
        input: impl IntoParallelIterator<Item: AsRef<SourceInput<T>>>,
    ) -> String {
        let res = input
            .into_par_iter()
            .map(|x| {
                x.as_ref()
                    .extract()
                    .filter_map(|token| {
                        to_css_rule(token, &self.ctx).map(|rule| (SmolStr::from(token), rule))
                    })
                    .collect::<HashMap<SmolStr, GenerateResult>>()
            })
            .reduce(HashMap::default, |mut a, b| {
                a.extend(b);
                a
            });
        self.run_inner(res)
    }

    pub fn run_inner(&mut self, res: HashMap<SmolStr, GenerateResult>) -> String {
        let mut writer = Writer::default(String::with_capacity(1024));
        let mut groups = HashMap::default();
        for (name, v) in res.iter() {
            self.ctx.seen_variants.extend(v.variants.clone());
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
                .map(|v| self.ctx.seen_variants.iter().position(|x| x == v).unwrap())
                .fold(0u128, |order, o| order | (1 << o))
        };

        let ordering = create_ordering();

        let mut om = OrderingMap::new(ordering);
        om.insert_many(res.into_iter().map(|r| {
            let key = get_key(&r.1);
            OrderingItem::new(r.0, r.1, key)
        }));

        for r in om.get_ordered() {
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
                        format_smolstr!(".{}", {
                            let mut w = String::new();
                            serialize_name(s, &mut w).unwrap();
                            w
                        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_app;

    #[test]
    fn test_application() {
        let mut app = create_app();
        let input = SourceInput::new(r#"<div class="flex">"#, "html");
        let res = app.run_parallel([input]);

        println!("{}", res);
    }
}
