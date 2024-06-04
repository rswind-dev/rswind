use derive_more::{Deref, DerefMut};
use rustc_hash::FxHashMap as HashMap;
use smol_str::SmolStr;

use crate::{
    css::rule::RuleList,
    parsing::VariantCandidate,
    process::{Variant, VariantOrdering},
};

#[derive(Default, Debug, Clone, Deref, DerefMut)]
pub struct VariantStorage {
    #[deref]
    #[deref_mut]
    map: HashMap<SmolStr, Variant>,
    order: u64,
}

impl VariantStorage {
    pub fn new() -> Self {
        Self { map: HashMap::default(), order: 0 }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    fn next_order(&mut self) -> VariantOrdering {
        self.order += 1;
        VariantOrdering::Insertion(self.order)
    }

    pub fn add_variant<T>(&mut self, key: impl Into<SmolStr>, matcher: T) -> &mut Self
    where
        T: IntoIterator,
        T::Item: Into<SmolStr>,
        T::IntoIter: ExactSizeIterator,
    {
        let order = self.next_order();
        self.map.insert(key.into(), Variant::new_static(matcher).with_ordering(order));
        self
    }

    pub fn add_variant_fn(
        &mut self,
        key: &str,
        func: fn(RuleList, &VariantCandidate) -> RuleList,
        nested: bool,
    ) -> &Self {
        let order = self.next_order();
        self.map.insert(key.into(), Variant::new_dynamic(func, nested).with_ordering(order));
        self
    }

    pub fn add_variant_composable(
        &mut self,
        key: &str,
        handler: fn(RuleList, &VariantCandidate) -> RuleList,
    ) -> &mut Self {
        let order = self.next_order();
        self.map.insert(key.into(), Variant::new_composable(handler).with_ordering(order));
        self
    }

    pub fn get(&self, key: &str) -> Option<&Variant> {
        self.map.get(key)
    }
}
