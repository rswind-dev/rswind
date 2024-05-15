use arrowcss::{app::Application, config::ArrowConfig, preset::preset_tailwind};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::fmt()
        .init();

    let mut app = Application::builder()
        .with_preset(preset_tailwind)
        .with_config(ArrowConfig::from_file("arrow.config").unwrap())
        .build();

    let css = app.run_with(["foo-blue-500/80", "foo-bar", "foo-bar-baz"]);
    println!("{}", css);
}
