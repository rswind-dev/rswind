use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rswind::{
    context::Context,
    preset::{dynamics::load_dynamic_utilities, statics::load_static_utilities},
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Load Static", |b| {
        b.iter(|| {
            let mut ctx = Context::new();
            load_static_utilities(&mut ctx);
            black_box(());
        });
    });

    c.bench_function("Load Dynamic & Theme", |b| {
        b.iter(|| {
            let mut ctx = Context::new();
            load_dynamic_utilities(&mut ctx);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
