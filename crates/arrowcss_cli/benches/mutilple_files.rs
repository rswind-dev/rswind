use std::{
    env::args,
    fs::read_to_string,
    ops::Deref,
    path::{Path, PathBuf},
    rc::Rc,
};

use arrowcss::{create_app, extract::SourceInput};
use criterion::{criterion_group, criterion_main, Criterion};
use rayon::prelude::*;
use walkdir::WalkDir;

fn get_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .min_depth(1)
        .into_iter()
        .map(|e| e.unwrap().into_path())
        .collect()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_path = args().nth(1).unwrap();
    let input_path = input_path.as_str();
    c.bench_function("Parse Multi", |b| {
        let files = get_files(input_path);
        let files = files
            .par_iter()
            .map(|f| SourceInput::Html(read_to_string(f).unwrap()))
            .collect::<Vec<_>>();

        let file_ref = Rc::new(
            files
                .iter()
                .map(|f| SourceInput::Html(f.as_str()))
                .collect::<Vec<_>>(),
        );

        b.iter(move || {
            let mut app = create_app();
            app.run_parallel(file_ref.clone().deref());
        });
    });

    c.bench_function("Parse Multi With Read", |b| {
        b.iter(move || {
            let mut app = create_app();
            let files = get_files(input_path);
            let files = files
                .par_iter()
                .map(|f| SourceInput::Html(read_to_string(f).unwrap()));
            app.run_parallel(files);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
