use arrowcss::extract::SourceInput;
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
}

criterion_group! { benches, criterion_benchmark }

criterion_main!(benches);
