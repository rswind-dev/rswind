use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::parser::to_css_rule;
use crate::types::PropertyId;
use crate::{
    add_theme_rule,
    config::ArrowConfig,
    context::Context,
    css::ToCss,
    decls,
    rule::Rule,
    rules::{dynamics::load_dynamic_rules, statics::STATIC_RULES},
    types::CssDataType,
    writer::{self, Writer, WriterConfig},
};
use config::{Config, File};
use cssparser::color::parse_hash_color;
use hashbrown::HashSet;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;

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
        let writer = Writer::new(
            w,
            WriterConfig {
                minify: false,
                linefeed: writer::LineFeed::LF,
                indent_width: 2,
                indent_type: writer::IndentType::Space,
            },
        );

        Ok(Self {
            ctx: Context::new(config),
            writer,
            buffer: String::new(),
            cache: String::new(),
        })
    }

    pub fn init(&mut self) -> &mut Self {
        load_dynamic_rules(&mut self.ctx);
        self.ctx.clone()
    .add_rule(
        "text", 
        Rule::new(|_, value| {
            // if !ctx.config.core_plugins.text_opacity {
            //     return Some(decls! {
            //         "color" => value
            //     });
            // }
            let color = value.strip_prefix('#')?;
            let (r, g, b, a) = parse_hash_color(color.as_bytes()).ok()?;
            Some(decls! {
                "--tw-text-opacity" => a.to_string(),
                "color" => format!("rgb({} {} {} / var(--tw-text-opacity))", r, g, b)
            })
        }).infer_by(PropertyId::Color)
        .allow_values(self.ctx.get_theme("colors").unwrap())
    )
    .add_rule(
        "text",
        Rule::new(move |_, value| {
            Some(decls! {
                "font-size" => value,
                // "line-height" => ctx.get_theme_value("fontSize:lineHeight", ctx.raw)
            })
        })
        .infer_by(PropertyId::FontSize)
        .allow_values(self.ctx.get_theme("fontSize").unwrap())
    )
    .add_rule(
        "ring",
        Rule::new(|_, value| {
            Some(decls! {
                "--tw-ring-offset-shadow" => "var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)",
                "--tw-ring-shadow" => format!("var(--tw-ring-inset) 0 0 0 calc({value} + var(--tw-ring-offset-width)) var(--tw-ring-color)"),
                "box-shadow" => "var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)"
            })
        })
        .infer_by(CssDataType::LengthPercentage)
        .allow_values(self.ctx.get_theme("ringWidth").unwrap())
    )
    .add_rule("ring", Rule::new(|_, value| {
            Some(decls! {
                "--tw-ring-color" => value
            })
        })
        .allow_values(self.ctx.get_theme("colors").unwrap())
        .infer_by(PropertyId::Color)
    )
    .add_rule("ring-offset", Rule::new(|_, value| {
            Some(decls! {
                "--tw-ring-offset-color" => value
            })
        })
        .allow_values(self.ctx.get_theme("colors").unwrap())
        .infer_by(PropertyId::Color)
    )
    .add_rule("ring-offset", Rule::new(|_, value| {
            Some(decls! {
                "--tw-ring-offset-width" => value
            })
        })
        .infer_by(PropertyId::Width)
        .allow_values(self.ctx.get_theme("ringOffsetWidth").unwrap())
    )
    .add_rule("bg", Rule::new(|_, value| {
            Some(decls! {
                "background-color" => value
            })
        })
        .infer_by(PropertyId::Color)
        .allow_values(self.ctx.get_theme("colors").unwrap())
    )
    .add_rule("bg", Rule::new(|_, value| {
            Some(decls! {
                "background-position" => value
            })
        })
        .infer_by(PropertyId::BackgroundPosition)
        .allow_values(self.ctx.get_theme("backgroundPosition").unwrap())
    )
    .add_rule("bg", Rule::new(|_, value| {
            Some(decls! {
                "background-size" => value
            })
        })
        .infer_by(PropertyId::BackgroundSize)
        .allow_values(self.ctx.get_theme("backgroundSize").unwrap())
    )
    .add_rule("bg", Rule::new(|_, value| {
            Some(decls! {
                "background-image" => value
            })
        })
        .infer_by(PropertyId::BackgroundImage)
        .allow_values(self.ctx.get_theme("backgroundImage").unwrap())
    )
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
        .add_variant(
            "motion-safe",
            ["@media(prefers-reduced-motion: no-preference)"],
        )
        .add_variant(
            "hover",
            ["@media (hover: hover) and (pointer: fine) | &:hover"],
        )
        .add_variant("disabled", ["&:disabled"]);

        STATIC_RULES.iter().for_each(|(key, value)| {
            self.ctx.add_static((*key, value.clone()));
        });

        add_theme_rule!(self.ctx.clone(), {
            "spacing" => {
                "m" => ["margin"]
                "mx" => ["margin-left", "margin-right"]
                "my" => ["margin-top", "margin-bottom"]
                "mt" => ["margin-top"]
                "mr" => ["margin-right"]
                "mb" => ["margin-bottom"]
                "ml" => ["margin-left"]
                "ms" => ["margin-inline-start"]
                "me" => ["margin-inline-end"]

                "p" => ["padding"]
                "px" => ["padding-left", "padding-right"]
                "py" => ["padding-top", "padding-bottom"]
                "pt" => ["padding-top"]
                "pr" => ["padding-right"]
                "pb" => ["padding-bottom"]
                "pl" => ["padding-left"]
                "ps" => ["padding-inline-start"]
                "pe" => ["padding-inline-end"]

                "inset" => ["top", "right", "bottom", "left"]
                "inset-x" => ["left", "right"]
                "inset-y" => ["top", "bottom"]

                "top" => ["top"]
                "right" => ["right"]
                "bottom" => ["bottom"]
                "left" => ["left"]

                "gap" => ["gap"]

                "w" => ["width"]
                "h" => ["height"]
                "size" => ["width", "height"]
            }
        });
        self
    }

    pub fn generate(&mut self, _: Vec<PathBuf>) {
        let mut buffer = String::new();
        let start = Instant::now();
        let file =
            std::fs::File::open(Path::new("examples/test.html")).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let _ = reader.read_to_string(&mut buffer);

        let parts = buffer
            .split(['\n', '\r', '\t', ' ', '"', '\'', ';', '{', '}', '`'])
            .filter(|s| {
                s.starts_with(char::is_lowercase)
                    || self.ctx.cache.contains_key(*s)
            })
            .collect::<HashSet<_>>();
        println!("split: {} us", start.elapsed().as_micros());
        let _ = self.ctx.cache.iter().map(|rule| {
            let _ = self.writer.write_str(&rule.1);
        });
        for token in parts {
            if let Some(rule) = to_css_rule(token, &mut self.ctx) {
                let mut w = String::with_capacity(100);
                let mut writer = Writer::default(&mut w);
                let _ = rule.to_css(&mut writer);
                let _ = self.writer.write_str(&w);
                self.ctx.cache.insert(String::from(token), w);
            }
        }

        self.cache = buffer.clone();
        let mut w = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .open(Path::new("examples/test.css"))
                .unwrap(),
        );

        println!("Execution time: {} us", start.elapsed().as_micros());
        w.write(self.writer.dest.as_bytes()).unwrap();
        self.writer.dest.clear();
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
}
