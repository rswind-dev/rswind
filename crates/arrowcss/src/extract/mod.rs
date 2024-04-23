use fxhash::FxHashSet as HashSet;

pub struct Extractor<'i> {
    pub haystack: &'i str,
}

impl<'i> Extractor<'i> {
    pub fn new(haystack: &'i str) -> Self {
        Self { haystack }
    }

    pub fn extract(&self) -> HashSet<&'i str> {
        self.haystack
            .split(['\n', '\r', '\t', ' ', '"', '\'', ';', '{', '}', '`'])
            .filter(|s| s.starts_with(char::is_lowercase) || s.starts_with('-'))
            .collect::<HashSet<_>>()
    }
}
