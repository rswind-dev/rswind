use fxhash::FxHashMap as HashMap;
use smallvec::{smallvec, SmallVec};
use std::hash::Hash;

use crate::css::rule::RuleList;

#[derive(Debug)]
struct GroupItem {
    group_id: usize,
    id: usize,
}

#[derive(Debug)]
pub struct UtilityOrdering<T> {
    ordering: HashMap<T, (GroupItem, usize)>,
    n: usize,
}

pub struct OrderingMap<'a, T, I> {
    ordering: &'a UtilityOrdering<T>,
    // key: group_id
    map: HashMap<usize, SmallVec<[Vec<I>; 4]>>,
    unordered: Vec<I>,
}

pub trait Orderable<T> {
    fn order_key(&self) -> T;
}

impl Orderable<OrderingKey> for OrderingKey {
    fn order_key(&self) -> OrderingKey {
        *self
    }
}

impl<'a> Orderable<OrderingKey>
    for (String, (RuleList<'a>, OrderingKey))
{
    fn order_key(&self) -> OrderingKey {
        self.1 .1
    }
}

impl<'a, T: Hash + Eq, Item: Orderable<T> + Clone> OrderingMap<'a, T, Item> {
    pub fn new(ordering: &'a UtilityOrdering<T>) -> Self {
        Self {
            ordering,
            map: HashMap::default(),
            unordered: vec![],
        }
    }

    pub fn insert_many(&mut self, items: impl IntoIterator<Item = Item>) {
        for key in items {
            if let Some((item, len)) =
                self.ordering.ordering.get(&key.order_key())
            {
                self.map
                    .entry(item.group_id)
                    .or_insert_with(|| smallvec![vec![]; *len])[item.id]
                    .push(key);
            } else {
                self.unordered.push(key);
            }
        }
    }

    pub fn get_ordered(&self) -> impl Iterator<Item = &Item> {
        self.map
            .iter()
            .flat_map(|(_, v)| v.iter())
            .flat_map(|v| v.iter())
            .chain(self.unordered.iter())
    }
}

impl<T: Hash + Eq> UtilityOrdering<T> {
    pub fn new() -> Self {
        Self {
            ordering: HashMap::default(),
            n: 0,
        }
    }

    pub fn add_order<'a>(
        &mut self,
        rule: impl IntoIterator<Item = T, IntoIter: ExactSizeIterator>,
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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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
}

pub fn create_ordering() -> UtilityOrdering<OrderingKey> {
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
