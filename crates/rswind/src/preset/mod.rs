use tracing::{enabled, info, Level};

use rswind_core::design::DesignSystem;

pub mod theme;
pub mod utility;
pub mod variant;

pub fn tailwind_theme(design: &mut DesignSystem) {
    let initial_length = enabled!(Level::INFO).then_some(0);

    theme::load_theme(design);

    if enabled!(Level::INFO) {
        let length = initial_length.unwrap_or(0);
        info!(theme = design.theme.len() - length, "Loaded tailwind preset",);
    }
}

pub fn tailwind_preset(design: &mut DesignSystem) {
    let initial_length = if enabled!(Level::INFO) {
        Some((design.utilities.len(), design.variants.len()))
    } else {
        None
    };

    utility::load_static_utilities(design);
    utility::load_dynamic_utilities(design);
    variant::load_variants(design);

    if enabled!(Level::INFO) {
        let (utilities, variants) = initial_length.unwrap_or((0, 0));
        info!(
            utilities = design.utilities.len() - utilities,
            variants = design.variants.len() - variants,
            "Loaded tailwind preset",
        );
    }
}
