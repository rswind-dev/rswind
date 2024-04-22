use arrowcss::app::Application;

fn main() {
    let mut app = Application::new().unwrap();
    app.init()
        .run_parallel("examples/html", Some("examples/index.css"));
}
