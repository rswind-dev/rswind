use smallvec::{smallvec, SmallVec};

use super::ParserPosition;
use crate::{
    common::MaybeArbitrary,
    context::Context,
    css::rule::RuleList,
    process::{ComposableHandler, Variant, VariantHandlerExt},
};

#[derive(Debug, Clone)]
pub struct VariantCandidate<'a> {
    pub key: &'a str,
    pub value: Option<MaybeArbitrary<'a>>,
    pub modifier: Option<MaybeArbitrary<'a>>,
    // fully arbitrary, e.g. [@media(min-width:300px)] [&:nth-child(3)]
    pub arbitrary: bool,
    pub processor: Variant,
    pub layers: SmallVec<[ComposableHandler; 1]>,
    pub ordering_key: Option<&'a str>,
}

impl<'a> VariantCandidate<'a> {
    pub fn handle(&self, rule: RuleList) -> RuleList {
        let rule = self.processor.handle(self.clone(), rule);
        self.layers
            .iter()
            .rev()
            .fold(rule, |rule, handler| handler.handle(self.clone(), rule))
    }
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
///   '[' <any> ']'
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
            Some(MaybeArbitrary::Named(
                self.current().trim_start_matches('-'),
            ))
        };
    }

    pub fn parse(&mut self, ctx: &Context) -> Option<VariantCandidate<'a>> {
        // find key
        if let Some(processor) = ctx.variants.get(self.current()) {
            self.key = Some(self.current());
            return Some(VariantCandidate {
                key: self.key?,
                value: None,
                modifier: None,
                arbitrary: false,
                ordering_key: None,
                layers: SmallVec::new(),
                processor: processor.clone(),
            });
        }
        let mut processor: Option<Variant> = None;
        let mut composes = smallvec![];

        if self.current().starts_with('@') {
            self.key = Some("@");
            self.pos.advance(1);
        } else {
            if self.current().starts_with('[') && self.current().ends_with(']') {
                // let arbitrary = self.current().get(1..self.current().len() - 1)?;
                todo!("parse arbitrary")
            }
            // let mut iter = self.current().match_indices('-');
            let mut iter = memchr::memchr_iter(b'-', self.current().as_bytes());
            let next = iter.next()?;
            let key = self.current().get(0..next)?;
            self.key = Some(key);
            if let Some(v) = ctx.variants.get(key) {
                processor = Some(v.clone());
                if v.composable {
                    composes.push(v.take_composable().unwrap().clone());
                    let key_str = self.current();
                    self.pos.advance(key.len());

                    let mut prev_i = next;
                    for i in iter {
                        if let Some((next_key, Some(compose_handler))) =
                            key_str.get(prev_i + 1..i).and_then(|next_key| {
                                ctx.variants
                                    .get(next_key)
                                    .map(|v| (next_key, v.take_composable()))
                            })
                        {
                            composes.push(compose_handler.clone());
                            self.pos.advance(1 + next_key.len());
                        }
                        prev_i = i;
                    }
                }
            }
        }

        self.key?;

        // find value and modifier
        self.parse_value_and_modifier();
        if !composes.is_empty() {
            let variant = ctx.variants.get(self.value?.take_named()?).unwrap();
            // let composer =
            //     Composer::new_with_layers(composes.into(), variant.clone());
            return Some(VariantCandidate {
                key: self.key?,
                value: self.value,
                modifier: self.modifier,
                arbitrary: false,
                ordering_key: None,
                layers: composes,
                processor: variant.clone(),
            });
        }

        let candidate = VariantCandidate {
            key: self.key?,
            value: self.value,
            arbitrary: false,
            modifier: self.modifier,
            ordering_key: None,
            layers: smallvec![],
            processor: processor?,
        };

        Some(candidate)
    }
}

#[cfg(test)]
mod tests {
    use smallvec::smallvec;
    use smol_str::format_smolstr;

    use crate::{
        context::Context,
        css::{Decl, Rule},
        parsing::VariantParser,
    };

    #[test]
    fn test_parse_variant() {
        let mut ctx = Context::default();
        ctx.add_variant("hover", ["&:hover"]);
        ctx.add_variant_composable("has", |r, _| {
            r.modify_with(|s| format_smolstr!("&:has({})", s.replace('&', "*")))
        });
        ctx.add_variant_composable("not", |r, _| {
            r.modify_with(|s| format_smolstr!("&:not({})", s.replace('&', "*")))
        });

        let mut input = VariantParser::new("has-not-hover");
        let c = input.parse(&ctx).unwrap();

        let rule =
            Rule::new_with_decls("&", smallvec![Decl::new("display", "flex")]).to_rule_list();

        let res = c.handle(rule);

        assert_eq!(res.as_single().unwrap().selector, "&:has(*:not(*:hover))")
    }
}
