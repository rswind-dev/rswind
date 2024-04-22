use crate::{
    common::MaybeArbitrary,
    context::{utilities::UtilityStorage, Context},
    css::rule::RuleList,
    ordering::OrderingKey,
    process::{ModifierProcessor, RuleMatchingFn, Utility, UtilityHandler},
    types::TypeValidator,
};

use super::ParserPosition;

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
    pub fn is_fraction_like(&self) -> bool {
        matches!(
            (self.value, self.modifier),
            (
                Some(MaybeArbitrary::Named(_)),
                Some(MaybeArbitrary::Named(_))
            )
        )
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

    fn current<'b>(&self) -> &'b str
    where
        'a: 'b,
    {
        self.input.get(self.pos.start..self.pos.end).unwrap()
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

    pub fn parse(&mut self, ctx: &Context) -> Option<UtilityCandidate<'a>> {
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

        // find key
        if ctx.utilities.get(self.current()).is_some() {
            self.key = Some(self.current());
            return Some(UtilityCandidate {
                key: self.key?,
                value: None,
                modifier: None,
                arbitrary: false,
                important: self.is_important,
                negative: self.is_negative,
            });
        } else {
            for (i, _) in self.current().match_indices('-').rev() {
                let key = self.current().get(0..i)?;
                if ctx.utilities.get(key).is_some() {
                    self.key = Some(key);
                    self.pos.start += i + 1;
                    break;
                }
            }
        }

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

pub struct UtilityBuilder<'i, 'c> {
    ctx: &'i mut Context<'c>,
    key: &'i str,
    theme_key: Option<&'i str>,
    handler: UtilityHandler,
    modifier: Option<ModifierProcessor<'c>>,
    validator: Option<Box<dyn TypeValidator>>,
    additional_css: Option<RuleList<'c>>,
    wrapper: Option<String>,
    supports_negative: bool,
    supports_fraction: bool,
    ordering_key: Option<OrderingKey>,
}

impl<'i, 'c> UtilityBuilder<'i, 'c> {
    pub fn new(
        ctx: &'i mut Context<'c>,
        key: &'i str,
        handler: impl RuleMatchingFn + 'static,
    ) -> Self {
        Self {
            ctx,
            key,
            handler: UtilityHandler::Dynamic(Box::new(handler)),
            theme_key: None,
            supports_negative: false,
            supports_fraction: false,
            modifier: None,
            validator: None,
            additional_css: None,
            wrapper: None,
            ordering_key: None,
        }
    }

    pub fn with_theme(mut self, key: &'i str) -> Self {
        self.theme_key = Some(key);
        self
    }

    pub fn support_negative(mut self) -> Self {
        self.supports_negative = true;
        self
    }

    #[allow(dead_code)]
    pub fn support_fraction(mut self) -> Self {
        self.supports_fraction = true;
        self
    }

    pub fn with_modifier(mut self, modifier: ModifierProcessor<'c>) -> Self {
        self.modifier = Some(modifier);
        self
    }

    pub fn with_validator(
        mut self,
        validator: impl TypeValidator + 'static,
    ) -> Self {
        self.validator = Some(Box::new(validator));
        self
    }

    pub fn with_additional_css(mut self, css: RuleList<'c>) -> Self {
        self.additional_css = Some(css);
        self
    }

    pub fn with_wrapper(mut self, wrapper: &str) -> Self {
        self.wrapper = Some(wrapper.to_string());
        self
    }

    pub fn with_ordering(mut self, key: OrderingKey) -> Self {
        self.ordering_key = Some(key);
        self
    }
}

/// Automatically adds the rule to the context when dropped.
/// This is useful for defining rules in a more declarative way.
impl<'i, 'c> Drop for UtilityBuilder<'i, 'c> {
    fn drop(&mut self) {
        let allowed_values = self.theme_key.map(|key| {
            self.ctx
                .get_theme(key)
                .unwrap_or_else(|| panic!("theme key `{key}` not found"))
                .clone()
        });
        let validator = std::mem::take(&mut self.validator);
        let handler = std::mem::take(&mut self.handler);
        let modifier = std::mem::take(&mut self.modifier);

        self.ctx.add_utility(
            self.key,
            Utility {
                validator,
                allowed_values,
                handler,
                modifier,
                supports_negative: self.supports_negative,
                supports_fraction: self.supports_fraction,
                additional_css: std::mem::take(&mut self.additional_css),
                wrapper: std::mem::take(&mut self.wrapper),
                ordering_key: std::mem::take(&mut self.ordering_key),
            },
        );
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test() {
    //     assert_eq!(
    //         parse_candidate("text-[1rem]/[2rem]").unwrap(),
    //         Utility {
    //             key: "text",
    //             value: MaybeArbitrary::Arbitrary("1rem"),
    //             arbitrary: false,
    //             important: false,
    //             negative: false,
    //             modifier: Some(MaybeArbitrary::Arbitrary("2rem"))
    //         }
    //     );
    // }

    // #[test]
    // fn test_named_modifier() {
    //     assert_eq!(
    //         parse_candidate("text-[1rem]/2").unwrap(),
    //         Utility {
    //             key: "text",
    //             value: MaybeArbitrary::Arbitrary("1rem"),
    //             arbitrary: false,
    //             important: false,
    //             negative: false,
    //             modifier: Some(MaybeArbitrary::Named("2"))
    //         }
    //     );
    // }

    // #[test]
    // fn test_no_modifier() {
    //     assert_eq!(
    //         parse_candidate("text-[1/2]").unwrap(),
    //         Utility {
    //             key: "text",
    //             value: MaybeArbitrary::Arbitrary("1/2"),
    //             arbitrary: false,
    //             modifier: None,
    //             important: false,
    //             negative: false
    //         }
    //     );
    // }

    // #[test]
    // fn test_arbitrary() {
    //     assert_eq!(
    //         parse_candidate("text-[1rem]").unwrap(),
    //         Utility {
    //             key: "text",
    //             value: MaybeArbitrary::Arbitrary("1rem"),
    //             arbitrary: false,
    //             modifier: None,
    //             important: false,
    //             negative: false
    //         }
    //     );
    // }

    // #[test]
    // fn test_no_arbitrary() {
    //     assert_eq!(
    //         parse_candidate("text-lg").unwrap(),
    //         Utility {
    //             key: "text",
    //             value: MaybeArbitrary::Named("lg"),
    //             arbitrary: false,
    //             modifier: None,
    //             important: false,
    //             negative: false
    //         }
    //     );
    // }

    // #[test]
    // fn test_negative() {
    //     assert_eq!(
    //         parse_candidate("-text-lg").unwrap(),
    //         Utility {
    //             key: "text",
    //             value: MaybeArbitrary::Named("lg"),
    //             arbitrary: false,
    //             modifier: None,
    //             important: false,
    //             negative: true
    //         }
    //     );
    // }

    // #[test]
    // fn test_no_arbitrary_modifier() {
    //     assert_eq!(
    //         parse_candidate("text-lg/2").unwrap(),
    //         Utility {
    //             key: "text",
    //             value: MaybeArbitrary::Named("lg"),
    //             arbitrary: false,
    //             modifier: Some(MaybeArbitrary::Named("2")),
    //             important: false,
    //             negative: false
    //         }
    //     );
    // }

    // #[test]
    // fn test_no_arbitrary_arbitrary_modifier() {
    //     assert_eq!(
    //         parse_candidate("text-lg/[2px]").unwrap(),
    //         Utility {
    //             key: "text",
    //             value: MaybeArbitrary::Named("lg"),
    //             arbitrary: false,
    //             modifier: Some(MaybeArbitrary::Arbitrary("2px")),
    //             important: false,
    //             negative: false
    //         }
    //     );
    // }

    // #[test]
    // fn test_fraction() {
    //     assert_eq!(
    //         parse_candidate("text-1/2").unwrap(),
    //         Utility {
    //             key: "text",
    //             value: MaybeArbitrary::Named("1"),
    //             arbitrary: false,
    //             modifier: Some(MaybeArbitrary::Named("2")),
    //             important: false,
    //             negative: false
    //         }
    //     );
    // }

    // fn test_fully_arbitrary() {
    //     assert_eq!(
    //         parse_candidate("[color:red]").unwrap(),
    //         Utility {
    //             key: "color",
    //             value: MaybeArbitrary::Named("red"),
    //             arbitrary: true,
    //             modifier: None,
    //             important: false,
    //             negative: false
    //         }
    //     );
    // }
}
