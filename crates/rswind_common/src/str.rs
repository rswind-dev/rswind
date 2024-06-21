use std::ops::Deref;

pub struct SmolStr(smol_str::SmolStr);

impl SmolStr {
    pub fn new(s: &str) -> Self {
        Self(smol_str::SmolStr::new(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for SmolStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<SmolStr> for String {
    fn from(s: SmolStr) -> Self {
        s.0.to_string()
    }
}

impl From<&str> for SmolStr {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for SmolStr {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}

impl Deref for SmolStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
