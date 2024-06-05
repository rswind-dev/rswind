pub mod utility;
pub mod variant;

use std::fmt::Write;

use serde::Deserialize;
use smol_str::SmolStr;
use thiserror::Error;

pub use self::{utility::*, variant::*};
use crate::{
    common::MaybeArbitrary,
    parsing::UtilityCandidate,
    theme::{Theme, ThemeValue},
    types::TypeValidator,
};

static DEFAULT: &str = "DEFAULT";

/// A trait for preprocessing values.
///
/// This trait is used to preprocessing values before they pass to [`UtilityHandler`].
///
/// Simultaneously, the value validation is also done here.
///
/// e.g. `m-1`'s value is `1`, by getting value though `allowed_values`, we can get the value `0.25rem`.
///      `m-[2px]`'s value is an arbitrary value of `2px`, which will first be validate though `validate` method.
///
/// To some utilities allow value is `None`, we will use the `DEFAULT` value.
pub trait ValuePreprocessor {
    fn validate(&self, value: &str) -> bool;
    fn allowed_values(&self) -> Option<&ThemeValue>;

    fn preprocess(&self, value: Option<MaybeArbitrary<'_>>) -> Option<SmolStr> {
        match value {
            Some(MaybeArbitrary::Arbitrary(value)) => {
                let value = decode_arbitrary_value(value);
                self.validate(&value).then_some(value)
            }
            Some(MaybeArbitrary::Named(value)) => self.allowed_values()?.get(value),
            None => self.allowed_values()?.get(DEFAULT),
        }
    }
}

pub fn decode_arbitrary_value(input: &str) -> SmolStr {
    let mut writer = smol_str::Writer::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next_char) = chars.peek() {
                if *next_char == '_' {
                    chars.next();
                    let _ = writer.write_str("_");
                    continue;
                }
            }
        }
        let _ = writer.write_char(if c == '_' { ' ' } else { c });
    }

    SmolStr::from(writer)
}

// TODO: json schema docs below
/// An unparsed value representation.
///
/// This struct is used to store the raw value and modifier of a utility
/// and will be parse into [`ValueRepr`].
///
/// Used at: preset definitions, config deserializaion.
#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
pub struct RawValueRepr {
    #[serde(rename = "type")]
    pub validator: Option<Box<dyn TypeValidator>>,
    #[serde(rename = "theme")]
    pub theme_key: Option<SmolStr>,
}

#[derive(Debug, Error)]
pub enum ThemeParseError {
    #[error("Theme key `{0}` does not exist")]
    InvalidThemeKey(SmolStr),
}

impl RawValueRepr {
    pub fn new(theme_key: impl Into<SmolStr>) -> Self {
        Self { validator: None, theme_key: Some(theme_key.into()) }
    }

    pub fn with_validator(mut self, validator: impl TypeValidator + 'static) -> Self {
        self.validator = Some(Box::new(validator));
        self
    }

    pub fn parse(self, theme: &Theme) -> Result<ValueRepr, ThemeParseError> {
        if let Some(key) = self.theme_key {
            return Ok(ValueRepr {
                validator: self.validator,
                allowed_values: Some(
                    theme.get(&key).ok_or(ThemeParseError::InvalidThemeKey(key))?.clone(),
                ),
            });
        }

        Ok(ValueRepr { validator: self.validator, allowed_values: None })
    }
}

/// A parsed value representation.
///
/// This struct is used to store the value representation, include the validator and allowed values.
///
/// See also: [`ValuePreprocessor`].
#[derive(Debug, Default)]
pub struct ValueRepr {
    pub validator: Option<Box<dyn TypeValidator>>,
    pub allowed_values: Option<ThemeValue>,
}

impl ValueRepr {
    pub fn new(allowed_values: ThemeValue) -> Self {
        Self { validator: None, allowed_values: Some(allowed_values) }
    }

    pub fn with_validator(self, validator: impl TypeValidator + 'static) -> Self {
        Self { validator: Some(Box::new(validator)), allowed_values: self.allowed_values }
    }
}

impl ValuePreprocessor for ValueRepr {
    fn validate(&self, value: &str) -> bool {
        self.validator.as_ref().map_or(true, |validator| validator.validate(value))
    }

    fn allowed_values(&self) -> Option<&ThemeValue> {
        self.allowed_values.as_ref()
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
        Self { raw_value: candidate.value, raw_modifier: candidate.modifier, modifier: None }
    }

    /// Create a new `MetaData` with only the modifier set.
    pub(crate) fn modifier(modifier: impl Into<SmolStr>) -> Self {
        Self { raw_value: None, raw_modifier: None, modifier: Some(modifier.into()) }
    }
}
