use serde::Deserialize;
use smol_str::{format_smolstr, SmolStr};

use super::ParserPosition;
use crate::{
    common::MaybeArbitrary,
    context::utilities::{UtilityStorage, UtilityStorageImpl},
    css::rule::RuleList,
    ordering::OrderingKey,
    process::{RawValueRepr, RuleMatchingFn, Utility, UtilityGroup, UtilityHandler},
    theme::Theme,
    types::TypeValidator,
};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct UtilityCandidate<'a> {
    pub key: &'a str,
    pub value: Option<MaybeArbitrary<'a>>,
    pub modifier: Option<MaybeArbitrary<'a>>,
    // fully arbitrary, e.g. [color:red] [text:--my-font-size]
    pub arbitrary: bool,
    pub important: bool,
    pub negative: bool,
}

impl UtilityCandidate<'_> {
    // only if value and modifier are both named
    pub fn take_fraction(&self) -> Option<SmolStr> {
        match (self.value, self.modifier) {
            (Some(MaybeArbitrary::Named(v)), Some(MaybeArbitrary::Named(m))) => {
                Some(format_smolstr!("{v}/{m}",))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct UtilityParser<'a> {
    input: &'a str,
    key: Option<&'a str>,
    value: Option<MaybeArbitrary<'a>>,
    modifier: Option<MaybeArbitrary<'a>>,
    pos: ParserPosition,
    // The current arbitrary value, could either be a `modifier` or a `value`
    arbitrary_start: usize,
    cur_arbitrary: Option<&'a str>,
    is_negative: bool,
    is_important: bool,
}

impl<'a> UtilityParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            pos: ParserPosition {
                start: 0,
                end: input.len(),
            },
            input,
            key: None,
            value: None,
            is_important: false,
            arbitrary_start: usize::MAX,
            modifier: None,
            cur_arbitrary: None,
            is_negative: false,
        }
    }

    fn current(&self) -> &'a str {
        &self.input[self.pos.start..self.pos.end]
    }

    fn inside_arbitrary(&self) -> bool {
        self.arbitrary_start != usize::MAX
    }

    fn arbitrary_start_at(&mut self, i: usize) {
        self.arbitrary_start = i;
    }

    fn consume_modifier(&mut self, pos: usize) {
        if let Some(arbitrary) = self.cur_arbitrary {
            self.modifier = Some(MaybeArbitrary::Arbitrary(arbitrary));
            self.cur_arbitrary = None;
        } else {
            self.modifier = Some(MaybeArbitrary::Named(
                self.current().get(pos + 1..).unwrap(),
            ));
        }
        self.pos.end = self.pos.start + pos;
    }

    fn consume_arbitrary(&mut self, pos: usize) {
        self.cur_arbitrary = self.current().get(pos..self.arbitrary_start);
        self.arbitrary_start = usize::MAX;
    }

    fn parse_important(&mut self) {
        if self.current().ends_with('!') {
            self.pos.end -= 1;
            self.is_important = true;
        }
    }

    fn parse_negative(&mut self) {
        if self.current().starts_with('-') {
            self.pos.start += 1;
            self.is_negative = true;
        }
    }

    pub fn parse(&mut self, utilities: &UtilityStorageImpl) -> Option<UtilityCandidate<'a>> {
        // find key
        if utilities.get(self.current()).is_some() {
            self.key = Some(self.current());
            return Some(UtilityCandidate {
                key: self.key?,
                value: None,
                modifier: None,
                arbitrary: false,
                important: self.is_important,
                negative: self.is_negative,
            });
        }

        self.parse_important();
        self.parse_negative();

        if self.current().starts_with('[') && self.current().ends_with(']') {
            let arbitrary = self.current().get(1..self.current().len() - 1)?;
            let (key, value) = arbitrary.split_once(':')?;
            return Some(UtilityCandidate {
                key,
                value: Some(MaybeArbitrary::Named(value)),
                arbitrary: true,
                important: self.is_important,
                negative: self.is_negative,
                modifier: None,
            });
        }

        // for (i, _) in self.current().match_indices('-').rev() {
        for i in memchr::memchr_iter(b'-', self.current().as_bytes()).rev() {
            let (key, _value) = self.current().split_at(i);
            if let Some(_utility) = utilities.get(key) {
                self.key = Some(key);
                self.pos.start += i + 1;
                break;
            }
        }

        // if no key is found, return None
        self.key?;

        // find value and modifier\
        let len = self.current().len();
        for (i, c) in self.current().chars().rev().enumerate() {
            let i = len - i - 1;
            match c {
                '/' if !self.inside_arbitrary() => self.consume_modifier(i),
                ']' => self.arbitrary_start_at(i),
                '[' => self.consume_arbitrary(i + 1),
                _ => (),
            }
        }

        if let Some(arbitrary) = self.cur_arbitrary {
            self.value = Some(MaybeArbitrary::Arbitrary(arbitrary));
        } else {
            self.value = Some(MaybeArbitrary::Named(self.current()));
        }

        let candidate = UtilityCandidate {
            key: self.key?,
            value: self.value,
            arbitrary: false,
            important: self.is_important,
            negative: self.is_negative,
            modifier: self.modifier,
        };

        Some(candidate)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UtilityBuilder {
    pub key: SmolStr,

    #[serde(rename = "css")]
    pub handler: Option<UtilityHandler>,

    #[serde(default)]
    pub modifier: Option<RawValueRepr>,

    #[serde(rename = "theme")]
    pub theme_key: Option<SmolStr>,

    #[serde(rename = "type")]
    pub validator: Option<Box<dyn TypeValidator>>,

    #[serde(default)]
    pub wrapper: Option<SmolStr>,

    #[serde(default)]
    pub supports_negative: bool,
    #[serde(default)]
    pub supports_fraction: bool,
    // TODO: add support for below fields
    #[serde(skip_deserializing)]
    pub additional_css: Option<RuleList>,
    #[serde(skip_deserializing)]
    pub ordering_key: Option<OrderingKey>,
    #[serde(skip_deserializing)]
    pub group: Option<UtilityGroup>,
}

impl UtilityBuilder {
    pub fn new(key: impl Into<SmolStr>, handler: impl RuleMatchingFn + 'static) -> Self {
        Self {
            key: key.into(),
            handler: Some(UtilityHandler::new(handler)),
            theme_key: None,
            supports_negative: false,
            supports_fraction: false,
            modifier: None,
            validator: None,
            additional_css: None,
            wrapper: None,
            ordering_key: None,
            group: None,
        }
    }

    // TODO: return Result<Self, Error> remove unwrap
    pub fn parse(self, theme: &Theme) -> Utility {
        Utility {
            handler: self.handler.unwrap(),
            supports_negative: self.supports_negative,
            supports_fraction: self.supports_fraction,
            value_repr: RawValueRepr {
                theme_key: self.theme_key,
                validator: self.validator,
            }
            .parse(theme)
            .unwrap(),
            modifier: self.modifier.map(|m| m.parse(theme)).transpose().unwrap(),
            wrapper: self.wrapper,
            additional_css: self.additional_css,
            ordering_key: self.ordering_key,
            group: self.group,
        }
    }

    pub fn with_theme(&mut self, key: impl Into<SmolStr>) -> &mut Self {
        self.theme_key = Some(key.into());
        self
    }

    pub fn support_negative(&mut self) -> &mut Self {
        self.supports_negative = true;
        self
    }

    pub fn support_fraction(&mut self) -> &mut Self {
        self.supports_fraction = true;
        self
    }

    pub fn with_modifier(&mut self, modifier: RawValueRepr) -> &mut Self {
        self.modifier = Some(modifier);
        self
    }

    pub fn with_validator(&mut self, validator: impl TypeValidator + 'static) -> &mut Self {
        self.validator = Some(Box::new(validator));
        self
    }

    pub fn with_additional_css(&mut self, css: RuleList) -> &mut Self {
        self.additional_css = Some(css);
        self
    }

    pub fn with_wrapper(&mut self, wrapper: &str) -> &mut Self {
        self.wrapper = Some(wrapper.into());
        self
    }

    pub fn with_ordering(&mut self, key: OrderingKey) -> &mut Self {
        self.ordering_key = Some(key);
        self
    }

    pub fn with_group(&mut self, group: UtilityGroup) -> &mut Self {
        self.group = Some(group);
        self
    }
}
