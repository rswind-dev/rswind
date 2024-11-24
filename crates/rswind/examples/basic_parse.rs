use rswind::extract::{Extractable, Extractor, InputKind};
use rswind::generator::GeneratorBuilder;
use rswind::preset::{tailwind_preset, tailwind_theme};

fn main() {
    let mut app = GeneratorBuilder::new()
        .with_theme(tailwind_theme)
        .with_preset(tailwind_preset)
        .build_processor()
        .unwrap();
    let input =
        Extractor::new(r#"<div class="flex text-sm text-blue-500"></div>"#, InputKind::Html)
            .extract();
    let css = app.run_with(input);
    println!("{}", css.css);
}
