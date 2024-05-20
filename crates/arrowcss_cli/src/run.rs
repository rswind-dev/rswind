use std::path::PathBuf;

use arrowcss::app::Application;
use arrowcss_extractor::Extractable;
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

use crate::read::FileInput;

pub trait RunParallel {
    fn run_parallel(&mut self, input: impl IntoParallelIterator<Item = PathBuf>) -> String;
}

impl RunParallel for Application {
    fn run_parallel(&mut self, input: impl IntoParallelIterator<Item = PathBuf>) -> String {
        let contents = input.into_par_iter().map(|f| FileInput::from_file(&f)).collect::<Vec<_>>();

        self.run_parallel_with(contents.par_iter().map(Extractable::extract).reduce(
            HashSet::default,
            |mut acc, x| {
                acc.extend(x);
                acc
            },
        ))
    }
}
