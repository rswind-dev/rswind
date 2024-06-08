use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rswind::{
    context::DesignSystem,
    preset::{dynamics::load_dynamic_utilities, statics::load_static_utilities},
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Load Static", |b| {
        b.iter(|| {
            let mut design = DesignSystem::new();
            load_static_utilities(&mut design);
            black_box(());
        });
    });

    c.bench_function("Load Dynamic & Theme", |b| {
        b.iter(|| {
            let mut design = DesignSystem::new();
            load_dynamic_utilities(&mut design);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
