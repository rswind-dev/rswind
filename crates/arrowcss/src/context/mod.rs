use std::{cmp::Ordering, fmt::Write, sync::Arc};

use cssparser::serialize_name;
use derive_more::{Deref, DerefMut};
use smallvec::SmallVec;
use smol_str::SmolStr;

use self::{
    utilities::{StaticUtility, UtilityStorage},
    variants::VariantStorage,
};
use crate::{
    common::{StrReplaceExt, StrSplitExt},
    css::rule::RuleList,
    ordering::OrderingKey,
    parsing::{candidate::CandidateParser, UtilityCandidate, VariantCandidate},
    process::{Utility, UtilityApplyResult, UtilityGroup, VariantOrdering},
    theme::{Theme, ThemeValue},
};
#[macro_use]
pub mod macros;
pub mod utilities;
pub mod variants;

// pub type VariantStorage = HashMap<SmolStr, Variant>;

#[derive(Default)]
pub struct Context {
    /// Storage for utilities
    pub utilities: UtilityStorage,

    /// Storage for variants
    pub variants: VariantStorage,

    /// Theme values
    pub theme: Theme,
}

/// The result of a utility generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateResult {
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

    pub additional_css: Option<Arc<RuleList>>,
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

impl From<GenerateResult> for CacheKey {
    fn from(res: GenerateResult) -> Self {
        Self { raw: res.raw, ordering: res.ordering, variants: res.variants }
    }
}

impl From<&GenerateResult> for CacheKey {
    fn from(res: &GenerateResult) -> Self {
        Self { raw: res.raw.clone(), ordering: res.ordering, variants: res.variants.clone() }
    }
}

impl PartialOrd for GenerateResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GenerateResult {
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
        match self.len().partial_cmp(&other.len()) {
            Some(Ordering::Equal) => self.0.partial_cmp(&other.0),
            non_eq => non_eq,
        }
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

impl Context {
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
    /// use arrowcss::Context;
    /// use arrowcss::css::{Decl, DeclList, ToCssString};
    ///
    /// let mut ctx = Context::new();
    ///
    /// ctx.add_static("flex", DeclList::from([Decl::new("display", "flex")]));
    ///
    /// let res = ctx.generate("flex").unwrap();
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
    /// use arrowcss::Context;
    /// use arrowcss::css::{Decl, DeclList, ToCssString};
    ///
    /// let mut ctx = Context::new();
    ///
    /// ctx.add_static("flex", DeclList::from([Decl::new("display", "flex")]));
    /// ctx.add_variant("hover", ["&:hover"]);
    ///
    /// let res = ctx.generate("hover:flex").unwrap();
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

    pub fn get_theme(&self, key: &str) -> Option<ThemeValue> {
        self.theme.get(key).cloned()
    }

    /// Try generate a utility with the given value
    pub fn generate(&self, value: &str) -> Option<GenerateResult> {
        // Try static utility first
        if let Some(UtilityApplyResult { rule: node, ordering, group, .. }) =
            self.utilities.try_apply(UtilityCandidate::with_key(value))
        {
            return Some(GenerateResult {
                raw: SmolStr::from(value),
                group,
                rule: fill_selector_placeholder(value, node.to_rule_list())?,
                ordering,
                variants: VariantOrder::default(),
                additional_css: None,
            });
        }

        let mut parts: SmallVec<[&str; 2]> = value.split_toplevel(b':')?;
        let utility = parts.pop()?;

        let utility_candidate = CandidateParser::new(utility).parse_utility(&self.utilities)?;

        let vs = parts
            .into_iter()
            .map(|v| CandidateParser::new(v).parse_variant(&self.variants))
            .collect::<Option<SmallVec<[_; 2]>>>()?;

        let variants = vs.iter().map(|v| v.processor.ordering).collect();

        let (nested, selector): (SmallVec<[_; 1]>, SmallVec<[_; 1]>) =
            vs.iter().partition(|v| v.processor.nested);

        let UtilityApplyResult { rule: node, ordering, group, additional_css } =
            self.utilities.try_apply(utility_candidate)?;

        let mut node = selector.iter().fold(node.to_rule_list(), |acc, cur| cur.handle(acc));

        node = fill_selector_placeholder(value, node)?;

        let node = nested.iter().fold(node, |acc, cur| cur.handle(acc));

        Some(GenerateResult {
            raw: SmolStr::from(value),
            rule: node,
            ordering,
            group,
            variants,
            additional_css,
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
