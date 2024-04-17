use fxhash::FxHashMap as HashMap;
use smallvec::{smallvec, SmallVec};

#[derive(Debug)]
struct GroupItem {
    group_id: usize,
    id: usize,
}

#[derive(Debug)]
pub struct UtilityOrdering {
    ordering: HashMap<&'static str, (GroupItem, usize)>,
    n: usize,
}

pub struct OrderingMap<'a> {
    ordering: &'a UtilityOrdering,
    // key: group_id
    map: HashMap<usize, SmallVec<[Vec<&'static str>; 4]>>,
    unordered: Vec<&'static str>,
}

impl<'a> OrderingMap<'a> {
    pub fn new(ordering: &'a UtilityOrdering) -> Self {
        Self {
            ordering,
            map: HashMap::default(),
            unordered: vec![],
        }
    }

    pub fn insert_many(
        &mut self,
        items: impl IntoIterator<Item = &'static str>,
    ) {
        for key in items {
            if let Some((item, len)) = self.ordering.ordering.get(key) {
                self.map
                    .entry(item.group_id)
                    .or_insert_with(|| smallvec![vec![]; *len])[item.id]
                    .push(key);
            } else {
                self.unordered.push(key);
            }
        }
    }

    pub fn get_ordered(&self) -> impl Iterator<Item = &&'static str> {
        self.map
            .iter()
            .flat_map(|(_, v)| v.iter())
            .flat_map(|v| v.iter())
            .chain(self.unordered.iter())
    }
}

impl Default for UtilityOrdering {
    fn default() -> Self {
        Self::new()
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
        rule: impl IntoIterator<Item = &'static str, IntoIter: ExactSizeIterator>,
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

pub fn create_ordering() -> UtilityOrdering {
    let mut ordering = UtilityOrdering::new();
    ordering.add_order(["inset", "inset-axis", "inset-side", "position-side"]);
    ordering.add_order(["margin", "margin-axis", "margin-side"]);
    ordering.add_order(["padding", "padding-axis", "padding-side"]);
    ordering.add_order([
        "translate",
        "translate-axis",
        "scale",
        "scale-axis",
        "rotate",
        "rotate-axis",
        "skew",
        "skew-axis",
        "transform",
    ]);
    ordering.add_order(["rounded", "rounded-side", "rounded-corner"]);

    ordering
}
