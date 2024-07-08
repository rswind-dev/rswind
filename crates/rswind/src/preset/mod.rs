use tracing::{enabled, info, Level};

use rswind_core::context::DesignSystem;

pub mod theme;
pub mod utility;
pub mod variant;

pub fn preset_tailwind(design: &mut DesignSystem) {
    let initial_length = if enabled!(Level::INFO) {
        Some((design.theme.len(), design.utilities.len(), design.variants.len()))
    } else {
        None
    };

    theme::load_theme(design);

    utility::load_static_utilities(design);
    utility::load_dynamic_utilities(design);
    variant::load_variants(design);

    if enabled!(Level::INFO) {
        let (theme, utilities, variants) = initial_length.unwrap_or((0, 0, 0));
        info!(
            theme = design.theme.len() - theme,
            utilities = design.utilities.len() - utilities,
            variants = design.variants.len() - variants,
            "Loaded tailwind preset",
        );
    }
}
