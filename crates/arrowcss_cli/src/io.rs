use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use arrowcss_extractor::{
    ecma::EcmaExtractor, html::HtmlExtractor, BasicExtractor, Extractable, InputKind,
    UniqueCandidate,
};
use rustc_hash::FxHashSet as HashSet;
use walkdir::WalkDir;

static ALLOWED_EXTENSIONS: [&str; 7] = ["html", "vue", "js", "jsx", "ts", "tsx", "svelte"];

pub struct FileInput {
    content: String,
    kind: InputKind,
}

impl FileInput {
    pub fn from_file(f: &Path) -> Self {
        Self {
            content: read_to_string(f).unwrap(),
            kind: InputKind::from(f.extension().unwrap().to_str().unwrap_or_default()),
        }
    }

    #[allow(dead_code)]
    pub fn into_unknown(self) -> Self {
        Self { content: self.content, kind: InputKind::Unknown }
    }
}

impl<'a> Extractable<'a> for &'a FileInput {
    fn extract(self) -> HashSet<&'a str> {
        match self.kind {
            InputKind::Html => HtmlExtractor::new(&self.content)
                .apply_options(|o| o.class_only = true)
                .filter_invalid(),
            InputKind::Ecma => EcmaExtractor::new(&self.content).filter_invalid(),
            InputKind::Unknown => BasicExtractor::new(&self.content).extract_inner(),
        }
    }
}

pub fn allowed_files(e: impl AsRef<Path>) -> bool {
    e.as_ref()
        .extension()
        .map(|e| ALLOWED_EXTENSIONS.contains(&e.to_str().unwrap_or("")))
        .unwrap_or(false)
}

pub fn get_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| {
            !e.file_name().to_str().unwrap_or_default().starts_with('.')
                && e.file_name() != "node_modules"
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && ALLOWED_EXTENSIONS
                    .contains(&e.path().extension().unwrap().to_str().unwrap_or(""))
        })
        .map(|e| e.into_path())
        .filter(|e| allowed_files(e))
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
