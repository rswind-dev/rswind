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
