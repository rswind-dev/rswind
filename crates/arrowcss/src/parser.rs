use crate::{
    context::Context,
    css::{AstNode, NodeList, Rule},
    utils::VariantHandler,
    variant::{
        ArbitraryVariant, ArbitraryVariantKind, MatchVariant, Variant,
        VariantKind,
    },
};
use cssparser::{serialize_identifier, ParseError, ParserInput};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref EXTRACT_RE: Regex = Regex::new(r#"[\s"';{}`]+"#).unwrap();
}

pub fn to_css_rule<'i, 'c>(
    value: &'i str,
    ctx: &Context<'c>,
) -> Option<NodeList<'c>> {
    let mut input = ParserInput::new(value);
    let mut parser = cssparser::Parser::new(&mut input);

    let mut variants = vec![];
    while let Ok(variant) = parser.try_parse(Variant::parse) {
        variants.push(variant);
    }

    let start = parser.position();
    let _ = parser.parse_entirely(|p| {
        while p.next().is_ok() {}
        Ok::<(), ParseError<'_, ()>>(())
    });
    let rule = parser.slice(start..parser.position());

    // Step 2: try static match
    let mut decls = NodeList::default();
    if let Some(static_rule) = ctx.get_static(rule) {
        decls = static_rule.into();
    } else {
        // Step 3: get all index of `-`
        for (i, _) in rule.match_indices('-') {
            if let Some(v) =
                ctx.utilities.try_apply(rule.get(..i)?, rule.get(i + 1..)?)
            {
                decls = v;
            }
        }
    }

    if decls.is_empty() {
        return None;
    }

    let mut selector = String::from(".");
    selector.reserve(value.len() + 5);
    let _ = serialize_identifier(value, &mut selector);
    let mut rule: NodeList = AstNode::Rule(Rule {
        selector,
        nodes: decls.to_vec(),
    })
    .into();

    // Step 4: apply variants
    let (at_rules_variants, plain_variants): (Vec<_>, Vec<_>) = variants
        .into_iter()
        .filter_map(|variant| match &variant.kind {
            VariantKind::Arbitrary(_) => Some(variant),
            VariantKind::Literal(v) => ctx
                .variants
                .contains_key(&v.value)
                .then_some(variant),
        })
        .partition(|variant| match &variant.kind {
            VariantKind::Arbitrary(ArbitraryVariant {
                kind: ArbitraryVariantKind::Nested,
                ..
            }) => true,
            VariantKind::Literal(v) => {
                ctx.variants.get(&v.value).is_some_and(|v| {
                    matches!(v, VariantHandler::Nested(_))
                })
            }
            _ => false,
        });

    for variant in plain_variants
        .into_iter()
        .chain(at_rules_variants.into_iter())
    {
        match variant.kind {
            VariantKind::Arbitrary(arbitrary_variant) => {
                let new_rule = arbitrary_variant.match_variant(rule)?;
                rule = new_rule;
            }
            VariantKind::Literal(v) => {
                let new_rule = (ctx.variants[&v.value])(rule)?;
                rule = new_rule;
            }
        }
    }

    Some(rule)
}
