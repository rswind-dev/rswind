use crate::context::Context;

pub mod dynamics;
pub mod statics;
pub mod variant;

pub fn load_preset(ctx: &mut Context) {
    statics::load_static_utilities(ctx);
    dynamics::load_dynamic_utilities(ctx);
    variant::load_variants(ctx);
}
