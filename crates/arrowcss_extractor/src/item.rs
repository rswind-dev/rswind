/// An Item extract from the css file
///
/// Contains the utility and @apply info (the origin selector)
#[derive(Debug)]
pub struct ExtractItem<'a> {
    pub candidate: &'a str,
    pub selector: Option<&'a str>,
}

impl<'a> From<&'a str> for ExtractItem<'a> {
    fn from(value: &'a str) -> Self {
        ExtractItem {
            candidate: value,
            selector: None,
        }
    }
}

impl<'a> AsRef<str> for ExtractItem<'a> {
    fn as_ref(&self) -> &str {
        self.candidate
    }
}

impl<'a> ExtractItem<'a> {
    pub fn new(candidate: &'a str, selector: &'a str) -> Self {
        Self {
            candidate,
            selector: Some(selector),
        }
    }

    pub fn as_str(&self) -> &str {
        self.candidate
    }

    pub fn selector(&self) -> Option<&str> {
        self.selector
    }
}
