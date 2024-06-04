use rswind::create_generator;
use rswind_extractor::{Extractable, Extractor, InputKind};

fn main() {
    let mut app = create_generator();
    let input = Extractor::new(r#"<div class="flex"></div>"#, InputKind::Html).extract();
    let css = app.run_with(input);
    println!("{}", css);
}
