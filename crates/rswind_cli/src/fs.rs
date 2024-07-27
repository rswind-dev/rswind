use std::{fs::OpenOptions, io::Write, path::Path};

use tracing::debug;

pub fn write_file(content: &str, filename: impl AsRef<Path>) {
    debug!("write to {:?}", filename.as_ref());
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

pub fn write_output<P: AsRef<Path>>(content: &str, output: Option<P>) {
    if let Some(output) = output {
        write_file(content, output);
    } else {
        std::io::stdout().write_all(content.as_bytes()).unwrap();
    }
}
