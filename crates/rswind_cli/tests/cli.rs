use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    process::{self, Stdio},
    thread::{self},
    time::{Duration, Instant},
};

use assert_cmd::{cargo::CommandCargoExt, Command};
use assert_fs::{
    assert::PathAssert,
    fixture::{FileWriteStr, PathChild, PathCopy},
    TempDir,
};

fn cli() -> Command {
    Command::cargo_bin("rswind_cli").expect("Failed to build rswind_cli")
}

macro_rules! until_updated {
    ($left:expr, $right:expr) => {
        let start = Instant::now();
        let predicate_fn = || $left == $right;
        while predicate_fn() == false {
            thread::sleep(Duration::from_millis(100));
            if start.elapsed() > Duration::from_secs(3) {
                assert_eq!($left, $right)
            }
        }
    };
}

#[test]
fn test_cli() {
    let temp = assert_fs::NamedTempFile::new("index.css").expect("Failed to create tempfile");

    cli().arg("-o").arg(temp.path()).assert().success();

    temp.assert(".flex {\n  display: flex;\n}\n");
}

#[test]
fn test_cli_with_config() {
    let cwd = TempDir::new().expect("Failed to create tempdir");
    cwd.copy_from("tests", &["**/*.html"]).expect("Failed to copy fixtures");

    cli().arg("--cwd").arg(cwd.path()).arg("-o").arg("index.css").assert().success();

    cwd.child("index.css").assert(".flex {\n  display: flex;\n}\n");
}

#[test]
fn test_cli_with_watch() {
    let cwd = TempDir::new().expect("Failed to create tempdir");
    cwd.copy_from("tests", &["**/*.html"]).expect("Failed to copy fixtures");

    let path = cwd.path().to_owned();
    process::Command::cargo_bin("rswind_cli")
        .expect("Failed to build rswind_cli")
        .arg("--watch")
        .arg("--cwd")
        .arg(path)
        .arg("-o")
        .arg("index.css")
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap();

    until_updated!(
        read_to_string(cwd.child("index.css").path())
            .map(|s| s.split_whitespace().collect::<String>())
            .as_deref()
            .ok(),
        Some(".flex{display:flex;}")
    );

    cwd.child(PathBuf::from("fixtures").join("index.html"))
        .write_str("<div class=\"text-sm\"></div>")
        .expect("Failed to write to file");

    until_updated!(
        read_to_string(cwd.child("index.css").path())
            .map(|s| s.split_whitespace().collect::<String>())
            .as_deref()
            .ok(),
        Some(".flex{display:flex;}.text-sm{font-size:0.875rem;line-height:1.25rem;}")
    );
}
