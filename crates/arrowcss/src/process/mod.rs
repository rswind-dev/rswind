pub mod utility;
pub mod variant;

use crate::common::MaybeArbitrary;
use crate::parsing::UtilityCandidate;
use crate::theme::ThemeValue;
use lightningcss::traits::IntoOwned;
use lightningcss::values::string::CowArcStr;

pub use self::utility::*;
pub use self::variant::*;

pub trait ArbitraryValueProcessor<'a> {
    fn validate(&self, value: &str) -> bool;
    fn allowed_values(&self) -> Option<&ThemeValue<'a>>;

    fn process(&self, value: MaybeArbitrary<'_>) -> Option<CowArcStr<'static>> {
        match value {
            MaybeArbitrary::Arbitrary(value) => self
                .validate(value)
                .then(|| CowArcStr::from(value).into_owned()),
            MaybeArbitrary::Named(value) => self
                .allowed_values()?
                .get(value)
                .map(|v| v.clone().into_owned()),
        }
    }
}

#[derive(Clone, Default)]
pub struct MetaData<'a> {
    pub candidate: UtilityCandidate<'a>,
    pub modifier: Option<String>,
}

impl<'a> MetaData<'a> {
    pub(crate) fn new(candidate: UtilityCandidate<'a>) -> Self {
        Self {
            candidate,
            modifier: None,
        }
    }
}
