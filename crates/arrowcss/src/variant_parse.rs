use cssparser::{
    BasicParseError, BasicParseErrorKind, ParseError, Parser, ParserInput,
    Token,
};

use crate::css::{Container, CSSAtRule, CSSRule};

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub raw: String,
    pub kind: VariantKind,
}

#[derive(Debug, PartialEq)]
pub enum VariantKind {
    Arbitrary(ArbitraryVariant),
    Literal(LiteralVariant),
}

// TODO: name better
// Replacement: &:nth-child(3)
// Nested: @media
#[derive(Debug, PartialEq)]
pub enum ArbitraryVariantKind {
    Replacement,
    Nested,
}

// MatchVariant trait has a VariantMatchingFn function
pub trait MatchVariant {
    fn match_variant(self, container: Container) -> Option<Container>;
}

// Something like [@media(min-width:300px)] or [&:nth-child(3)]
#[derive(Debug, PartialEq)]
pub struct ArbitraryVariant {
    pub kind: ArbitraryVariantKind,
    pub value: String,
}

impl MatchVariant for ArbitraryVariant {
    fn match_variant(self, mut container: Container) -> Option<Container> {
        match self.kind {
            ArbitraryVariantKind::Replacement => {
                for node in container.nodes.iter_mut() {
                    match node {
                        CSSRule::Style(ref mut it) => {
                            it.selector = self.value.replace('&', &it.selector);
                        }
                        _ => {},
                    }
                }
                Some(container)
            }
            ArbitraryVariantKind::Nested => Some(
                CSSRule::AtRule(CSSAtRule {
                    name: self.value.trim_start_matches('@').to_owned(),
                    params: "".into(),
                    nodes: vec![container],
                })
                .into(),
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Arbitrary(String),
    Literal(String),
}

#[derive(Debug, PartialEq)]
pub struct LiteralVariant {
    pub value: String,
    pub modifier: Option<Modifier>,
    pub arbitrary: Option<String>,
}

enum ParserError {
    UnexpectedToken,
    UnexpectedEnd,
}

impl<'i> ArbitraryVariant {
    fn parse<'a>(
        parser: &mut Parser<'i, 'a>,
    ) -> Result<Self, ParseError<'a, ()>> {
        let start = parser.state();
        let mut kind = None;
        parser.parse_nested_block(|parser| loop {
            match parser.next() {
                Err(BasicParseError {
                    kind: BasicParseErrorKind::EndOfInput,
                    ..
                }) => {
                    return Ok(Self {
                        // return Err when kind is None
                        kind: kind.ok_or(parser.new_custom_error(()))?,
                        value: parser
                            .slice(start.position()..parser.position())
                            .to_string(),
                    });
                }
                Ok(Token::AtKeyword(at_rule)) => {
                    kind = Some(ArbitraryVariantKind::Nested);
                }
                Ok(Token::Delim('&')) => {
                    kind = Some(ArbitraryVariantKind::Replacement);
                }
                other => {
                    println!("other: {:?}", other);
                }
            }
        })
    }
}

// fn parse_arbitrary<'i, 'a>(
//     parser: &mut Parser<'i, 'a>,
// ) -> Result<String, ParseError<'a, ()>> {
//     let start = parser.state();
//     while let Err(e) = parser.next() {
//         match e.kind {
//             BasicParseErrorKind::EndOfInput => {
//                 let value = parser.slice(start.position()..parser.position());
//                 println!("{:?}", value);
//                 return Ok(value.get(..value.len() - 1).unwrap().into());
//             }
//             _ => {
//                 parser.reset(&start);
//                 return Err(parser.new_custom_error(()));
//             }
//         }
//     }
//     Err(parser.new_custom_error(()))
// }

impl<'i> Variant {
    pub fn parse<'a>(
        parser: &mut Parser<'i, 'a>,
    ) -> Result<Self, ParseError<'a, ()>> {
        let mut is_first_token = true;
        let mut ident = String::new();
        let mut arbitrary: Option<String> = None;

        let start_state = parser.state();

        loop {
            match parser.next() {
                Ok(Token::Colon) => {
                    break;
                }
                Ok(Token::SquareBracketBlock) => {
                    if is_first_token {
                        // trait as ArbitraryVariant
                        let arbitrary_variant =
                            parser.try_parse(ArbitraryVariant::parse)?;
                        parser.expect_colon()?;
                        return Ok(Self {
                            raw: parser
                                .slice(
                                    start_state.position()..parser.position(),
                                )
                                .into(),
                            kind: VariantKind::Arbitrary(arbitrary_variant),
                        });
                    } else {
                        arbitrary = parser
                            .parse_nested_block(|parser| {
                                let start = parser.state();
                                while let e = parser.next() {
                                    match e {
                                        Err(BasicParseError {
                                            kind:
                                                BasicParseErrorKind::EndOfInput,
                                            ..
                                        }) => {
                                            return Ok(parser
                                                .slice(
                                                    start.position()
                                                        ..parser.position(),
                                                )
                                                .into());
                                        }
                                        Ok(_) => {}
                                        _ => {
                                            return Err(parser
                                                .new_custom_error::<(), ()>(
                                                    (),
                                                ));
                                        }
                                    }
                                }
                                Err(parser.new_custom_error(()))
                            })
                            .ok();
                    }
                }
                Ok(Token::Delim('/')) => {
                    // modifier = parser.try_parse(Modifier::parse);
                }
                Ok(Token::Ident(id)) => {
                    ident = id.to_string();
                }
                Ok(token @ Token::Delim('@')) => {
                    ident += "@";
                }
                Ok(Token::AtKeyword(at_rule)) => {
                    println!("{:?}", at_rule);
                    ident += "@";
                    ident += at_rule;
                }
                Ok(token) => {
                    println!("{:?}", token);
                }
                Err(e) => {
                    parser.reset(&start_state);
                    return Err(parser.new_custom_error(()));
                }
            }
            is_first_token = false;
        }

        // remove the last `-` if it exists
        if ident.ends_with('-') {
            ident.pop();
        }

        Ok(Self {
            raw: parser
                .slice(start_state.position()..parser.position())
                .into(),
            kind: VariantKind::Literal(LiteralVariant {
                value: ident,
                modifier: None,
                arbitrary,
            }),
        })
    }
}

pub fn create_variant(input: &str) -> Option<Variant> {
    let mut input = ParserInput::new(input);
    let mut parser = Parser::new(&mut input);
    Variant::parse(&mut parser).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_variant() {
        assert_eq!(
            create_variant("group-hover:").unwrap(),
            Variant {
                raw: "group-hover:".into(),
                kind: VariantKind::Literal(LiteralVariant {
                    value: "group-hover".into(),
                    modifier: None,
                    arbitrary: None,
                })
            }
        );
    }

    #[test]
    fn test_arbitrary_variant() {
        assert_eq!(
            create_variant("[@media(min-width:200px)]:").unwrap(),
            Variant {
                raw: "[@media(min-width:200px)]:".into(),
                kind: VariantKind::Arbitrary(ArbitraryVariant {
                    kind: ArbitraryVariantKind::Nested,
                    value: "@media(min-width:200px)".into(),
                })
            }
        );
    }

    // group-[&:hover]
    #[test]
    fn test_literal_variant_with_arbitrary() {
        assert_eq!(
            create_variant("group-[&:hover]:").unwrap(),
            Variant {
                raw: "group-[&:hover]:".into(),
                kind: VariantKind::Literal(LiteralVariant {
                    value: "group".into(),
                    modifier: None,
                    arbitrary: Some("&:hover".into()),
                })
            }
        );
    }

    // group-[&:hover]/sidebar
    #[test]
    fn test_literal_variant_with_arbitrary_and_literal_modifier() {
        // TODO: fix this
        // assert_eq!(
        //     create_variant("group-[&:hover]/sidebar:").unwrap(),
        //     Variant {
        //         raw: "group-[&:hover]/sidebar:".into(),
        //         kind: VariantKind::Literal(LiteralVariant {
        //             value: "group".into(),
        //             modifier: Some(Modifier::Literal("sidebar".into())),
        //             arbitrary: Some("&:hover".into()),
        //         })
        //     }
        // );
    }

    // group-[&:hover]/[sidebar]
    #[test]
    fn test_literal_variant_with_arbitrary_and_modifier() {
        assert_eq!(
            create_variant("group-[&:hover]/[sidebar]:").unwrap(),
            Variant {
                raw: "group-[&:hover]/[sidebar]:".into(),
                kind: VariantKind::Literal(LiteralVariant {
                    value: "group".into(),
                    modifier: Some(Modifier::Arbitrary("sidebar".into())),
                    arbitrary: Some("&:hover".into()),
                })
            }
        );
    }

    // @md
    #[test]
    fn test_at() {
        assert_eq!(
            create_variant("@md:").unwrap(),
            Variant {
                raw: "@md:".into(),
                kind: VariantKind::Literal(LiteralVariant {
                    value: "@md".into(),
                    modifier: None,
                    arbitrary: None,
                })
            }
        );
    }
}
