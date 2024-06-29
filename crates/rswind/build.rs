use instance_code::build::provide_instance;
use rswind_theme::codegen::ThemeCodegen;

fn main() {
    provide_instance::<ThemeCodegen>("theme", "preset/tailwind-theme.toml");
}
