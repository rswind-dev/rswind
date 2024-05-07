use rustc_hash::FxHashSet;

pub mod css;
pub mod cursor;
pub mod ecma;
pub mod html;
pub mod item;

pub trait Extractor<'i> {
    fn extract(&'i self) -> Box<dyn Iterator<Item = &'i str> + 'i>;
}

pub struct BasicExtractor<'i> {
    pub haystack: &'i str,
}

impl<'i> BasicExtractor<'i> {
    pub fn new(haystack: &'i str) -> Self {
        Self { haystack }
    }

    pub fn _extract(&self) -> impl Iterator<Item = &'i str> + 'i {
        self.haystack
            .split(['\n', '\r', '\t', ' ', '"', '\'', ';', '{', '}', '`'])
            .filter(|s| s.starts_with(char::is_lowercase) || s.starts_with('-'))
            .collect::<FxHashSet<_>>()
            .into_iter()
    }
}

impl<'i> Extractor<'i> for BasicExtractor<'i> {
    fn extract(&self) -> Box<dyn Iterator<Item = &'i str> + 'i> {
        Box::new(self._extract())
    }
}
