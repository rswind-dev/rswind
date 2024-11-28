use std::{cmp::Ordering, fmt::Write, sync::Arc};

use cssparser::serialize_name;
use derive_more::{Deref, DerefMut};
use rswind_css::rule::RuleList;
use rswind_theme::{Theme, ThemeMap};
use smallvec::SmallVec;
use smol_str::SmolStr;
use tracing::debug;

use self::{
    utilities::{StaticUtility, UtilityStorage},
    variants::VariantStorage,
};
use crate::{
    common::{StrReplaceExt, StrSplitExt},
    ordering::OrderingKey,
    parse::{candidate::CandidateParser, UtilityCandidate, VariantCandidate},
    process::{Utility, UtilityApplyResult, UtilityGroup, VariantOrdering},
};

pub mod utilities;
pub mod variants;

#[derive(Default)]
pub struct DesignSystem {
    /// Storage for utilities
    pub utilities: UtilityStorage,

    /// Storage for variants
    pub variants: VariantStorage,

    /// Theme values
    pub theme: Theme,
}

impl Extend<(SmolStr, Utility)> for DesignSystem {
    fn extend<T: IntoIterator<Item = (SmolStr, Utility)>>(&mut self, iter: T) {
        for (key, utility) in iter {
            self.add_utility(&key, utility);
        }
    }
}

/// The result of a utility generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedUtility {
    pub raw: SmolStr,
    /// The generated rule
    pub rule: RuleList,

    /// The grouping of the utility, if any
    pub group: Option<UtilityGroup>,

    /// The ordering key, will be [`OrderingKey::Disorder`] if not set
    pub ordering: OrderingKey,

    /// The variants in the utility, collect to sort them later
    /// Ordering: len -> variant order
    pub variants: VariantOrder,

    pub extra_css: Option<Arc<RuleList>>,
}

/// We can use the derived `PartialOrd` and `Ord` implementations
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CacheKey {
    pub variants: VariantOrder,

    pub ordering: OrderingKey,

    pub raw: SmolStr,
}

impl CacheKey {
    pub fn new_grouped(key: SmolStr) -> Self {
        Self { raw: key, ordering: OrderingKey::Grouped, variants: VariantOrder::default() }
    }

    pub fn new_property(key: SmolStr) -> Self {
        Self { raw: key, ordering: OrderingKey::Property, variants: VariantOrder::default() }
    }
}

impl From<GeneratedUtility> for CacheKey {
    fn from(res: GeneratedUtility) -> Self {
        Self { raw: res.raw, ordering: res.ordering, variants: res.variants }
    }
}

impl From<&GeneratedUtility> for CacheKey {
    fn from(res: &GeneratedUtility) -> Self {
        Self { raw: res.raw.clone(), ordering: res.ordering, variants: res.variants.clone() }
    }
}

impl PartialOrd for GeneratedUtility {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GeneratedUtility {
    fn cmp(&self, other: &Self) -> Ordering {
        self.variants
            .cmp(&other.variants)
            .then_with(|| self.ordering.cmp(&other.ordering))
            .then_with(|| self.raw.cmp(&other.raw))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deref, DerefMut)]
pub struct VariantOrder(Vec<VariantOrdering>);

impl FromIterator<VariantOrdering> for VariantOrder {
    fn from_iter<T: IntoIterator<Item = VariantOrdering>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl PartialOrd for VariantOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VariantOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.len().cmp(&other.len()) {
            Ordering::Equal => self.0.cmp(&other.0),
            non_eq => non_eq,
        }
    }
}

impl DesignSystem {
    pub fn new() -> Self {
        Self {
            variants: VariantStorage::default(),
            utilities: UtilityStorage::default(),
            theme: Theme::default(),
        }
    }

    /// Add a static utility
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use rswind_core::DesignSystem;
    /// use rswind_css::{Decl, DeclList, ToCssString};
    ///
    /// let mut design = DesignSystem::new();
    ///
    /// design.add_static("flex", DeclList::from([Decl::new("display", "flex")]));
    ///
    /// let res = design.generate("flex").unwrap();
    ///
    /// assert_eq!(res.rule.to_css_minified(), ".flex{display:flex;}");
    ///
    /// ```
    pub fn add_static(
        &mut self,
        key: impl Into<SmolStr>,
        value: impl Into<StaticUtility>,
    ) -> &Self {
        self.utilities.add_static(key.into(), value.into());
        self
    }

    /// Add a static variant
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use rswind_core::DesignSystem;
    /// use rswind_css::{Decl, DeclList, ToCssString};
    ///
    /// let mut design = DesignSystem::new();
    ///
    /// design.add_static("flex", DeclList::from([Decl::new("display", "flex")]));
    /// design.add_variant("hover", ["&:hover"]);
    ///
    /// let res = design.generate("hover:flex").unwrap();
    ///
    /// assert_eq!(res.rule.to_css_minified(), ".hover\\:flex:hover{display:flex;}");
    ///
    /// ```
    pub fn add_variant<T>(&mut self, key: impl Into<SmolStr>, matcher: T) -> &mut Self
    where
        T: IntoIterator,
        T::Item: Into<SmolStr>,
        T::IntoIter: ExactSizeIterator,
    {
        self.variants.add_variant(key, matcher);
        self
    }

    pub fn add_variant_fn(
        &mut self,
        key: &str,
        func: fn(RuleList, &VariantCandidate) -> RuleList,
        nested: bool,
    ) -> &Self {
        self.variants.add_variant_fn(key, func, nested);
        self
    }

    pub fn add_variant_composable(
        &mut self,
        key: &str,
        handler: fn(RuleList, &VariantCandidate) -> RuleList,
    ) -> &mut Self {
        self.variants.add_variant_composable(key, handler);
        self
    }

    pub fn add_utility(&mut self, key: &str, utility: Utility) {
        self.utilities.add(key.into(), utility);
    }

    pub fn get_theme(&self, key: &str) -> Option<Arc<ThemeMap>> {
        self.theme.get(key).cloned()
    }

    /// Try generate a utility with the given value
    pub fn generate(&self, value: &str) -> Option<GeneratedUtility> {
        // Try static utility first
        if let Some(UtilityApplyResult { rule: node, ordering, group, extra_css, .. }) =
            self.utilities.try_apply(UtilityCandidate::with_key(value))
        {
            return Some(GeneratedUtility {
                raw: SmolStr::from(value),
                group,
                rule: fill_selector_placeholder(value, node.to_rule_list())?,
                ordering,
                variants: VariantOrder::default(),
                extra_css,
            });
        }

        let mut parts: SmallVec<[&str; 2]> = value.split_toplevel(b':')?;
        let utility = parts.pop()?;

        let utility_candidate = CandidateParser::new(utility).parse_utility(&self.utilities)?;

        debug!(?utility_candidate);

        let vs = parts
            .into_iter()
            .map(|v| CandidateParser::new(v).parse_variant(&self.variants))
            .collect::<Option<SmallVec<[_; 2]>>>()?;

        let variants = vs.iter().map(|v| v.processor.ordering).collect();

        let (nested, selector): (SmallVec<[_; 1]>, SmallVec<[_; 1]>) =
            vs.iter().partition(|v| v.processor.nested);

        let UtilityApplyResult { rule: node, ordering, group, extra_css } =
            self.utilities.try_apply(utility_candidate)?;

        // TODO: rev()? variants order in tailwind v3 and v4 are different
        let mut node = selector.iter().fold(node.to_rule_list(), |acc, cur| cur.handle(acc));

        node = fill_selector_placeholder(value, node)?;

        let node = nested.iter().fold(node, |acc, cur| cur.handle(acc));

        Some(GeneratedUtility {
            raw: SmolStr::from(value),
            rule: node,
            ordering,
            group,
            variants,
            extra_css,
        })
    }
}

fn fill_selector_placeholder(value: &str, node: RuleList) -> Option<RuleList> {
    let mut writer = smol_str::Writer::new();
    writer.write_str(".").ok()?;
    serialize_name(value, &mut writer).ok()?;
    let w = SmolStr::from(writer);

    Some(node.modify_with(|s| s.replace_char('&', &w)))
}
