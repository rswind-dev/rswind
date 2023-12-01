use crate::{parser::Parse, utils::StripArbitrary};

#[derive(Debug, PartialEq)]
pub struct Variant {
    raw: String,
    name: String,
    modifier: Option<Modifier>,
    arbitrary: Option<String>,
}

#[derive(Debug, PartialEq)]
enum Modifier {
    Literal(String),
    Arbitrary(String),
}

impl Parse<&str> for Variant {
    fn parse<'a>(
        ctx: &'a crate::context::Context,
        input: &str,
    ) -> Option<Self> {
        // Step 1: extract modifier
        let mut modifier: Option<Modifier> = None;
        let mut variant = input;
        if let Some((rest, modi)) = input.rsplit_once('/') {
            if let Some(ar) = modi.strip_arbitrary() {
                modifier = Some(Modifier::Arbitrary(ar.into()));
            } else {
                modifier = Some(Modifier::Literal(modi.into()));
            }
            variant = rest;
        }

        // Step 2: check if it's an fully arbitrary modifier
        if variant.starts_with('[') {
            if let Some(extracted) = input.strip_arbitrary() {
                return Some(Variant {
                    raw: input.into(),
                    name: String::new(),
                    modifier,
                    arbitrary: Some(extracted.into()),
                });
            }
        }

        // Step 3: check if it's a group-[&:hover] like
        if let Some((rest, maybe_arbitrary)) = variant.rsplit_once('-') {
            // is arbitrary
            if let Some(ar) = maybe_arbitrary.strip_arbitrary() {
                return Some(Variant {
                    raw: input.into(),
                    name: rest.into(),
                    modifier,
                    arbitrary: Some(ar.into()),
                });
            } else {
                return Some(Variant {
                    raw: input.into(),
                    name: variant.into(),
                    modifier,
                    arbitrary: None,
                });
            }
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use crate::context::Context;

    use super::*;

    #[test]
    fn parse_variants() {
        let ctx = Context::default();
        let res = Variant::parse(&ctx, "[@media(min-width:200px)]").unwrap();
        assert_eq!(
            res,
            Variant {
                raw: "[@media(min-width:200px)]".into(),
                name: "".into(),
                modifier: None,
                arbitrary: Some("@media(min-width:200px)".into()),
            }
        );
    }

    #[test]
    fn parse_variants_with_modifier() {
        let ctx = Context::default();
        let res = Variant::parse(&ctx, "group-hover/sidebar").unwrap();
        assert_eq!(
            res,
            Variant {
                raw: "group-hover/sidebar".into(),
                name: "group-hover".into(),
                modifier: Some(Modifier::Literal("sidebar".into())),
                arbitrary: None,
            }
        );
    }

    #[test]
    fn parse_variants_with_modifier_and_arbitrary() {
        let ctx = Context::default();
        let res = Variant::parse(&ctx, "group-hover/[&:hover]").unwrap();
        assert_eq!(
            res,
            Variant {
                raw: "group-hover/[&:hover]".into(),
                name: "group-hover".into(),
                modifier: Some(Modifier::Arbitrary("&:hover".into())),
                arbitrary: None,
            }
        );
    }

    #[test]
    fn parse_arbitrary_variants_with_arbitrary_modifier() {
        let ctx = Context::default();
        let res = Variant::parse(&ctx, "group-[&:hover]/[sidebar]").unwrap();
        assert_eq!(
            res,
            Variant {
                raw: "group-[&:hover]/[sidebar]".into(),
                name: "group".into(),
                modifier: Some(Modifier::Arbitrary("sidebar".into())),
                arbitrary: Some("&:hover".into()),
            }
        );
    }
}