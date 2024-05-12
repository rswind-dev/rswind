use std::{sync::mpsc, time::Duration};

use arrowcss::app::Application;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use rayon::iter::{
    ParallelBridge, ParallelIterator,
};

use crate::{read::{get_files}, run::RunParallel};

pub trait WatchApp {
    fn watch(&mut self, dir: &str);
}

impl WatchApp for Application {
    fn watch(&mut self, dir: &str) {
        let (tx, rx) = mpsc::channel();

        let mut debouncer = new_debouncer(Duration::from_millis(10), tx).unwrap();

        debouncer
            .watcher()
            .watch(std::path::Path::new(dir), RecursiveMode::NonRecursive)
            .unwrap();

        self.run_parallel(get_files(dir));

        for change in rx {
            self.run_parallel(change.unwrap().into_iter().map(|e| e.path).par_bridge());
        }
    }
}
