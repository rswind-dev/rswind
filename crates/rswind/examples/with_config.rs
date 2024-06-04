use rswind::{config::AppConfig, generator::Generator, preset::preset_tailwind};

fn main() {
    tracing_subscriber::fmt::fmt().init();

    let mut app = Generator::builder()
        .with_preset(preset_tailwind)
        .with_config(AppConfig::from_file("arrow.config").unwrap())
        .build_generator()
        .unwrap();

    let css = app.run_with(["foo-blue-500/80", "foo-bar", "foo-bar-baz"]);
    println!("{}", css);
}
