use std::{sync::mpsc, time::Duration};

use notify::{EventKind, RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;
use rayon::prelude::*;
use rswind::{
    app::App,
    generator::ParGenerateWith,
    glob::ParallelGlobFilter,
    io::{write_output, FileInput},
};
use rswind_extractor::ParCollectExtracted;
use rustc_hash::FxHashSet;
use tracing::debug;

pub trait WatchApp {
    fn watch(&mut self, output: Option<&str>);
}

impl WatchApp for App {
    fn watch(&mut self, output: Option<&str>) {
        let (tx, rx) = mpsc::channel();

        let mut debouncer = new_debouncer(Duration::from_millis(10), None, tx).unwrap();

        debouncer.watcher().watch(self.glob.base(), RecursiveMode::Recursive).unwrap();

        let res = self.generate_contents();
        write_output(&res, output);

        for change in rx {
            let Ok(changes) = change else {
                continue;
            };

            let changes = changes
                .into_iter()
                .filter_map(|e| match e.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => Some(e.event.paths),
                    _ => None,
                })
                .flatten()
                .collect::<FxHashSet<_>>();

            debug!("Changes: {:?}", changes);

            if changes.is_empty() {
                continue;
            }

            let res = changes
                .into_par_iter()
                .glob_filter(&self.glob)
                .map(FileInput::from_file)
                .collect::<Vec<_>>()
                .par_iter()
                .collect_extracted()
                .par_generate_with(&mut self.generator);

            write_output(&res, output);
        }
    }
}
