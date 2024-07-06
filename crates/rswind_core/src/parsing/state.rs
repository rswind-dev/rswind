use std::fmt::Debug;

use tracing::trace;

use super::candidate::Token;

pub trait StateTransformer {
    // this `Result` only used for `transpose` the
    #[allow(clippy::result_unit_err)]
    fn transform(state: &State, token: &Token) -> Option<State>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    /// Initial state
    ///
    /// Accepts: Ident, Arbitrary
    Initial,
    /// At least one Ident has been parsed
    ///
    /// Accepts: Ident, Arbitrary, Colon, Slash
    AfterIdent,
    /// After Arbitrary
    ///
    /// Accepts: Colon, Slash
    AfterArbitrary,
    /// After Slash
    ///
    /// Accepts: Ident, Arbitrary
    AfterSlash,
    /// After Slash Ident
    ///
    /// Accepts: Ident
    AfterSlashIdent,
    /// Done
    Eof,
}

pub struct UtilityTransformer;

impl StateTransformer for UtilityTransformer {
    fn transform(state: &State, token: &Token) -> Option<State> {
        let res = match (&state, token) {
            (State::Initial, Token::Bang) => Some(State::Initial),
            (State::Initial, Token::Minus) => Some(State::Initial),
            (State::Initial, Token::Ident(_)) => Some(State::AfterIdent),
            (State::Initial, Token::Arbitrary(_)) => Some(State::AfterArbitrary),
            (State::AfterIdent, Token::Ident(_)) => Some(State::AfterIdent),
            (State::AfterIdent, Token::Arbitrary(_)) => Some(State::AfterArbitrary),
            (State::AfterIdent, Token::Slash) => Some(State::AfterSlash),
            (State::AfterIdent, Token::Bang) => Some(State::Eof),
            (State::AfterArbitrary, Token::Bang) => Some(State::Eof),
            (State::AfterArbitrary, Token::Slash) => Some(State::AfterSlash),
            (State::AfterSlash, Token::Ident(_)) => Some(State::Eof),
            (State::AfterSlashIdent, Token::Ident(_)) => Some(State::AfterSlashIdent),
            (State::AfterSlash, Token::Arbitrary(_)) => Some(State::Eof),
            _ => None,
        };
        trace!(from = ?state, to = ?res, token = ?token, "transform");
        res
    }
}

pub struct VariantTransformer;

impl StateTransformer for VariantTransformer {
    fn transform(state: &State, token: &Token) -> Option<State> {
        let res = match (&state, token) {
            (State::Initial, Token::Ident(_)) => Some(State::AfterIdent),
            (State::Initial, Token::At) => Some(State::AfterIdent),
            (State::Initial, Token::Arbitrary(_)) => Some(State::AfterArbitrary),
            (State::AfterIdent, Token::Ident(_)) => Some(State::AfterIdent),
            (State::AfterIdent, Token::Arbitrary(_)) => Some(State::AfterArbitrary),
            (State::AfterIdent, Token::Slash) => Some(State::AfterSlash),
            (State::AfterArbitrary, Token::Slash) => Some(State::AfterSlash),
            (State::AfterSlash, Token::Ident(_)) => Some(State::AfterSlashIdent),
            (State::AfterSlashIdent, Token::Ident(_)) => Some(State::AfterSlashIdent),
            (State::AfterSlash, Token::Arbitrary(_)) => Some(State::Eof),
            _ => None,
        };
        trace!(from = ?state, to = ?res, token = ?token, "transform");
        res
    }
}
