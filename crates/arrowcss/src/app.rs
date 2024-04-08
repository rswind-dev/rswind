use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs::{read_to_string, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use config::{Config, File};
use fxhash::FxHashSet as HashSet;
use lightningcss::traits::IntoOwned;
use lightningcss::values::string::CowArcStr;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::css::{AstNode, Rule};
use crate::parser::to_css_rule;
use crate::{
    config::ArrowConfig,
    context::Context,
    css::ToCss,
    rules::{dynamics::load_dynamic_rules, statics::STATIC_RULES},
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
            .build()?
            .try_deserialize::<ArrowConfig>()?;

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
        load_dynamic_rules(&mut self.ctx);
        self.ctx
            .add_variant("first", ["&:first-child"])
            .add_variant("last", ["&:last-child"])
            .add_variant(
                "motion-safe",
                ["@media(prefers-reduced-motion: no-preference)"],
            )
            .add_variant(
                "hover",
                ["@media (hover: hover) and (pointer: fine) | &:hover"],
            )
            .add_variant("focus", ["&:focus"])
            .add_variant("marker", ["& *::marker", "&::marker"])
            .add_variant("*", ["& > *"])
            .add_variant("first", ["&:first-child"])
            .add_variant("last", ["&:last-child"])
            .add_variant("disabled", ["&:disabled"]);

        for (key, value) in self.ctx.get_theme("breakpoints").unwrap().iter() {
            let value: CowArcStr<'static> = value.clone().into_owned();
            self.ctx.add_variant_fn(&key.clone(), move |rule| {
                Some(
                    AstNode::Rule(Rule {
                        selector: format!("@media (width >= {value})"),
                        nodes: rule,
                    })
                    .into(),
                )
            });
        }

        STATIC_RULES.iter().for_each(|(key, value)| {
            self.ctx.add_static((*key, value.clone()));
        });
        self
    }

    pub fn generate(&mut self, _: Vec<PathBuf>) {
        let start = Instant::now();
        let buffer = std::fs::read_to_string("examples/test.html").unwrap();
        println!("read: {} us", start.elapsed().as_micros());

        let parts = buffer
            .split(['\n', '\r', '\t', ' ', '"', '\'', ';', '{', '}', '`'])
            .filter(|s| {
                s.starts_with(char::is_lowercase)
                    && !self.ctx.cache.contains_key(*s)
            })
            .collect::<HashSet<_>>();
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

    pub fn run(&mut self) {
        let (tx, rx) = mpsc::channel();

        let mut debouncer =
            new_debouncer(Duration::from_millis(0), tx).unwrap();

        debouncer
            .watcher()
            .watch(Path::new("examples/test.html"), RecursiveMode::NonRecursive)
            .unwrap();

        self.generate(vec![]);

        for _ in rx {
            self.generate(vec![]);
        }
    }

    pub fn run_parallel(&mut self, dir: &str) {
        let start = Instant::now();
        let res = WalkDir::new(dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| {
                Some(e.ok()?.path().to_owned()).filter(|p| p.is_file())
            })
            .par_bridge()
            .map(|x| generate_parallel(&self.ctx, x))
            .reduce(HashMap::new, |mut a, b| {
                a.extend(b);
                a
            });

        for (token, rule) in res {
            // if self.ctx.cache.contains_key(&token) {
            //     continue;
            // }
            let mut w = String::with_capacity(100);
            let mut writer = Writer::default(&mut w);
            let _ = rule.to_css(&mut writer);
            let _ = self.writer.write_str(&w);
            self.ctx.cache.insert(token, Some(w));
        }

        let mut w = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .truncate(true)
                .open(Path::new("examples/test.css"))
                .unwrap(),
        );

        w.write_all(self.writer.dest.as_bytes()).unwrap();
        println!("Execution time: {:?}", start.elapsed());
    }
}

pub fn generate_parallel<'a, 'c: 'a, P: AsRef<Path>>(
    ctx: &'a Context<'c>,
    path: P,
) -> HashMap<String, Vec<AstNode<'c>>> {
    read_to_string(path.as_ref())
        .unwrap()
        .split(['\n', '\r', '\t', ' ', '"', '\'', ';', '{', '}', '`'])
        .filter(|s| s.starts_with(char::is_lowercase))
        .collect::<HashSet<_>>()
        .into_iter()
        .filter_map(|token| {
            to_css_rule(token, ctx).map(|rule| (token.to_owned(), rule))
        })
        .collect::<HashMap<String, Vec<AstNode>>>()
}
