use cssparser::{
    BasicParseError, BasicParseErrorKind, ParseError, Parser, Token,
};

#[derive(Debug, PartialEq)]
struct Variant<'a> {
    raw: String,
    kind: VariantKind<'a>,
}

#[derive(Debug, PartialEq)]
enum VariantKind<'a> {
    Arbitrary(ArbitraryVariant),
    Literal(LiteralVariant<'a>),
}

// TODO: name better
// Replacement: &:nth-child(3)
// Nested: @media
#[derive(Debug, PartialEq)]
enum ArbitraryVariantKind {
    Replacement,
    Nested,
}

// Something like [@media(min-width: 300px)] or [&:nth-child(3)]
#[derive(Debug, PartialEq)]
struct ArbitraryVariant {
    kind: ArbitraryVariantKind,
    value: String,
}

#[derive(Debug, PartialEq)]
enum Modifier<'a> {
    Arbitrary(Token<'a>),
    Literal(String),
}

#[derive(Debug, PartialEq)]
struct LiteralVariant<'a> {
    value: String,
    modifier: Option<Modifier<'a>>,
    arbitrary: Option<String>,
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
        parser.parse_nested_block(|parser| loop {
            match parser.next() {
                Err(BasicParseError {
                    kind: BasicParseErrorKind::EndOfInput,
                    ..
                }) => {
                    return Ok(Self {
                        kind: ArbitraryVariantKind::Nested,
                        value: parser
                            .slice(start.position()..parser.position())
                            .to_string(),
                    });
                }
                Ok(Token::AtKeyword(at_rule)) => {
                    println!("at_rule: {at_rule:?}");
                }
                other => {
                    println!("other: {:?}", other);
                }
            }
        })
    }
}

fn parse_arbitrary<'i, 'a>(
    parser: &mut Parser<'i, 'a>,
) -> Result<String, ParseError<'a, ()>> {
    let start = parser.state();
    while let Err(e) = parser.next() {
        match e.kind {
            BasicParseErrorKind::EndOfInput => {
                let value = parser.slice(start.position()..parser.position());
                println!("{:?}", value);
                return Ok(value.get(..value.len() - 1).unwrap().into());
            }
            _ => {
                parser.reset(&start);
                return Err(parser.new_custom_error(()));
            }
        }
    }
    Err(parser.new_custom_error(()))
}

impl<'i> Variant<'i> {
    fn parse<'a>(
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
                        parser.expect_colon();
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
        let _ = ident.strip_suffix('-');
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

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::{Parser, ParserInput};

    #[test]
    fn test_variant_parse() {
        let mut input =
            ParserInput::new("group-[&:hover]/[sidebar]:@md:[@media(min-width:200px)]:text-blue-500");
        let mut parser = Parser::new(&mut input);
        let mut list = vec![];
        while let Ok(variant) = parser.try_parse(Variant::parse) {
            list.push(variant);
        }

        println!("{:#?}", list);
    }

    #[test]
    fn test_plain_variant() {
        let mut input = ParserInput::new("group-hover:");
        let mut parser = Parser::new(&mut input);
        let variant = Variant::parse(&mut parser).unwrap();

        assert_eq!(
            variant,
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
}
