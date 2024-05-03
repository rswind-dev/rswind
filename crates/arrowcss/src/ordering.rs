use std::hash::Hash;

use fxhash::FxHashMap as HashMap;
use lazy_static::lazy_static;
use smallvec::{smallvec, SmallVec};

use crate::parser::GenerateResult;

#[derive(Debug)]
struct GroupItem {
    group_id: usize,
    id: usize,
}

#[derive(Debug, Default)]
pub struct UtilityOrdering {
    ordering: HashMap<OrderingKey, (GroupItem, usize)>,
    n: usize,
}

#[derive(Debug, Clone)]
pub struct OrderingItem<'a> {
    pub name: String,
    pub item: GenerateResult<'a>,
    variant_ordering: u128,
}

impl<'a> OrderingItem<'a> {
    pub fn new(name: String, item: GenerateResult<'a>, variant_ordering: u128) -> Self {
        Self {
            name,
            item,
            variant_ordering,
        }
    }
}

impl PartialEq for OrderingItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.variant_ordering == other.variant_ordering
    }
}

impl Eq for OrderingItem<'_> {}

impl PartialOrd for OrderingItem<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderingItem<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.variant_ordering.cmp(&other.variant_ordering) {
            std::cmp::Ordering::Equal => self.name.cmp(&other.name),
            other => other,
        }
    }
}

#[derive(Default)]
pub struct OrderingMap<'i> {
    ordering: UtilityOrdering,
    map: HashMap<usize, SmallVec<[Vec<OrderingItem<'i>>; 4]>>,
    unordered: Vec<OrderingItem<'i>>,
}

impl<'i> OrderingMap<'i> {
    pub fn new(ordering: UtilityOrdering) -> Self {
        Self {
            ordering,
            map: HashMap::default(),
            unordered: vec![],
        }
    }

    pub fn insert_many(&mut self, items: impl IntoIterator<Item = OrderingItem<'i>>) {
        for key in items {
            if let Some((item, len)) = self.ordering.ordering.get(&key.item.ordering) {
                self.map
                    .entry(item.group_id)
                    .or_insert_with(|| smallvec![vec![]; *len])[item.id]
                    .push(key);
            } else {
                self.unordered.push(key);
            }
        }
        self.unordered.sort();
    }

    pub fn get_ordered(&self) -> impl Iterator<Item = &OrderingItem<'i>> {
        let (bare, mut variant): (Vec<_>, Vec<_>) = self
            .map
            .iter()
            .flat_map(|(_, v)| v.iter())
            .flat_map(|v| v.iter())
            .chain(self.unordered.iter())
            .partition(|v| v.variant_ordering == 0);

        variant.sort();

        bare.into_iter().chain(variant)
    }
}

impl UtilityOrdering {
    pub fn new() -> Self {
        Self {
            ordering: HashMap::default(),
            n: 0,
        }
    }

    pub fn add_order(
        &mut self,
        rule: impl IntoIterator<Item = OrderingKey, IntoIter: ExactSizeIterator>,
    ) -> usize {
        self.n += 1;
        let iter = rule.into_iter();
        let len = iter.len();

        for (inner_n, key) in iter.enumerate() {
            self.ordering.insert(
                key,
                (
                    GroupItem {
                        group_id: self.n,
                        id: inner_n,
                    },
                    len,
                ),
            );
        }
        self.n
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum OrderingKey {
    Disorder,

    Translate,
    TranslateAxis,
    Scale,
    ScaleAxis,
    Rotate,
    RotateAxis,
    Skew,
    SkewAxis,
    Transform,

    Margin,
    MarginAxis,
    MarginSide,

    Padding,
    PaddingAxis,
    PaddingSide,

    Rounded,
    RoundedSide,
    RoundedCorner,

    Inset,
    InsetAxis,
    InsetSide,
    PositionSide,

    BorderSpacing,
    BorderSpacingAxis,

    BorderColor,
    BorderColorAxis,
    BorderColorSide,

    BorderWidth,
    BorderWidthAxis,
    BorderWidthSide,

    Size,
    SizeAxis,
}

lazy_static! {
    pub static ref ORDERING: UtilityOrdering = create_ordering();
}

pub fn create_ordering() -> UtilityOrdering {
    use crate::ordering::OrderingKey::*;

    let mut ordering = UtilityOrdering::new();
    ordering.add_order([Inset, InsetAxis, InsetSide, PositionSide]);
    ordering.add_order([
        Translate,
        TranslateAxis,
        Scale,
        ScaleAxis,
        Rotate,
        RotateAxis,
        Skew,
        SkewAxis,
        Transform,
    ]);
    ordering.add_order([Margin, MarginAxis, MarginSide]);
    ordering.add_order([Padding, PaddingAxis, PaddingSide]);
    ordering.add_order([Rounded, RoundedSide, RoundedCorner]);
    ordering.add_order([BorderSpacing, BorderSpacingAxis]);
    ordering.add_order([BorderColor, BorderColorAxis, BorderColorSide]);
    ordering.add_order([BorderWidth, BorderWidthAxis, BorderWidthSide]);
    ordering.add_order([Size, SizeAxis]);

    ordering
}
