use std::{sync::mpsc, time::Duration};

use arrowcss::{app::Application, common::ScopeFunctions, extract::SourceInput};
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::read::{get_files, ReadFromFile};

pub(crate) trait WatchApp {
    fn watch(&mut self, dir: &str);
}

impl WatchApp for Application<'_> {
    fn watch(&mut self, dir: &str) {
        let (tx, rx) = mpsc::channel();

        let mut debouncer = new_debouncer(Duration::from_millis(10), tx).unwrap();

        debouncer
            .watcher()
            .watch(std::path::Path::new(dir), RecursiveMode::NonRecursive)
            .unwrap();

        let strict_mode = self.strict_mode;
        let files = get_files(dir);
        let files = files
            .par_iter()
            .map(|f| SourceInput::from_file(f).run_if(strict_mode, |s| s.as_unknown()));

        self.run_parallel(files);

        for change in rx {
            self.run_parallel(
                change.unwrap().par_iter().map(|f| {
                    SourceInput::from_file(&f.path).run_if(strict_mode, |s| s.as_unknown())
                }),
            );
        }
    }
}
