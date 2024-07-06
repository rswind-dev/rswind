use instance_code::build::provide_instance;
use rswind_core::build::{StaticUtilityConfig, UtilityInput};
use rswind_theme::codegen::ThemeCodegen;

fn main() {
    provide_instance::<ThemeCodegen>("theme", "preset/tailwind-theme.toml");
    provide_instance::<StaticUtilityConfig>("static_utilities", "preset/static-utilities.toml");
    provide_instance::<UtilityInput>("utilities", "preset/utilities.yaml");
}
