use std::{iter, rc::Rc};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use either::Either::{Left, Right};
use rswind::{create_processor, process::ValuePreprocessor};
use rswind_extractor::{Extractable, Extractor, InputKind};
use smol_str::format_smolstr;

fn gen_fixtures() -> String {
    let app = create_processor();
    let (utilities, variants) = (&app.design.utilities, &app.design.variants);

    let mut combinations = utilities
        .iter()
        .flat_map(|(key, values)| {
            values.iter().flat_map(move |value| match value {
                Left(_) => Some(Left(iter::once(key.to_owned()))),
                Right(utility) => Some(Right(
                    utility
                        .allowed_values()?
                        .iter()
                        .map(move |(k, _)| format_smolstr!("{key}-{k}")),
                )),
            })
        })
        .flat_map(|v| v.into_iter())
        .collect::<Vec<_>>();

    combinations.sort_unstable();

    combinations
        .iter()
        .flat_map(|f| variants.iter().map(|(k, _)| k).map(move |v| format_smolstr!("{v}:{f}")))
        .map(|v| format!("<div class=\"{v}\"></div>"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn bench_all(c: &mut Criterion) {
    let fixture = gen_fixtures();
    c.bench_function("Generate all rules", |b| {
        b.iter(|| {
            let mut app = create_processor();
            let input = Extractor::new(&fixture, InputKind::Html).extract();
            let _a = app.run_with(input);
        });
    });

    c.bench_function("Generate all rules parallel", |b| {
        b.iter(|| {
            let mut app = create_processor();
            let input = Extractor::new(&fixture, InputKind::Html).extract();
            let _a = app.run_parallel_with(input);
        });
    });
}

pub fn bench_static(c: &mut Criterion) {
    c.bench_function("create", |b| {
        b.iter(|| {
            let _app = rswind::create_processor();
        });
    });

    c.bench_function("parse basic", |b| {
        b.iter(|| {
            let mut app = rswind::create_processor();
            let input = Extractor::new(r#"<div class="flex">"#, InputKind::Html);
            let _a = app.run_with(input.extract());
        });
    });

    let mut group = c.benchmark_group("LargeFile");

    for i in [1, 10, 1000].iter() {
        let input = include_str!("fixtures/template_html").repeat(*i);
        group.bench_with_input(BenchmarkId::new("Normal", i), i, |b, _| {
            b.iter(|| {
                let mut app = rswind::create_processor();
                let input = Extractor::new(&input, InputKind::Html);
                let _a = app.run_with(input.extract());
            });
        });

        group.bench_with_input(BenchmarkId::new("Parallel", i), i, |b, _| {
            b.iter(|| {
                let mut app = rswind::create_processor();
                let input = Extractor::new(&input, InputKind::Html);
                let _a = app.run_parallel_with(input.extract());
            });
        });

        group.bench_with_input(BenchmarkId::new("Without Extract", i), i, |b, _| {
            let extracted = Extractor::new(&input, InputKind::Html);
            let extracted = Rc::new(extracted.extract());

            b.iter(|| {
                let mut app = rswind::create_processor();
                let _a = app.run_with(Rc::clone(&extracted).iter().copied());
            });
        });
    }
}

criterion_group! { benches, bench_all }

criterion_main!(benches);
