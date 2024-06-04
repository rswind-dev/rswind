use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rswind_extractor::css::CssExtractor;

static INPUT: &str = r#".foo {
  color: red;
  @apply bar baz;
}"#;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = INPUT.repeat(1000);
    let mut group = c.benchmark_group("Extractors");

    for i in [1, 4, 16, 64, 256].iter() {
        let input = input.repeat(*i);

        group.bench_with_input(BenchmarkId::new("Lexer", i), i, |b, _| {
            b.iter(|| {
                let _set = CssExtractor::new(&input).collect::<Vec<_>>();
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
