use std::hash::Hash;
use std::sync::Arc;

use fxhash::FxHashMap as HashMap;

use crate::map;
use crate::theme::Theme;
use crate::themes::colors::colors;
use crate::themes::spacing::spacing;

mod colors;
mod spacing;

macro_rules! create_theme {
    ($($key:expr => $value:expr),*) => {
        {
            let mut m = fxhash::FxHashMap::default();
            $(
                m.insert($key.to_string(), $value);
            )*
            m
        }
    };
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

pub fn theme() -> Theme<'static> {
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
        }),
        "blur" => map! {
            "0" => "0",
            "none" => "0",
            "sm" => "4px",
            "DEFAULT" => "8px",
            "md" => "12px",
            "lg" => "16px",
            "xl" => "24px",
            "2xl" => "40px",
            "3xl" => "64px"
        }.into(),
        "ringWidth" => map! {
            "DEFAULT" => "3px",
            "0" => "0px",
            "1" => "1px",
            "2" => "2px",
            "4" => "4px",
            "8" => "8px"
        }.into(),
        "ringOffsetWidth" => map! {
            "0" => "0px",
            "1" => "1px",
            "2" => "2px",
            "4" => "4px",
            "8" => "8px"
        }.into(),
        "backgroundPosition" => map! {
            "bottom" => "bottom",
            "center" => "center",
            "left" => "left",
            "left-bottom" => "left bottom",
            "left-top" => "left top",
            "right" => "right",
            "right-bottom" => "right bottom",
            "right-top" => "right top",
            "top" => "top"
        }.into(),
        "backgroundSize" => map! {
            "auto" => "auto",
            "cover" => "cover",
            "contain" => "contain"
        }.into(),
        "backgroundImage" => map! {
            "none" => "none",
            "gradient-to-t" => "linear-gradient(to top, var(--tw-gradient-stops))",
            "gradient-to-tr" => "linear-gradient(to top right, var(--tw-gradient-stops))",
            "gradient-to-r" => "linear-gradient(to right, var(--tw-gradient-stops))",
            "gradient-to-br" => "linear-gradient(to bottom right, var(--tw-gradient-stops))",
            "gradient-to-b" => "linear-gradient(to bottom, var(--tw-gradient-stops))",
            "gradient-to-bl" => "linear-gradient(to bottom left, var(--tw-gradient-stops))",
            "gradient-to-l" => "linear-gradient(to left, var(--tw-gradient-stops))",
            "gradient-to-tl" => "linear-gradient(to top left, var(--tw-gradient-stops))"
        }.into(),
        "opacity" => map! {
            "0" => "0",
            "5" => "0.05",
            "10" => "0.1",
            "20" => "0.2",
            "25" => "0.25",
            "30" => "0.3",
            "40" => "0.4",
            "50" => "0.5",
            "60" => "0.6",
            "70" => "0.7",
            "75" => "0.75",
            "80" => "0.8",
            "90" => "0.9",
            "95" => "0.95",
            "100" => "1"
        }.into(),
        "lineClamp" => map! {
            "1" => "1",
            "2" => "2",
            "3" => "3",
            "4" => "4",
            "5" => "5",
            "6" => "6"
        }.into(),
        "borderWidth" => map! {
            "DEFAULT" => "1px",
            "0" => "0",
            "2" => "2px",
            "4" => "4px",
            "8" => "8px"
        }.into(),
        "breakpoints" => map! {
            "sm" => "640px",
            "md" => "768px",
            "lg" => "1024px",
            "xl" => "1280px",
            "2xl" => "1536px"
        }.into(),
        "lineHeight" => map! {
            "3" => "0.75rem",
            "4" => "1rem",
            "5" => "1.25rem",
            "6" => "1.5rem",
            "7" => "1.75rem",
            "8" => "2rem",
            "9" => "2.25rem",
            "10" => "2.5rem",
            "none" => "1",
            "tight" => "1.25",
            "snug" => "1.375",
            "normal" => "1.5",
            "relaxed" => "1.625",
            "loose" => "2"
        }.into(),
        "animate" => map! {
            "none" => "none",
            "spin" => "spin 1s linear infinite",
            "ping" => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite",
            "pulse" => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
            "bounce" => "bounce 1s infinite"
        }.into(),
        "fontSize" => map! {
            "xs" => "0.75rem",
            "sm" => "0.875rem",
            "base" => "1rem",
            "lg" => "1.125rem",
            "xl" => "1.25rem",
            "2xl" => "1.5rem",
            "3xl" => "1.875rem",
            "4xl" => "2.25rem",
            "5xl" => "3rem",
            "6xl" => "4rem"
        }.into(),
        "fontSize:lineHeight" => map! {
            "xs" => "1.5",
            "sm" => "1.5",
            "base" => "1.5",
            "lg" => "1.5",
            "xl" => "1.5",
            "2xl" => "1.5",
            "3xl" => "1.25",
            "4xl" => "1.25",
            "5xl" => "1.25",
            "6xl" => "1.125"
        }.into()
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
