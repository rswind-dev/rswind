use std::{
    fmt::Write as _,
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::mpsc,
    time::{Duration, Instant},
};

use config::{Config, File};
use cssparser::serialize_name;
use fxhash::FxHashMap as HashMap;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::{
    common::ScopeFunctions,
    config::ArrowConfig,
    context::Context,
    css::{Rule, ToCss},
    extract::{BasicExtractor, Extractor, SourceType},
    ordering::{create_ordering, OrderingItem, OrderingMap},
    parser::{to_css_rule, GenerateResult},
    rules::{dynamics::load_dynamic_utilities, statics::load_static_utilities},
    variant::load_variants,
    writer::Writer,
};

pub struct Application<'c> {
    pub ctx: Context<'c>,
    pub writer: Writer<'c, String>,
    pub buffer: String,
    pub cache: String,
    pub strict_mode: bool,
}

impl<'c> Application<'c> {
    pub fn new() -> Result<Self, config::ConfigError> {
        // TODO: temporarily disable this because of the wasm build
        // let config = Config::builder()
        //     .add_source(File::with_name("arrow.config"))
        //     .build()
        //     .map(|c| c.try_deserialize::<ArrowConfig>())
        //     .unwrap_or_else(|_| Ok(ArrowConfig::default()))?;
        let config = ArrowConfig::default();

        let w = String::new();
        let writer = Writer::default(w);

        Ok(Self {
            ctx: Context::new(config.theme),
            writer,
            buffer: String::new(),
            cache: String::new(),
            strict_mode: config.features.strict_mode,
        })
    }

    pub fn init(&mut self) -> &mut Self {
        load_static_utilities(&mut self.ctx);
        load_dynamic_utilities(&mut self.ctx);
        load_variants(&mut self.ctx);
        self
    }

    pub fn watch(&mut self, dir: &str, output: Option<&str>) {
        let (tx, rx) = mpsc::channel();

        let mut debouncer = new_debouncer(Duration::from_millis(0), tx).unwrap();

        debouncer
            .watcher()
            .watch(Path::new(dir), RecursiveMode::NonRecursive)
            .unwrap();

        let strict_mode = self.strict_mode;
        let files = get_files(dir)
            .into_par_iter()
            .map(|f| SourceType::from_file(&f).run_if(strict_mode, |s| s.as_unknown()));

        self.run_parallel_with(files, output);

        for change in rx {
            self.run_parallel_with(
                change.unwrap().into_par_iter().map(|f| {
                    SourceType::from_file(&f.path).run_if(strict_mode, |s| s.as_unknown())
                }),
                output,
            );
        }
    }

    pub fn run_parallel(&mut self, path: impl AsRef<Path>, output: Option<&str>) -> String {
        self.run_parallel_with(
            get_files(path.as_ref())
                .par_iter()
                .map(|f| SourceType::from_file(&f)),
            output,
        )
    }

    pub fn run_parallel_with(
        &mut self,
        input: impl IntoParallelIterator<Item = SourceType>,
        output: Option<&str>,
    ) -> String {
        // let start = Instant::now();
        self.writer.dest.clear();
        let res = input
            .into_par_iter()
            .map(|x| {
                x.extract()
                    .into_iter()
                    .filter_map(|token| {
                        to_css_rule(token, &self.ctx).map(|rule| (token.to_owned(), rule))
                    })
                    .collect::<HashMap<String, GenerateResult>>()
            })
            .reduce(HashMap::default, |mut a, b| {
                a.extend(b);
                a
            });

        let mut groups = HashMap::default();
        for (name, v) in res.iter() {
            self.ctx.seen_variants.extend(v.variants.clone());
            if let Some(group) = &v.group {
                groups
                    .entry(group.clone())
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
        let res_len = res.len();

        let mut om = OrderingMap::new(ordering);
        om.insert_many(res.into_iter().map(|r| {
            let key = get_key(&r.1);
            OrderingItem::new(r.0, r.1, key)
        }));

        for r in om.get_ordered() {
            let mut w = String::with_capacity(100);
            let mut writer = Writer::default(&mut w);
            let _ = r.item.rule.to_css(&mut writer);
            let _ = self.writer.write_str(&w);
            self.ctx.cache.insert(r.name.to_owned(), Some(w));
        }

        for (group, names) in groups {
            let rule = Rule::new_with_decls(
                names
                    .iter()
                    .map(|s| {
                        format!(".{}", {
                            let mut w = String::new();
                            serialize_name(&s, &mut w).unwrap();
                            w
                        })
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                group.as_decls(),
            );
            let _ = rule.to_css(&mut self.writer);
        }

        // TODO: temporarily disable this because of the wasm build
        // let w: &mut dyn Write = if let Some(output) = output {
        //     &mut BufWriter::new(
        //         OpenOptions::new()
        //             .write(true)
        //             .create(true)
        //             .append(false)
        //             .truncate(true)
        //             .open(Path::new(output))
        //             .unwrap(),
        //     )
        // } else {
        //     &mut std::io::stdout()
        // };

        // w.write_all(self.writer.dest.as_bytes()).unwrap();
        // println!(
        //     "Parsed in {:>8.2?}ms, {} rules generated",
        //     start.elapsed().as_micros() as f32 / 1000f32,
        //     res_len,
        // );
        self.writer.dest.clone()
    }
}

pub fn get_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| Some(e.ok()?.path().to_owned()).filter(|p| p.is_file()))
        .collect()
}
