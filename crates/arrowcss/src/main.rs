#![feature(trait_alias)]
#![feature(fn_traits)]

use std::fs::{self, read_to_string};
use std::rc::Rc;

use ::config::{Config, File};

use crate::config::ArrowConfig;
use crate::context::{Context, ThemeValue};
use crate::css::{CSSAtRule, CSSDecls, CSSRule, ToCss};
use crate::parser::parse;
use crate::rules::statics::STATIC_RULES;
use crate::writer::{Writer, WriterConfig};

mod config;
mod context;
mod css;
mod parser;
mod rule;
mod rules;
mod theme;
mod writer;

fn main() {
    let config = Config::builder()
        .add_source(File::with_name("examples/arrow.config"))
        .build()
        .unwrap()
        .try_deserialize::<ArrowConfig>()
        .unwrap();

    let theme = Rc::new(config.theme);

    let input: &'static String = Box::leak(Box::new(read_to_string("examples/test.html").unwrap()));
    let mut ctx = Context::new(theme.clone());

    ctx.add_rule("text", |value, theme| {
        theme
            .colors
            .get(&value)
            .map(|color| CSSDecls::one("color", color))
    })
    .add_variant("disabled", |a| match a {
        CSSRule::Style(mut it) => {
            it.selector += ":disabled";
            Some(CSSRule::Style(it))
        }
        _ => None,
    })
    .add_at_rule_variant("motion-safe", |a| match a {
        CSSRule::Style(it) => {
            let rule = CSSAtRule {
                name: "media".into(),
                params: "(prefers-reduced-motion: no-preference)".into(),
                nodes: vec![CSSRule::Style(it)],
            };
            Some(CSSRule::AtRule(rule))
        }
        _ => None,
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
    parse(input, &mut ctx);

    ctx.tokens.values().for_each(|it| {
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

        assert_eq!(config.dark_mode, "class");
        assert_eq!(
            config.theme.colors.get("blue-500"),
            Some(&"#3b82f6".to_string())
        );
        assert_eq!(config.theme.colors.get("black"), Some(&"#000".to_string()));
    }
}
