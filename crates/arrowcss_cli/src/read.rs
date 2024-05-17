use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use arrowcss_extractor::{
    ecma::EcmaExtractor, html::HtmlExtractor, BasicExtractor, Extractable, InputKind,
    UniqueCandidate,
};
use walkdir::WalkDir;

static ALLOWED_EXTENSIONS: [&str; 7] = ["html", "vue", "js", "jsx", "ts", "tsx", "svelte"];

pub struct FileInput {
    content: String,
    kind: InputKind,
}

impl FileInput {
    pub fn from_file(f: &PathBuf) -> Self {
        Self {
            content: read_to_string(f).unwrap(),
            kind: InputKind::from(f.extension().unwrap().to_str().unwrap_or_default()),
        }
    }

    #[allow(dead_code)]
    pub fn into_unknown(self) -> Self {
        Self {
            content: self.content,
            kind: InputKind::Unknown,
        }
    }
}

impl<'a> Extractable<'a> for &'a FileInput {
    fn extract(self) -> impl Iterator<Item = &'a str> {
        match self.kind {
            InputKind::Html => HtmlExtractor::new(&self.content)
                .apply_options(|o| o.class_only = true)
                .filter_invalid(),
            InputKind::Ecma => EcmaExtractor::new(&self.content).filter_invalid(),
            InputKind::Unknown => BasicExtractor::new(&self.content).extract_inner(),
        }
    }
}

pub fn get_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
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
