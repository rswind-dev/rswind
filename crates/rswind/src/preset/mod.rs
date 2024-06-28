use tracing::{enabled, info, Level};

use crate::context::DesignSystem;

pub mod dynamics;
pub mod statics;
pub mod theme;
pub mod variant;

pub trait Preset {
    fn load_preset(self: Box<Self>, design: &mut DesignSystem);
}

impl<T> Preset for T
where
    T: FnOnce(&mut DesignSystem) + 'static,
{
    fn load_preset(self: Box<Self>, design: &mut DesignSystem) {
        (*self)(design);
    }
}

pub fn preset_tailwind(design: &mut DesignSystem) {
    let initial_length = if enabled!(Level::INFO) {
        Some((design.theme.len(), design.utilities.len(), design.variants.len()))
    } else {
        None
    };

    // theme::load_theme(design);
    statics::load_static_utilities(design);
    dynamics::load_dynamic_utilities(design);
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
