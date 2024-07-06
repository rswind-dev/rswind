use std::sync::Arc;

use instance_code::inject_instance;
use rswind_core::theme::{values::*, Theme, ThemeMap};
use rustc_hash::FxHashMap;

use rswind_core::DesignSystem;

#[allow(clippy::disallowed_types)]
fn theme() -> Theme {
    inject_instance!("theme")
}

pub fn load_theme(design: &mut DesignSystem) {
    design.theme = theme();
}
