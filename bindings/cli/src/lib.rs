#[macro_use]
extern crate napi_derive;

#[napi]
pub fn run_cli(args: Vec<String>) {
    rswind_cli::cli(args)
}
