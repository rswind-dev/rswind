use std::{path::Path, sync::mpsc, time::Duration};

use rswind::app::Application;
use notify::{EventKind, RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;
use rustc_hash::FxHashSet;
use tracing::info;

use crate::{
    io::{allowed_files, get_files, write_output},
    run::RunParallel,
};

pub trait WatchApp {
    fn watch(&mut self, dir: &str, output: Option<&str>);
}

impl WatchApp for Application {
    fn watch(&mut self, dir: &str, output: Option<&str>) {
        let (tx, rx) = mpsc::channel();

        let mut debouncer = new_debouncer(Duration::from_millis(10), None, tx).unwrap();

        debouncer.watcher().watch(Path::new(dir), RecursiveMode::Recursive).unwrap();

        let files = get_files(dir);

        info!("Found {} files in {}", files.len(), dir);

        let res = self.run_parallel(files);

        write_output(&res, output);

        for change in rx {
            let Ok(changes) = change else {
                continue;
            };

            let changes = changes
                .iter()
                .filter_map(|e| match e.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => Some(e.paths.iter()),
                    _ => None,
                })
                .flatten()
                .filter(|e| allowed_files(e))
                .collect::<FxHashSet<_>>();

            if changes.is_empty() {
                continue;
            }

            let res = self.run_parallel(changes);
            write_output(&res, output);
        }
    }
}
