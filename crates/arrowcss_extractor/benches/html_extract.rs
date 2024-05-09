use arrowcss_extractor::html::HtmlExtractor;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

static HTML: &str = include_str!("fixtures/html_input");

fn extract_split(input: &str) -> impl Iterator<Item = &str> {
    input
        .split(['\n', ' ', '"', '\'', ';', '{', '}', '`', '\r', '\t'])
        .filter(|s| s.starts_with(char::is_lowercase) || s.starts_with('-'))
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Extractors");

    for i in [1, 64, 1024].iter() {
        let input = HTML.repeat(*i);

        group.bench_with_input(BenchmarkId::new("Split", i), i, |b, _| {
            b.iter(|| {
                let _set = extract_split(&input).collect::<Vec<_>>();
            })
        });

        group.bench_with_input(BenchmarkId::new("Lexer", i), i, |b, _| {
            b.iter(|| {
                let _set = HtmlExtractor::new(&input).collect::<Vec<_>>();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
