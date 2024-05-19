use tracing::{enabled, info, Level};

use crate::context::Context;

pub mod colors;
pub mod dynamics;
pub mod spacing;
pub mod statics;
pub mod theme;
pub mod variant;

pub fn load_preset(ctx: &mut Context) {
    theme::load_theme(ctx);
    statics::load_static_utilities(ctx);
    dynamics::load_dynamic_utilities(ctx);
    variant::load_variants(ctx);
}

pub trait Preset {
    fn load_preset(self: Box<Self>, ctx: &mut Context);
}

impl<T> Preset for T
where
    T: FnOnce(&mut Context) + 'static,
{
    fn load_preset(self: Box<Self>, ctx: &mut Context) {
        (*self)(ctx);
    }
}

pub fn preset_tailwind(ctx: &mut Context) {
    let inital_length = if enabled!(Level::INFO) {
        Some((ctx.theme.len(), ctx.utilities.len(), ctx.variants.len()))
    } else {
        None
    };

    theme::load_theme(ctx);
    statics::load_static_utilities(ctx);
    dynamics::load_dynamic_utilities(ctx);
    variant::load_variants(ctx);

    if enabled!(Level::INFO) {
        let (theme, utilities, variants) = inital_length.unwrap_or((0, 0, 0));
        info!(
            theme = ctx.theme.len() - theme,
            utilities = ctx.utilities.len() - utilities,
            variants = ctx.variants.len() - variants,
            "Loaded tailwind preset",
        );
    }
}
