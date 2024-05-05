use arrowcss::{create_app, extract::SourceInput};

fn main() {
    let mut app = create_app();
    let input = SourceInput::Html(r#"<div class="flex">"#);
    let css = app.run(input);
    println!("{}", css);
}
