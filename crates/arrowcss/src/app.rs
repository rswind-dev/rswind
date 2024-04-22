use std::{
    fmt::Write as _,
    fs::{read_to_string, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::mpsc,
    time::{Duration, Instant},
};

use config::{Config, File};
use fxhash::FxHashMap as HashMap;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::{
    config::ArrowConfig,
    context::Context,
    css::ToCss,
    extract::Extractor,
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
}

impl<'c> Application<'c> {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("arrow.config"))
            .build()
            .map(|c| c.try_deserialize::<ArrowConfig>())
            .unwrap_or_else(|_| Ok(ArrowConfig::default()))?;

        let w = String::new();
        let writer = Writer::default(w);

        Ok(Self {
            ctx: Context::new(config),
            writer,
            buffer: String::new(),
            cache: String::new(),
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

        let files = get_files(dir);
        let files = files
            .par_iter()
            .into_par_iter()
            .map(|f| read_to_string(f).unwrap());
        // .map(|f| generate_parallel(&self.ctx, &f))
        // .collect_vec_list();

        self.run_parallel_with(files, output);

        for change in rx {
            self.run_parallel_with(
                change
                    .unwrap()
                    .into_par_iter()
                    .map(|f| read_to_string(f.path).unwrap()),
                output,
            );
        }
    }

    pub fn run_parallel(&mut self, path: impl AsRef<Path>, output: Option<&str>) -> String {
        self.run_parallel_with(
            get_files(path.as_ref())
                .par_iter()
                .into_par_iter()
                .map(|f| read_to_string(f).unwrap()),
            output,
        )
    }

    pub fn run_parallel_with(
        &mut self,
        input: impl IntoParallelIterator<Item = String>,
        output: Option<&str>,
    ) -> String {
        let start = Instant::now();
        let res = input
            .into_par_iter()
            .map(|x| generate_parallel(&self.ctx, &x))
            .reduce(HashMap::default, |mut a, b| {
                a.extend(b);
                a
            });

        for (_, v) in res.iter() {
            self.ctx.seen_variants.extend(v.variants.clone());
        }

        let get_key = |r: &GenerateResult| {
            r.variants
                .iter()
                .map(|v| self.ctx.seen_variants.iter().position(|x| x == v).unwrap())
                .fold(0u128, |order, o| order | (1 << o))
        };

        let ordering = create_ordering();
        let res_len = res.len();

        let mut om = OrderingMap::new(&ordering);
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
        println!("Execution time: {:?}", start.elapsed());

        let w: &mut dyn Write = if let Some(output) = output {
            &mut BufWriter::new(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(false)
                    .truncate(true)
                    .open(Path::new(output))
                    .unwrap(),
            )
        } else {
            &mut std::io::stdout()
        };

        w.write_all(self.writer.dest.as_bytes()).unwrap();
        println!(
            "Parsed in {:>8.2?}, {} rules generated",
            start.elapsed(),
            res_len,
        );
        self.writer.dest.clone()
    }
}

pub fn generate_parallel<'a, 'c: 'a>(
    ctx: &'a Context<'c>,
    input: &str,
) -> HashMap<String, GenerateResult<'c>> {
    Extractor::new(&input)
        .extract()
        .into_iter()
        .filter_map(|token| to_css_rule(token, ctx).map(|rule| (token.to_owned(), rule)))
        .collect::<HashMap<String, GenerateResult>>()
}

pub fn get_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| Some(e.ok()?.path().to_owned()).filter(|p| p.is_file()))
        .collect()
}
