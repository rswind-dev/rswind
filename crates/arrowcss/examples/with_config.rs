use arrowcss::{app::Application, config::ArrowConfig};
use config::Config;

fn main() -> Result<(), anyhow::Error> {
    let mut app = Application::builder(
        Config::builder()
            .add_source(config::File::with_name("arrow.config"))
            .build()?
            .try_deserialize::<ArrowConfig>()?,
    )
    .init();
    let css = app.run_with(["foo-blue-500/80", "foo-bar", "foo-bar-baz"]);
    println!("{}", css);

    Ok(())
}
