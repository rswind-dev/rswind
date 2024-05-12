use std::{ops::Deref, rc::Rc};

use arrowcss_extractor::{Extractable, Extractor, InputKind};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rayon::iter::ParallelBridge;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create", |b| {
        b.iter(|| {
            let _app = arrowcss::create_app();
        });
    });

    c.bench_function("parse basic", |b| {
        b.iter(|| {
            let mut app = arrowcss::create_app();
            let input = Extractor::new(r#"<div class="flex">"#, InputKind::Html);
            let _a = app.run_with(input.extract());
        });
    });

    let mut group = c.benchmark_group("LargeFile");

    for i in [1, 10, 1000].iter() {
        let input = include_str!("fixtures/template_html").repeat(*i);
        group.bench_with_input(BenchmarkId::new("Normal", i), i, |b, _| {
            b.iter(|| {
                let mut app = arrowcss::create_app();
                let input = Extractor::new(&input, InputKind::Html);
                let _a = app.run_with(input.extract());
            });
        });

        group.bench_with_input(BenchmarkId::new("Parallel", i), i, |b, _| {
            b.iter(|| {
                let mut app = arrowcss::create_app();
                let input = Extractor::new(&input, InputKind::Html);
                let _a = app.run_parallel_with(input.extract().par_bridge());
            });
        });

        group.bench_with_input(BenchmarkId::new("Without Extract", i), i, |b, _| {
            let extracted = Extractor::new(&input, InputKind::Html);
            let extracted = Rc::new(extracted.extract().collect::<Vec<_>>());

            b.iter(move || {
                let mut app = arrowcss::create_app();
                let _a = app.run_with(extracted.clone().deref().into_iter());
            });
        });
    }
}

criterion_group! { benches, criterion_benchmark }

criterion_main!(benches);
