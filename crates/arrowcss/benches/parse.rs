use std::{ops::Deref, rc::Rc};

use arrowcss::source::SourceInput;
use arrowcss_extractor::Extractor;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create", |b| {
        b.iter(|| {
            let _app = arrowcss::create_app();
        });
    });

    c.bench_function("parse basic", |b| {
        b.iter(|| {
            let mut app = arrowcss::create_app();
            let input = SourceInput::Html(r#"<div class="flex">"#);
            let _a = app.run(input);
        });
    });

    c.bench_function("Large File", |b| {
        b.iter(|| {
            let mut app = arrowcss::create_app();
            let input = SourceInput::Html(include_str!("fixtures/template_html"));
            let _a = app.run(input);
        });
    });

    c.bench_function("Large File Without Extract", |b| {
        let extracted = Rc::new(
            SourceInput::Html(include_str!("fixtures/template_html"))
                .extract()
                .collect::<Vec<_>>(),
        );
        b.iter(|| {
            let mut app = arrowcss::create_app();
            let _a = app.run_with(extracted.clone().deref().iter());
        });
    });
}

criterion_group! { benches, criterion_benchmark }

criterion_main!(benches);
