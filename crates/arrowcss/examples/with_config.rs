use arrowcss::{app::Application, config::ArrowConfig};
use config::Config;

fn main() {
    let mut app = Application::builder(
        Config::builder()
            .add_source(config::File::with_name("arrow.config"))
            .build()
            .unwrap()
            .try_deserialize::<ArrowConfig>()
            .unwrap(),
    )
    .init();
    let css = app.run_with(["foo-blue-500/80"]);
    println!("{}", css);
}
