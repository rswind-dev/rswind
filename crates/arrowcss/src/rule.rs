use crate::{
    context::Context,
    parser::{extract_variants, Parse},
    utility::Utility,
    variant::Variant,
};

struct Rule {
    variants: Vec<Variant>,
    utility: Utility,
}

impl Parse<&str> for Rule {
    fn parse(ctx: &Context, input: &str) -> Option<Self> {
        let (variants, utility) = extract_variants(input);

        let utility = Utility::parse(ctx, &utility)?;
        let variants = variants
            .iter()
            .map(|v| Variant::parse(ctx, v))
            .collect::<Option<Vec<_>>>()?;

        Some(Self {
            variants,
            utility,
        })
    }
}
