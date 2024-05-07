use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use arrowcss::source::SourceInput;
use walkdir::WalkDir;

static ALLOWED_EXTENSIONS: [&str; 7] = ["html", "vue", "js", "jsx", "ts", "tsx", "svelte"];

pub(crate) trait ReadFromFile {
    fn from_file(f: &PathBuf) -> Self;
}

impl ReadFromFile for SourceInput<String> {
    fn from_file(f: &PathBuf) -> Self {
        Self::new(
            read_to_string(f).unwrap(),
            f.extension().unwrap().to_str().unwrap_or_default(),
        )
    }
}

pub(crate) fn get_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| {
            e.file_type().is_file()
                && !e.file_name().to_str().unwrap_or_default().starts_with('.')
                && e.file_name() != "node_modules"
        })
        .map(|e| e.unwrap().into_path())
        .filter(|e| {
            e.extension()
                .map(|e| ALLOWED_EXTENSIONS.contains(&e.to_str().unwrap_or("")))
                .is_some_and(|e| e)
        })
        .collect()
}
