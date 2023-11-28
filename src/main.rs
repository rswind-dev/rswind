use std::collections::HashMap;
use std::rc::Rc;

use ::config::{Config, File};

use crate::config::ArrowConfig;
use crate::context::Context;
use crate::css::{Rule, ToCss, ToCssRule, CSSDecls};
use crate::writer::{Writer, WriterConfig};

mod config;
mod context;
mod css;
mod theme;
mod writer;
mod rules;
mod rule;
mod parser;

fn main() {
    let config = Config::builder()
        .add_source(File::with_name("examples/arrow.config"))
        .build()
        .unwrap()
        .try_deserialize::<ArrowConfig>()
        .unwrap();

    let rules = vec![
        Rule {
            raw: "dark:text-red-500",
            rule: "text-red-500",
            modifier: vec!["dark"],
        },
        Rule {
            raw: "text-blue-500",
            rule: "text-blue-500",
            modifier: vec![],
        },
        Rule {
            raw: "inset-1",
            rule: "inset-1",
            modifier: vec![],
        },
        Rule {
            raw: "flex",
            rule: "flex",
            modifier: vec![],
        },
        Rule {
            raw: "block",
            rule: "block",
            modifier: vec![],
        },
    ];

    let theme = Rc::new(config.theme);

    let mut ctx = Context {
        static_rules: HashMap::new(),
        arbitrary_rules: HashMap::new(),
        rules: HashMap::new(),
        theme: Rc::clone(&theme),
        config: "config".into(),
    };

    ctx.add_rule("text", |value, theme| {
        theme.colors.get(value).map(|color|
            CSSDecls::one("color", color)
        )
    }).add_rule("inset", |value, theme| {
        theme.spacing.get(value).map(|space|
            CSSDecls::multi([
                ("top", space),
                ("right", space),
                ("bottom", space),
                ("left", space),
            ])
        )
    });

    add_static!(ctx, {
        "block" => { "display": "block"; }
        "inline-block" => { "display": "inline-block"; }
        "flex" => { "display": "flex"; }
        "inline-flex" => { "display": "inline-flex"; }
        "table" => { "display": "table"; }
        "inline-table" => { "display": "inline-table"; }
        "table-caption" => { "display": "table-caption"; }
        "table-cell" => { "display": "table-cell"; }
        "table-column" => { "display": "table-column"; }
        "table-column-group" => { "display": "table-column-group"; }
        "table-footer-group" => { "display": "table-footer-group"; }
        "table-header-group" => { "display": "table-header-group"; }
        "table-row-group" => { "display": "table-row-group"; }
        "table-row" => { "display": "table-row"; }
        "flow-root" => { "display": "flow-root"; }
        "grid" => { "display": "grid"; }
        "inline-grid" => { "display": "inline-grid"; }
        "contents" => { "display": "contents"; }
        "list-item" => { "display": "list-item"; }
        "hidden" => { "display": "none"; }
    });

    let mut w = String::new();
    let mut writer = Writer::new(&mut w, WriterConfig {
        minify: false,
        linefeed: writer::LineFeed::LF,
        indent_width: 2,
        indent_type: writer::IndentType::Space,
    });

    rules
    .iter()
    .map(|it| it.to_css_rule(&ctx))
    .filter_map(|it| it.is_some().then(|| it.unwrap()))
    .for_each(|it| {
        let _ = it.to_css(&mut writer);
    });

    println!("{}", w);
}
