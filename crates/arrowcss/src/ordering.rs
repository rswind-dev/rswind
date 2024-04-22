use fxhash::FxHashMap as HashMap;
use lazy_static::lazy_static;
use smallvec::{smallvec, SmallVec};
use std::hash::Hash;

use crate::parser::GenerateResult;

#[derive(Debug)]
struct GroupItem {
    group_id: usize,
    id: usize,
}

#[derive(Debug)]
pub struct UtilityOrdering {
    ordering: HashMap<OrderingKey<String>, (GroupItem, usize)>,
    n: usize,
}

#[derive(Debug, Clone)]
pub struct OrderingItem<'a> {
    pub name: String,
    pub item: GenerateResult<'a>,
    variant_ordering: u128,
}

impl<'a> OrderingItem<'a> {
    pub fn new(
        name: String,
        item: GenerateResult<'a>,
        variant_ordering: u128,
    ) -> Self {
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
        Some(self.variant_ordering.cmp(&other.variant_ordering))
    }
}

impl Ord for OrderingItem<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.variant_ordering.cmp(&other.variant_ordering)
    }
}

pub struct OrderingMap<'a, 'i> {
    ordering: &'a UtilityOrdering,
    map: HashMap<usize, SmallVec<[Vec<OrderingItem<'i>>; 4]>>,
    unordered: Vec<OrderingItem<'i>>,
}

impl<'a, 'i> OrderingMap<'a, 'i> {
    pub fn new(ordering: &'a UtilityOrdering) -> Self {
        Self {
            ordering,
            map: HashMap::default(),
            unordered: vec![],
        }
    }

    pub fn insert_many(
        &mut self,
        items: impl IntoIterator<Item = OrderingItem<'i>>,
    ) {
        for key in items {
            if let Some((item, len)) =
                self.ordering.ordering.get(&key.item.ordering)
            {
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

        bare.into_iter().chain(variant.into_iter())
    }
}

impl UtilityOrdering {
    pub fn new() -> Self {
        Self {
            ordering: HashMap::default(),
            n: 0,
        }
    }

    pub fn add_order<'a>(
        &mut self,
        rule: impl IntoIterator<
            Item = OrderingKey<String>,
            IntoIter: ExactSizeIterator,
        >,
    ) -> usize {
        self.n += 1;
        let mut inner_n = 0;
        let iter = rule.into_iter();
        let len = iter.len();

        for key in iter {
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
            inner_n += 1;
        }
        self.n
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub enum OrderingKey<T> {
    Disorder(T),

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
}

impl Ord for OrderingKey<String> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = ORDERING.ordering.get(self);
        let b = ORDERING.ordering.get(other);

        todo!()
    }
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

    ordering
}
