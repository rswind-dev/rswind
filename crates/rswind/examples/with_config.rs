use rswind::{
    config::GeneratorConfig,
    preset::{tailwind_preset, tailwind_theme},
    processor::GeneratorProcessor,
};

fn main() {
    tracing_subscriber::fmt::fmt().init();

    let mut app = GeneratorProcessor::builder()
        .with_theme(tailwind_theme)
        .with_preset(tailwind_preset)
        .with_config(GeneratorConfig::from_file("rswind.config").unwrap())
        .build_processor()
        .unwrap();

    let css = app.run_with(["foo-blue-500/80", "foo-bar", "foo-bar-baz"]);
    println!("{}", css.css);
}
