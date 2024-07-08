use rswind::extract::{Extractable, Extractor, InputKind};
use rswind::generator::GeneratorBuilder;
use rswind::preset::preset_tailwind;

fn main() {
    let mut app = GeneratorBuilder::new().with_preset(preset_tailwind).build_processor().unwrap();
    let input =
        Extractor::new(r#"<div class="flex text-sm text-blue-500"></div>"#, InputKind::Html)
            .extract();
    let css = app.run_with(input);
    println!("{}", css.css);
}
