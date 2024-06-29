use std::sync::Arc;

use instance_code::inject_instance;
use rswind_css::{rule::RuleList, Decl, Rule};
use rswind_theme::{values::*, Theme, ThemeMap};
use rustc_hash::FxHashMap;

use crate::DesignSystem;

#[allow(clippy::disallowed_types)]
fn theme() -> Theme {
    inject_instance!("theme")
}

pub fn load_theme(design: &mut DesignSystem) {
    design.theme = theme();
}
