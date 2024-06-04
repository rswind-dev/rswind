// TODO: re-enable this benchmark
// use std::env;

use criterion::{criterion_group, criterion_main, Criterion};
// use dotenv::dotenv;
// use rswind::create_generator;
// use rswind_cli::io::FileInput;
// use rswind_extractor::Extractable;
// use rustc_hash::FxHashSet as HashSet;

pub fn criterion_benchmark(_c: &mut Criterion) {
    // dotenv().ok();

    // let input_path = env::var("BENCH_INPUT_PATH").expect("BENCH_INPUT_PATH is not set") + "/*.html";

    // c.bench_function("Parse Multi With Read", |b| {
    //     b.iter(|| {
    //         let mut app = create_generator();
    //         let reader =
    //             GlobWalker::new_with_cwd(vec![&input_path], input_path.clone().into()).unwrap();
    //         let files = reader.walk();

    //         app.run_parallel_with_read(files);
    //     });
    // });

    // c.bench_function("Parse Multi Without Read", |b| {
    //     let reader =
    //         GlobWalker::new_with_cwd(vec![&input_path], input_path.clone().into()).unwrap();
    //     let files =
    //         reader.walk().into_par_iter().map(|f| FileInput::from_file(&f)).collect::<Vec<_>>();

    //     b.iter(|| {
    //         let mut app = create_generator();

    //         app.run_parallel_with(files.par_iter().map(|f| f.extract()).reduce(
    //             HashSet::default,
    //             |mut acc, x| {
    //                 acc.extend(x);
    //                 acc
    //             },
    //         ))
    //     });
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
