use std::{env, process};

use colored::Colorize;
use rswind_cli::cli;

fn main() {
    if let Err(e) = cli(env::args()) {
        eprintln!("{}{}", "error: ".red().bold(), e);
        process::exit(1);
    }
}
