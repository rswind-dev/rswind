use smol_str::SmolStr;

pub struct CompStr(SmolStr);

impl CompStr {
    pub fn new(s: &str) -> Self {
        Self(SmolStr::new(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for CompStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<CompStr> for String {
    fn from(s: CompStr) -> Self {
        s.0.to_string()
    }
}

impl From<&str> for CompStr {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for CompStr {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}
