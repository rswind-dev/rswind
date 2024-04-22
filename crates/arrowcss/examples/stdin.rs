use arrowcss::app::{get_files, Application};

fn main() {
    let mut app = Application::new().unwrap();
    app.init().run_parallel(
        get_files("examples/html").as_slice(),
        Some("examples/index.css"),
    );
}
