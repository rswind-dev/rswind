use either::Either;

use crate::{common::MaybeArbitrary, context::Context, process::Variant};

use super::ParserPosition;

#[derive(Debug)]
pub struct VariantCandidate<'a> {
    pub key: &'a str,
    pub value: Option<MaybeArbitrary<'a>>,
    pub modifier: Option<MaybeArbitrary<'a>>,
    // fully arbitrary, e.g. [@media(min-width:300px)] [&:nth-child(3)]
    pub arbitrary: bool,
    pub compose: Either<bool, Box<VariantCandidate<'a>>>,
    pub processor: Variant,
}

/// Parser
/// formal syntax:
/// https://drafts.csswg.org/css-values/#value-defs
/// https://developer.mozilla.org/en-US/docs/Web/CSS/Value_definition_syntax
///
/// utility =
///   [ <utility> / <modifier>? ]
///
/// utility =
///   [ <ident> - <value>? ]
///
/// value =
///   <ident> | <arbitrary>
///
/// arbitrary =
///   '['<any>']'
///
/// modifier = <value>
#[derive(Debug)]
pub struct VariantParser<'a> {
    input: &'a str,
    key: Option<&'a str>,
    value: Option<MaybeArbitrary<'a>>,
    modifier: Option<MaybeArbitrary<'a>>,
    pos: ParserPosition,
    // The current arbitrary value, could either be a `modifier` or a `value`
    arbitrary_start: usize,
    cur_arbitrary: Option<&'a str>,
}

impl<'a> VariantParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            pos: ParserPosition {
                start: 0,
                end: input.len(),
            },
            input,
            key: None,
            value: None,
            arbitrary_start: usize::MAX,
            modifier: None,
            cur_arbitrary: None,
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

    fn parse_value_and_modifier(&mut self) {
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

        self.value = if let Some(arbitrary) = self.cur_arbitrary {
            Some(MaybeArbitrary::Arbitrary(arbitrary))
        } else {
            Some(MaybeArbitrary::Named(self.current()))
        };
    }

    pub fn parse(&mut self, ctx: &Context) -> Option<VariantCandidate<'a>> {
        if self.current().starts_with('[') && self.current().ends_with(']') {
            // let arbitrary = self.current().get(1..self.current().len() - 1)?;
            todo!("parse arbitrary")
        }

        let mut processor = None;

        // find key
        if let Some(processor) = ctx.variants.get(self.current()) {
            self.key = Some(self.current());
            return Some(VariantCandidate {
                key: self.key?,
                value: None,
                modifier: None,
                arbitrary: false,
                compose: Either::Left(true),
                processor: processor.clone(),
            });
        } else if self.current().starts_with('@') {
            self.key = Some("@");
            self.pos.advance(1);
        } else {
            for (i, _) in self.current().match_indices('-') {
                let key = self.current().get(0..i)?;
                if let Some(p) = ctx.variants.get(key) {
                    processor = Some(p.clone());
                    self.key = Some(key);
                    self.pos.advance(i + 1);
                    break;
                }
            }
        }

        self.key?;

        // find value and modifier\
        self.parse_value_and_modifier();

        let candidate = VariantCandidate {
            key: self.key?,
            value: self.value,
            arbitrary: false,
            modifier: self.modifier,
            compose: Either::Left(false),
            processor: processor?,
        };

        Some(candidate)
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_parse_variant() {
    //     let mut input = VariantParser::new("group-[&:hover]/[sidebar]");
    //     let expected = Variant {
    //         key: "group",
    //         value: Some(MaybeArbitrary::Arbitrary("&:hover")),
    //         modifier: Some(MaybeArbitrary::Arbitrary("sidebar")),
    //         arbitrary: false,
    //     };
    //     assert_eq!(input.parse(), Some(expected));
    // }

    // #[test]
    // fn test_arbitrary() {
    //     let mut input = VariantParser::new("group-[&:hover]/sidebar");
    //     let expected = Variant {
    //         key: "group",
    //         value: Some(MaybeArbitrary::Arbitrary("&:hover")),
    //         modifier: Some(MaybeArbitrary::Named("sidebar")),
    //         arbitrary: false,
    //     };
    //     assert_eq!(input.parse(), Some(expected));
    // }

    // #[test]
    // fn test_named_modifier() {
    //     let mut input = VariantParser::new("group-hover/sidebar");
    //     let expected = Variant {
    //         key: "group",
    //         value: Some(MaybeArbitrary::Named("hover")),
    //         modifier: Some(MaybeArbitrary::Named("sidebar")),
    //         arbitrary: false,
    //     };
    //     assert_eq!(input.parse(), Some(expected));
    // }

    // #[test]
    // fn test_named_arbitrary_modifier() {
    //     let mut input = VariantParser::new("group-hover/[sidebar]");
    //     let expected = Variant {
    //         key: "group",
    //         value: Some(MaybeArbitrary::Named("hover")),
    //         modifier: Some(MaybeArbitrary::Arbitrary("sidebar")),
    //         arbitrary: false,
    //     };
    //     assert_eq!(input.parse(), Some(expected));
    // }

    // #[test]
    // fn test_simple_variant() {
    //     let mut input = VariantParser::new("group-hover");
    //     let expected = Variant {
    //         key: "group",
    //         value: Some(MaybeArbitrary::Named("hover")),
    //         modifier: None,
    //         arbitrary: false,
    //     };
    //     assert_eq!(input.parse(), Some(expected));
    // }

    // #[test]
    // fn test_at_variant() {
    //     let mut input = VariantParser::new("@md");
    //     let expected = Variant {
    //         key: "@",
    //         value: Some(MaybeArbitrary::Named("md")),
    //         modifier: None,
    //         arbitrary: false,
    //     };
    //     assert_eq!(input.parse(), Some(expected));
    // }

    // #[test]
    // fn test_at_arbitrary() {
    //     let mut input = VariantParser::new("@[17.5rem]");
    //     let expected = Variant {
    //         key: "@",
    //         value: Some(MaybeArbitrary::Arbitrary("17.5rem")),
    //         modifier: None,
    //         arbitrary: false,
    //     };
    //     assert_eq!(input.parse(), Some(expected));
    // }

    // #[test]
    // fn test_at_arbitrary_with_modifier() {
    //     let mut input = VariantParser::new("@lg/main");
    //     let expected = Variant {
    //         key: "@",
    //         value: Some(MaybeArbitrary::Named("lg")),
    //         modifier: Some(MaybeArbitrary::Named("main")),
    //         arbitrary: false,
    //     };
    //     assert_eq!(input.parse(), Some(expected));
    // }
}
