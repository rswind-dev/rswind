use rswind_css::rule::RuleList;
use smol_str::SmolStr;

use crate::values::{FontFamily, FontSize};

#[derive(Clone, Debug)]
pub enum ThemeValue<'a> {
    Plain(SmolStr),
    // Utility values
    FontSize(&'a FontSize),
    FontFamily(&'a FontFamily),
    KeyFrames(&'a RuleList),
}

impl Default for ThemeValue<'_> {
    fn default() -> Self {
        Self::Plain(SmolStr::default())
    }
}

impl<'a> ThemeValue<'a> {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Plain(v) => Some(v.as_str()),
            _ => None,
        }
    }
}