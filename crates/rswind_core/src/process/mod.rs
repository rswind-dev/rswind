pub mod utility;
pub mod variant;

use std::{fmt::Write, sync::Arc};

use rswind_theme::{Theme, ThemeMap, ThemeValue};
use serde::Deserialize;
use smol_str::SmolStr;
use thiserror::Error;

pub use self::{utility::*, variant::*};
use crate::{
    common::MaybeArbitrary,
    parse::{ThemeKey, UtilityCandidate},
    types::{CssTypeValidator, TypeValidator},
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
    fn allowed_values(&self) -> Option<&ThemeMap>;

    fn preprocess(&self, value: Option<MaybeArbitrary>) -> Option<ThemeValue> {
        match value {
            Some(MaybeArbitrary::Arbitrary(value)) => {
                let value = decode_arbitrary_value(value);
                self.validate(&value).then_some(ThemeValue::Plain(value))
            }
            Some(MaybeArbitrary::Named(value)) => self.allowed_values()?.get_value(value),
            None => self.allowed_values()?.get_value(DEFAULT),
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
/// Used at: preset definitions, config deserialization.
#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "build", derive(instance_code::InstanceCode), instance(path = rswind_core::process))]
#[serde(deny_unknown_fields)]
pub struct RawValueDef {
    #[serde(rename = "type")]
    pub validator: Option<CssTypeValidator>,
    #[serde(rename = "theme")]
    pub theme_key: Option<ThemeKey>,
}

#[derive(Debug, Error)]
pub enum ThemeParseError {
    #[error("Theme key `{0}` does not exist")]
    InvalidThemeKey(SmolStr),
}

impl RawValueDef {
    pub fn new(theme_key: impl Into<ThemeKey>) -> Self {
        Self { validator: None, theme_key: Some(theme_key.into()) }
    }

    pub fn with_validator(mut self, validator: impl Into<CssTypeValidator>) -> Self {
        self.validator = Some(validator.into());
        self
    }

    pub fn parse(self, theme: &Theme) -> Result<ValueDef, ThemeParseError> {
        if let Some(key) = self.theme_key {
            return Ok(ValueDef {
                validator: self.validator,
                allowed_values: Some(key.parse(theme)?),
            });
        }

        Ok(ValueDef { validator: self.validator, allowed_values: None })
    }
}

/// A parsed value representation.
///
/// This struct is used to store the value representation, include the validator and allowed values.
///
/// See also: [`ValuePreprocessor`].
#[derive(Debug, Default)]
pub struct ValueDef {
    pub validator: Option<CssTypeValidator>,
    pub allowed_values: Option<Arc<ThemeMap>>,
}

impl ValueDef {
    pub fn new(allowed_values: ThemeMap) -> Self {
        Self { validator: None, allowed_values: Some(Arc::new(allowed_values)) }
    }

    pub fn with_validator(self, validator: CssTypeValidator) -> Self {
        Self { validator: Some(validator), allowed_values: self.allowed_values }
    }
}

impl ValuePreprocessor for ValueDef {
    fn validate(&self, value: &str) -> bool {
        self.validator.as_ref().map_or(true, |validator| validator.validate(value))
    }

    fn allowed_values(&self) -> Option<&ThemeMap> {
        self.allowed_values.as_deref()
    }
}

#[derive(Clone, Default, Debug)]
pub struct MetaData<'a> {
    pub raw_value: Option<MaybeArbitrary<'a>>,
    pub raw_modifier: Option<MaybeArbitrary<'a>>,
    pub modifier: Option<SmolStr>,
    pub theme_value: ThemeValue<'a>,
}

impl<'a> MetaData<'a> {
    pub fn from_candidate(candidate: &UtilityCandidate<'a>) -> Self {
        Self { raw_value: candidate.value, raw_modifier: candidate.modifier, ..Default::default() }
    }

    /// Create a new `MetaData` with only the modifier set.
    pub fn modifier(modifier: impl Into<SmolStr>) -> Self {
        Self { modifier: Some(modifier.into()), ..Default::default() }
    }

    pub(crate) fn theme_value(mut self, theme_value: ThemeValue<'a>) -> Self {
        self.theme_value = theme_value;
        self
    }
}
