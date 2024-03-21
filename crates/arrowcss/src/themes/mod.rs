use std::collections::HashMap;
use std::sync::Arc;

use crate::map;
use crate::theme::Theme;
use crate::themes::colors::colors;
use crate::themes::spacing::spacing;
use serde::Deserialize;
use serde_json::json;
use std::hash::Hash;

mod colors;
mod spacing;

macro_rules! create_theme {
    ($($key:expr => $value:expr),*) => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key.to_string(), $value);
            )*
            m
        }
    };
}

fn extend<K: Clone + Eq + Hash, V: Clone>(
    mut a: Arc<HashMap<K, V>>,
    mut b: HashMap<K, V>,
) -> Arc<HashMap<K, V>> {
    Arc::make_mut(&mut a).extend(b.drain());
    a
}

trait ArcExtend<K, V> {
    fn arc_extend(&mut self, other: HashMap<K, V>) -> Arc<HashMap<K, V>>;
}

impl<K: Clone + Eq + Hash, V: Clone> ArcExtend<K, V> for Arc<HashMap<K, V>> {
    fn arc_extend(&mut self, mut other: HashMap<K, V>) -> Arc<HashMap<K, V>> {
        Arc::make_mut(self).extend(other.drain());
        self.clone()
    }
}

pub fn theme() -> Theme {
    create_theme! {
        "colors" => colors(),
        "spacing" => spacing(),
        "translate" => spacing().arc_extend(map! {
            "1/2" => "50%",
            "1/3" => "33.333333%",
            "2/3" => "66.666667%",
            "1/4" => "25%",
            "2/4" => "50%",
            "3/4" => "75%",
            "full" => "100%"
        })
    }
    .into()
}

#[cfg(test)]
mod tests {
    use crate::map;

    use super::*;

    #[test]
    fn test_theme() {
        let mut translate = map! {
            "1/2" => "50%",
            "1/3" => "33.333333%",
            "2/3" => "66.666667%",
            "1/4" => "25%",
            "2/4" => "50%",
            "3/4" => "75%",
            "full" => "100%"
        };
        let mut spacing2 = spacing();
        Arc::make_mut(&mut spacing2).extend(translate.drain());

        println!("{:?}", translate);
        println!("{:#?}", spacing2.keys().len());
        println!("{:#?}", spacing().keys().len());
    }
}
