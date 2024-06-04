use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use crate::app::AppInput;
use rswind_extractor::Extractor;
use walkdir::WalkDir;

pub struct FileInput {
    pub(crate) content: String,
    pub(crate) path: PathBuf,
}

impl FileInput {
    pub fn from_file(f: PathBuf) -> Self {
        Self { content: read_to_string(&f).unwrap(), path: f }
    }
}

impl<'a> From<&'a FileInput> for AppInput<'a> {
    fn from(f: &'a FileInput) -> Self {
        AppInput::new(f.path.to_str().unwrap(), &f.content)
    }
}

impl<'a> From<&'a FileInput> for Extractor<'a> {
    fn from(f: &'a FileInput) -> Self {
        Extractor::new(&f.content, f.path.to_str().unwrap())
    }
}

pub fn walk(base: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(base)
        .into_iter()
        .filter_entry(|e| {
            // TODO: Extract this
            e.file_name() != "node_modules" && e.file_name() != ".git" && e.file_name() != "target"
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect()
}

pub fn write_file(content: &str, filename: impl AsRef<Path>) {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .append(false)
        .open(filename)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}

pub fn write_output(content: &str, output: Option<&str>) {
    if let Some(output) = output {
        write_file(content, output);
    } else {
        std::io::stdout().write_all(content.as_bytes()).unwrap();
    }
}
