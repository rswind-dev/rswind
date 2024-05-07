use arrowcss::{create_app, source::SourceInput};

fn main() {
    let mut app = create_app();
    let input = SourceInput::Html(include_str!("template_html"));
    let css = app.run(input);
    println!("{}", css);
}
