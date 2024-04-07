pub struct VariantStorage<'c> {
    pub variants: HashMap<String, VariantHandler<'c>>,
    pub static_variants: HashMap<String, VariantHandler<'c>>,
}

