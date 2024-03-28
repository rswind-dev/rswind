use std::{io::stdin, sync::Arc};

use config::{Config, File};
use cssparser::color::parse_hash_color;
use lightningcss::properties::PropertyId;

use crate::{
    add_theme_rule,
    config::ArrowConfig,
    context::Context,
    css::ToCss,
    decls,
    parser::parse,
    rule::Rule,
    rules::{dynamics::load_dynamic_rules, statics::STATIC_RULES},
    writer::{self, Writer, WriterConfig},
};

pub struct Application<'c> {
    pub ctx: Arc<Context<'c>>,
    pub writer: Writer<'c, String>,
    pub buffer: String,
}

impl<'c> Application<'c> {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("examples/arrow.config"))
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
            ctx: Arc::new(Context::new(config)),
            writer,
            buffer: String::new(),
        })
    }

    pub fn init(&mut self) -> &mut Self {
        load_dynamic_rules(self.ctx.clone());
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
        .infer_by(PropertyId::Width)
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
        .add_variant("first", "&:first-child")
        .add_variant("last", "&:last-child")
        .add_variant(
            "motion-safe",
            "@media(prefers-reduced-motion: no-preference)",
        )
        .add_variant(
            "hover",
            "@media (hover: hover) and (pointer: fine) | &:hover",
        )
        .add_variant("marker", vec!["& *::marker", "&::marker"])
        .add_variant("*", "& > *")
        .add_variant("disabled", "&:disabled");

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

    pub fn run(&mut self) {
        loop {
            stdin().read_line(&mut self.buffer).unwrap();
            let res = parse(&self.buffer, self.ctx.clone());

            res.iter().for_each(|rule| {
                let _ = rule.to_css(&mut self.writer);
            });
            println!("{}", self.writer.dest);
            self.writer.dest.clear();
        }
    }
}
