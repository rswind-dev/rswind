pub mod utility;
pub mod variant;

use smol_str::SmolStr;

pub use self::{utility::*, variant::*};
use crate::{common::MaybeArbitrary, parsing::UtilityCandidate, theme::ThemeValue};

static DEFAULT: &str = "DEFAULT";

pub trait ArbitraryValueProcessor {
    fn validate(&self, value: &str) -> bool;
    fn allowed_values(&self) -> Option<&ThemeValue>;

    fn process(&self, value: Option<MaybeArbitrary<'_>>) -> Option<SmolStr> {
        match value {
            Some(MaybeArbitrary::Arbitrary(value)) => {
                self.validate(value).then(|| SmolStr::from(value))
            }
            Some(MaybeArbitrary::Named(value)) => self.allowed_values()?.get(value),
            None => self.allowed_values()?.get(DEFAULT),
        }
    }
}

#[derive(Clone, Default)]
pub struct MetaData<'a> {
    pub raw_value: Option<MaybeArbitrary<'a>>,
    pub raw_modifier: Option<MaybeArbitrary<'a>>,
    pub modifier: Option<SmolStr>,
}

impl<'a> MetaData<'a> {
    pub fn from_candidate(candidate: &UtilityCandidate<'a>) -> Self {
        Self {
            raw_value: candidate.value,
            raw_modifier: candidate.modifier,
            modifier: None,
        }
    }

    /// Create a new `MetaData` with only the modifier set.
    pub(crate) fn modifier(modifier: impl Into<SmolStr>) -> Self {
        Self {
            raw_value: None,
            raw_modifier: None,
            modifier: Some(modifier.into()),
        }
    }
}
