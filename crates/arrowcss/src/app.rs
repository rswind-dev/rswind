use std::fmt::Write as _;
use std::fs::{read_to_string, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use config::{Config, File};
use fxhash::FxHashMap as HashMap;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::css::rule::RuleList;
use crate::extract::Extractor;
use crate::parser::to_css_rule;
use crate::theme::ThemeValue;
use crate::variant::create_variants;
use crate::{
    config::ArrowConfig,
    context::Context,
    css::ToCss,
    rules::{dynamics::load_dynamic_utilities, statics::STATIC_RULES},
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
        load_dynamic_utilities(&mut self.ctx);
        create_variants(&mut self.ctx);

        STATIC_RULES.iter().for_each(|(key, value)| {
            self.ctx.add_static((*key, value.clone()));
        });
        self
    }

    pub fn generate(&mut self, _: Vec<PathBuf>) {
        let start = Instant::now();
        let buffer = std::fs::read_to_string("examples/test.html").unwrap();
        println!("read: {} us", start.elapsed().as_micros());

        let parts = Extractor::new(&buffer).extract();

        println!("split: {} us", start.elapsed().as_micros());

        for token in parts {
            if let Some(rule) = to_css_rule(token, &self.ctx) {
                let mut w = String::with_capacity(100);
                let mut writer = Writer::default(&mut w);
                let _ = rule.to_css(&mut writer);
                let _ = self.writer.write_str(&w);
                self.ctx.cache.insert(String::from(token), Some(w));
            } else {
                self.ctx.cache.insert(String::from(token), None);
            }
        }

        let mut w = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(Path::new("examples/test.css"))
                .unwrap(),
        );

        println!("Execution time: {} us", start.elapsed().as_micros());
        w.write_all(self.writer.dest.as_bytes()).unwrap();
    }

    pub fn watch(&mut self, dir: &str, output: Option<&str>) {
        let (tx, rx) = mpsc::channel();

        let mut debouncer =
            new_debouncer(Duration::from_millis(0), tx).unwrap();

        debouncer
            .watcher()
            .watch(Path::new(dir), RecursiveMode::NonRecursive)
            .unwrap();

        let files = get_files(dir);
        self.run_parallel(files.as_slice(), output);

        for change in rx {
            let files = change
                .unwrap()
                .into_iter()
                .map(|e| e.path)
                .collect::<Vec<_>>();
            self.run_parallel(files.as_slice(), output);
        }
    }

    pub fn run_parallel(&mut self, paths: &[PathBuf], output: Option<&str>) {
        let start = Instant::now();
        let res = paths
            .par_iter()
            .map(|x| generate_parallel(&self.ctx, x))
            // .collect::<HashMap<_, _>>();
            .reduce(HashMap::default, |mut a, b| {
                a.extend(b);
                a
            });
        let res_len = res.len();
        for (token, rule) in res {
            let mut w = String::with_capacity(100);
            let mut writer = Writer::default(&mut w);
            let _ = rule.to_css(&mut writer);
            let _ = self.writer.write_str(&w);
            self.ctx.cache.insert(token, Some(w));
        }
        println!("Execution time: {:?}", start.elapsed());

        let mut w: Box<dyn Write> = if let Some(output) = output {
            Box::new(BufWriter::new(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(false)
                    .truncate(true)
                    .open(Path::new(output))
                    .unwrap(),
            ))
        } else {
            Box::new(std::io::stdout())
        };

        w.write_all(self.writer.dest.as_bytes()).unwrap();
        println!(
            "Parsed {:3} file{:1} in {:>8.2?}, {} rules generated",
            paths.len(),
            if paths.len() > 1 { "s" } else { "" },
            start.elapsed(),
            res_len,
        );
    }
}

pub fn generate_parallel<'a, 'c: 'a, P: AsRef<Path>>(
    ctx: &'a Context<'c>,
    path: P,
) -> HashMap<String, RuleList<'c>> {
    Extractor::new(&read_to_string(path.as_ref()).unwrap())
        .extract()
        .into_iter()
        .filter_map(|token| {
            to_css_rule(token, ctx).map(|rule| (token.to_owned(), rule))
        })
        .collect::<HashMap<String, RuleList>>()
}

pub fn get_files(dir: &str) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| Some(e.ok()?.path().to_owned()).filter(|p| p.is_file()))
        .collect()
}
