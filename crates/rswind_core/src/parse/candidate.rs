use std::ops::Index;

use derive_more::{Deref, DerefMut};
use either::Either::{self, Left, Right};
use rswind_extractor::cursor::Cursor;
use smallvec::{smallvec, SmallVec};
use tracing::{instrument, span, trace};

use super::{
    state::{State, StateTransformer, UtilityTransformer},
    UtilityCandidate, VariantCandidate,
};
use crate::{
    common::MaybeArbitrary,
    design::{utilities::UtilityStorage, variants::VariantStorage},
    parse::state::VariantTransformer,
    process::{Variant, VariantKind, VariantOrdering},
};

#[derive(Deref, DerefMut)]
pub struct CandidateParser<'a> {
    input: &'a str,
    #[deref]
    #[deref_mut]
    cursor: Cursor<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.start..index.end]
    }
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn to(&self, other: &Span) -> Span {
        Span { start: self.start, end: other.end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    /// An ident
    Ident(Span),
    /// `[...]`
    Arbitrary(&'a str),
    /// `/`
    Slash,
    /// `!`
    Bang,
    /// `-`
    Minus,
    /// `@`
    At,
}

impl<'a> CandidateParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, cursor: Cursor::new(input) }
    }

    fn str_from(&self, start: usize) -> &'a str {
        &self.input[start..self.cursor.pos()]
    }

    fn span_from(&self, start: usize) -> Span {
        Span { start, end: self.cursor.pos() }
    }

    fn next_token(&mut self) -> Result<Option<Token<'a>>, ()> {
        let start = self.pos();

        let res = match self.first() {
            'a'..='z' | '0'..='9' => {
                self.eat_while(|c| matches!(c, 'a'..='z' | '0'..='9' | '%' | '.'));
                let res = Some(Token::Ident(self.span_from(start)));
                if self.first() == '-' {
                    self.bump();
                }
                res
            }
            '[' => {
                self.bump();
                let start = self.cursor.pos();
                self.eat_until_char(b']');
                let tok = self.str_from(start);
                Some(Token::Arbitrary(tok))
            }
            '-' => Some(Token::Minus),
            '!' => Some(Token::Bang),
            '/' => Some(Token::Slash),
            '@' => Some(Token::At),
            '\0' => None,
            _ => return Err(()),
        };

        if !matches!(res, Some(Token::Ident(_))) {
            self.bump();
        }

        Ok(res)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UtilityRepr<'a> {
    idents: SmallVec<[Span; 2]>,
    arbitrary: Option<&'a str>,
    modifier: Option<Either<Span, &'a str>>,
    important: bool,
    negative: bool,
}

impl<'a> CandidateParser<'a> {
    fn parse_utility_repr(&mut self) -> Option<UtilityRepr<'a>> {
        let mut repr = UtilityRepr::default();
        let mut state = State::Initial;

        while let Some(token) = self.next_token().ok()? {
            let new_state = UtilityTransformer::transform(&state, &token)?;
            match (token, state) {
                (Token::Ident(span), State::Initial | State::AfterIdent) => {
                    repr.idents.push(span);
                }
                (Token::Arbitrary(arb), State::Initial | State::AfterIdent) => {
                    repr.arbitrary = Some(arb);
                }
                (Token::Bang, State::Initial) if !repr.important => {
                    repr.important = true;
                }
                (Token::Bang, _) if !repr.important && new_state == State::Eof => {
                    repr.important = true;
                }
                (Token::Minus, State::Initial) if !repr.negative => {
                    repr.negative = true;
                }
                (Token::Ident(span), State::AfterSlash) if repr.modifier.is_none() => {
                    repr.modifier = Some(Either::Left(span));
                }
                (Token::Arbitrary(arb), State::AfterSlash) if repr.modifier.is_none() => {
                    repr.modifier = Some(Either::Right(arb));
                }
                (Token::Slash, State::AfterArbitrary | State::AfterIdent) => {}
                _ => return None,
            }
            state = new_state;
        }
        Some(repr)
    }

    #[instrument(fields(input = self.input), skip_all, level = "trace")]
    pub fn parse_utility(&mut self, ut: &UtilityStorage) -> Option<UtilityCandidate<'a>> {
        if ut.get(self.input).is_some() {
            return Some(UtilityCandidate {
                key: self.input,
                value: None,
                modifier: None,
                arbitrary: false,
                important: false,
                negative: false,
            });
        }

        let repr = self.parse_utility_repr()?;
        trace!(input = self.input, ?repr);

        if let Some(arb) = repr.arbitrary {
            if repr.idents.is_empty() {
                let (key, value) = arb.split_once(':')?;
                return Some(UtilityCandidate {
                    key,
                    value: Some(MaybeArbitrary::Arbitrary(value)),
                    modifier: None,
                    arbitrary: true,
                    important: repr.important,
                    negative: repr.negative,
                });
            }

            let key = repr.idents[0].to(repr.idents.last().unwrap());

            ut.get(&self.input[key])?;
            return Some(UtilityCandidate {
                key: &self.input[key],
                value: Some(MaybeArbitrary::Arbitrary(arb)),
                modifier: match repr.modifier {
                    Some(Either::Left(span)) => Some(MaybeArbitrary::Named(&self.input[span])),
                    Some(Either::Right(arb)) => Some(MaybeArbitrary::Arbitrary(arb)),
                    None => None,
                },
                arbitrary: false,
                important: repr.important,
                negative: repr.negative,
            });
        }

        let mut iter = repr.idents.iter().rev().peekable();
        let mut prev = iter.next();

        for ident in iter {
            let key = &self.input[repr.idents[0].to(ident)];
            if ut.get(key).is_some() {
                return Some(UtilityCandidate {
                    key,
                    value: Some(MaybeArbitrary::Named(
                        &self.input[prev.unwrap().to(repr.idents.last().unwrap())],
                    )),
                    modifier: match repr.modifier {
                        Some(Either::Left(span)) => Some(MaybeArbitrary::Named(&self.input[span])),
                        Some(Either::Right(arb)) => Some(MaybeArbitrary::Arbitrary(arb)),
                        None => None,
                    },
                    arbitrary: false,
                    important: repr.important,
                    negative: repr.negative,
                });
            }

            prev = Some(ident);
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VariantRepr<'a> {
    idents: SmallVec<[Span; 2]>,
    arbitrary: Option<&'a str>,
    modifier: Option<MaybeArbitrary<'a>>,
}

impl<'a> CandidateParser<'a> {
    fn parse_variant_repr(&mut self) -> Option<VariantRepr<'a>> {
        let mut repr = VariantRepr::default();
        let mut modifier: Option<Either<Span, &str>> = None;
        let mut state = State::Initial;

        while let Some(token) = self.next_token().ok()? {
            let new_state = VariantTransformer::transform(&state, &token)?;

            match (token, state) {
                (Token::At, State::Initial) => {
                    repr.idents.push(Span::new(0, 1));
                }
                (Token::Ident(span), State::Initial | State::AfterIdent) => {
                    repr.idents.push(span);
                }
                (Token::Arbitrary(arb), State::Initial | State::AfterIdent) => {
                    repr.arbitrary = Some(arb);
                }
                (Token::Ident(span), State::AfterSlash) if modifier.is_none() => {
                    modifier = Some(Either::Left(span));
                }
                (Token::Ident(span), State::AfterSlashIdent)
                    if modifier.is_some_and(|m| m.is_left()) =>
                {
                    let Some(Left(s)) = modifier else { return None };
                    modifier = Some(Left(s.to(&span)))
                }
                (Token::Arbitrary(arb), State::AfterSlash) if repr.modifier.is_none() => {
                    modifier = Some(Either::Right(arb));
                }
                (Token::Slash, State::AfterArbitrary | State::AfterIdent) => {}
                _ => return None,
            }
            state = new_state;
        }
        repr.modifier = modifier.map(|m| match m {
            Left(span) => MaybeArbitrary::Named(&self.input[span]),
            Right(arb) => MaybeArbitrary::Arbitrary(arb),
        });
        Some(repr)
    }

    pub fn parse_variant(&mut self, v: &VariantStorage) -> Option<VariantCandidate<'a>> {
        // try static match
        if let Some(variant) = v.get(self.input) {
            return (variant.kind == VariantKind::Static)
                .then(|| VariantCandidate::new(variant.clone(), self.input));
        }

        let _span = span!(tracing::Level::TRACE, "parse_variant", input = self.input).entered();

        let repr = self.parse_variant_repr()?;

        // full arbitrary
        if let (Some(arb), true) = (repr.arbitrary, repr.idents.is_empty()) {
            return Some(
                VariantCandidate::new(
                    Variant::new_static([arb]).with_ordering(VariantOrdering::Arbitrary),
                    arb,
                )
                .arbitrary(),
            );
        }

        let mut layers = smallvec![];
        let slice = &mut repr.idents.as_slice();
        while let Some((key, variant)) = find_key(self.input, v, slice) {
            match variant.kind {
                VariantKind::Composable => {
                    layers.push(*variant.take_composable()?);
                }
                VariantKind::Dynamic => {
                    return VariantCandidate::new(variant, key)
                        .with_value(
                            (!slice.is_empty())
                                .then(|| MaybeArbitrary::Named(substr(self.input, slice)))
                                .or_else(|| repr.arbitrary.map(MaybeArbitrary::Arbitrary)),
                        )
                        .with_modifier(repr.modifier)
                        .with_layers(layers)
                        .into();
                }
                VariantKind::Static => {
                    // must exhausted
                    if !slice.is_empty() {
                        return None;
                    }
                    return VariantCandidate::new(variant, key)
                        .with_layers(layers)
                        .with_modifier(repr.modifier)
                        .into();
                }
            }
        }

        None
    }
}

fn substr<'a>(input: &'a str, span: &[Span]) -> &'a str {
    &input[span.first().unwrap().to(span.last().unwrap())]
}

fn find_key<'a>(
    input: &'a str,
    v: &VariantStorage,
    spans: &mut &[Span],
) -> Option<(&'a str, Variant)> {
    let first = spans.first()?;

    for (idx, i) in spans.iter().enumerate().rev() {
        let key = &input[first.to(i)];
        if let Some(variant) = v.get(key) {
            *spans = &spans[(idx + 1)..];
            return Some((key, variant.clone()));
        }
    }

    None
}
