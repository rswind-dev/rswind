use rswind::{config::AppConfig, processor::GeneratorProcessor, preset::preset_tailwind};

fn main() {
    tracing_subscriber::fmt::fmt().init();

    let mut app = GeneratorProcessor::builder()
        .with_preset(preset_tailwind)
        .with_config(AppConfig::from_file("arrow.config").unwrap())
        .build_processor()
        .unwrap();

    let css = app.run_with(["foo-blue-500/80", "foo-bar", "foo-bar-baz"]);
    println!("{}", css);
}
