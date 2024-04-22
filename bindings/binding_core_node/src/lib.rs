use std::{collections::HashMap, fmt::Write};

use arrowcss::{
    app::Application,
    create_app as _create_app,
    css::{rule::RuleList, ToCss},
    extract::Extractor,
    ordering::{create_ordering, OrderingItem, OrderingKey, OrderingMap},
    parser::{to_css_rule, GenerateResult},
    writer::Writer,
};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct ArrowCss {
    app: Application<'static>,
}

#[napi]
impl ArrowCss {
    pub fn new() -> Self {
        ArrowCss { app: _create_app() }
    }

    #[napi]
    pub fn generate(&mut self, css: String) -> String {
        let res = Extractor::new(&css)
            .extract()
            .into_iter()
            .filter_map(|token| {
                to_css_rule(token, &self.app.ctx)
                    .map(|rule| (token.to_owned(), rule))
            })
            .collect::<HashMap<String, GenerateResult>>();
        for (_, v) in res.iter() {
            self.app.ctx.seen_variants.extend(v.variants.clone());
        }

        let get_key = |r: &GenerateResult| {
            r.variants
                .iter()
                .map(|v| {
                    self.app
                        .ctx
                        .seen_variants
                        .iter()
                        .position(|x| x == v)
                        .unwrap()
                })
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
            let _ = self.app.writer.write_str(&w);
            self.app.ctx.cache.insert(r.name.to_owned(), Some(w));
        }

        self.app.writer.dest.clone()
    }
}

#[napi]
pub fn create_app() -> ArrowCss {
    ArrowCss::new()
}
