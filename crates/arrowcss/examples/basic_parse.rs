use arrowcss::create_app;
use arrowcss_extractor::{Extractable, Extractor, InputKind};

fn main() {
    let mut app = create_app();
    let input = Extractor::new(include_str!("template_html"), InputKind::Html);
    let css = app.run_with(input.extract());
    println!("{}", css);
}
