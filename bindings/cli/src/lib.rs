#[macro_use]
extern crate napi_derive;

#[napi]
pub fn run_cli(args: Vec<String>) -> napi::Result<()> {
    rswind_cli::cli(args).map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}
