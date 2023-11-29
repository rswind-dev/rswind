use arrowcss::generate;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn gen(input: String) -> String {
  generate(input)
}
