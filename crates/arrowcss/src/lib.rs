#![feature(trait_alias)]
#![feature(control_flow_enum)]
#![feature(auto_traits)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(tuple_trait)]

use std::sync::Arc;

use crate::{
    config::ArrowConfig,
    context::Context,
    css::ToCss,
    // parser::parse,
    rules::statics::STATIC_RULES,
    writer::{Writer, WriterConfig},
};
use ::config::{Config, File};

pub mod config;
pub mod context;
pub mod css;
pub mod macros;
pub mod parser;
pub mod rule;
pub mod rules;
pub mod theme;
pub mod themes;
pub mod utility;
pub mod utils;
// pub mod variant;
pub mod variant_parse;
pub mod writer;

pub fn generate(_input: String) -> String {
    let config = Config::builder()
        .add_source(File::with_name("examples/arrow.config"))
        .build()
        .unwrap()
        .try_deserialize::<ArrowConfig>()
        .unwrap();

    // let theme = Rc::new(config.theme);

    let ctx = Arc::new(Context::new(config));

    STATIC_RULES.iter().for_each(|(key, value)| {
        ctx.add_static((*key, value.clone()));
    });

    add_theme_rule!(ctx.clone(), {
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
    // parse("", ctx.clone());

    ctx.tokens.borrow().values().for_each(|it| {
        if let Some(rule) = it {
            let _ = rule.to_css(&mut writer);
        }
    });

    w
}
