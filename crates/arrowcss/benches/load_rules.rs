use arrowcss::{
    context::Context,
    rules::{dynamics::load_dynamic_utilities, statics::load_static_utilities},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Load Static", |b| {
        b.iter(|| {
            let mut ctx = Context::default();
            black_box(load_static_utilities(&mut ctx));
        });
    });

    c.bench_function("Load Dynamic & Theme", |b| {
        b.iter(|| {
            let mut ctx = Context::new(Default::default());
            load_dynamic_utilities(&mut ctx);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
