#![feature(trait_alias)]
#![feature(control_flow_enum)]
#![feature(auto_traits)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::fs::{self, read_to_string};
use std::sync::Arc;

use ::config::{Config, File};
use cssparser::color::parse_hash_color;
use lightningcss::properties::PropertyId;
use lightningcss::values::length::Length;

use crate::app::Application;
use crate::config::ArrowConfig;
use crate::context::{AddRule, Context, ThemeValue};
use crate::css::ToCss;
use crate::parser::parse;
use crate::rule::Rule;
use crate::rules::statics::STATIC_RULES;
use crate::writer::{Writer, WriterConfig};

mod config;
mod context;
mod css;
mod macros;
mod parser;
mod rule;
mod rules;
mod theme;
mod themes;
mod utility;
mod utils;
// mod variant;
mod variant_parse;
mod writer;
mod app;

fn main() {
    let mut app = Application::new().unwrap();
    app.init();
    app.run();

    let config = Config::builder()
        .add_source(File::with_name("examples/arrow.config"))
        .build()
        .unwrap()
        .try_deserialize::<ArrowConfig>()
        .unwrap();

    let input: &'static String =
        Box::leak(Box::new(read_to_string("examples/test.html").unwrap()));
    let ctx = Arc::new(Context::new(config));
    ctx
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

            "w" => ["width"]
            "h" => ["height"]
            "size" => ["width", "height"]
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
    parse(input, ctx.clone());

    ctx.clone().tokens.borrow().values().for_each(|it| {
        if let Some(rule) = it {
            let _ = rule.to_css(&mut writer);
        }
    });

    println!("{}", w);

    // write to test.css
    fs::write("examples/test.css", w).unwrap();
}

// unit test
#[cfg(test)]
mod tests {
    use config::{Config, File, FileFormat};

    use crate::config::ArrowConfig;

    #[test]
    fn test() {
        let config = Config::builder()
            .add_source(File::from_str(
                r##"{
        "darkMode": "class",
        "theme": {
          "spacing": {},
          "colors": {
            "black": "#000",
            "blue": {
              "50": "#eff6ff",
              "100": "#dbeafe",
              "200": "#bfdbfe",
              "300": "#93c5fd",
              "400": "#60a5fa",
              "500": "#3b82f6",
              "600": "#2563eb"
            }
          }
        }
      }"##,
                FileFormat::Json,
            ))
            .build()
            .unwrap()
            .try_deserialize::<ArrowConfig>()
            .unwrap();

        assert_eq!(config.config.dark_mode, "class");
        // assert_eq!(
        //     config.theme.get.get("blue-500"),
        //     Some(&"#3b82f6".to_string())
        // );
        // assert_eq!(config.theme.colors.get("black"), Some(&"#000".to_string()));
    }
}
