use std::{rc::Rc, collections::HashMap};

use ::config::{Config, File};

use crate::context::ThemeValue;
use crate::{config::ArrowConfig, context::Context, css::{CSSDecls, ToCss}, rules::statics::STATIC_RULES, writer::{WriterConfig, Writer}, parser::parse};

pub mod config;
pub mod context;
pub mod css;
pub mod parser;
pub mod rule;
pub mod rules;
pub mod theme;
pub mod writer;

pub fn generate(input: String) -> String {
  let config = Config::builder()
        .add_source(File::with_name("examples/arrow.config"))
        .build()
        .unwrap()
        .try_deserialize::<ArrowConfig>()
        .unwrap();

    let theme = Rc::new(config.theme);

    let mut ctx = Context {
        tokens: HashMap::new(),
        static_rules: HashMap::new(),
        arbitrary_rules: HashMap::new(),
        rules: HashMap::new(),
        theme: Rc::clone(&theme),
        config: "config".into(),
    };

    ctx.add_rule("text", |value, theme| {
        theme
            .colors
            .get(value)
            .map(|color| CSSDecls::one("color", color))
    });

    STATIC_RULES.iter().for_each(|(key, value)| {
        ctx.add_static((*key, value.clone()));
    });

    add_theme_rule!(ctx, {
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
        }
    });

    let mut w = String::new();
    let mut writer = Writer::new(
        &mut w,
        WriterConfig {
            minify: false,
            linefeed: writer::LineFeed::LF,
            indent_width: 2,
            indent_type: writer::IndentType::Space,
        },
    );

    // open test.html
    parse(&input, &mut ctx);

    ctx.tokens.values().into_iter().for_each(|it| {
        if let Some(rule) = it {
            let _ = rule.to_css(&mut writer);
        }
    });

    w
}