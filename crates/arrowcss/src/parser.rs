use crate::{
    context::Context,
    css::{CssDecls, CssRule, CssRuleList, StyleRule},
    utils::VariantHandler,
    variant_parse::{
        ArbitraryVariant, ArbitraryVariantKind, MatchVariant, Variant,
        VariantKind,
    },
};
use cssparser::{serialize_identifier, ParseError, ParserInput};
use lazy_static::lazy_static;
use lightningcss::traits::IntoOwned;
use regex::Regex;

lazy_static! {
    static ref EXTRACT_RE: Regex = Regex::new(r#"[\\:]?[\s'"`;{}]+"#).unwrap();
}

fn to_css_rule<'c, 'i>(
    value: &'i str,
    ctx: &mut Context<'c>,
) -> Option<CssRuleList<'i>> {
    let mut input = ParserInput::new(value);
    let mut parser = cssparser::Parser::new(&mut input);

    let mut variants = vec![];
    while let Ok(variant) = parser.try_parse(Variant::parse) {
        variants.push(variant);
    }

    let start = parser.position();
    let _ = parser.parse_entirely(|p| {
        while let Ok(_) = p.next() {}
        Ok::<(), ParseError<'_, ()>>(())
    });
    let rule = parser.slice(start..parser.position());

    // Step 2: try static match
    let mut decls = CssDecls::default();
    if let Some(static_rule) = ctx.get_static(rule) {
        decls = static_rule.clone();
    } else {
        // Step 3: get all index of `-`
        for (i, _) in rule.match_indices('-') {
            if let Some(v) =
                ctx.utilities.try_apply(rule.get(..i)?, rule.get(i + 1..)?)
            {
                decls = v.into_owned().into();
            }
        }
    }

    if decls.is_empty() {
        return None;
    }

    let mut selector = String::new();
    let _ = serialize_identifier(value, &mut selector);
    let mut rule: CssRuleList = CssRule::Style(StyleRule {
        selector,
        nodes: vec![decls
            .0
            .iter()
            .map(|decl| CssRule::Decl(decl.clone()))
            .collect::<CssRuleList>()],
    })
    .into();

    // Step 4: apply variants
    let (at_rules_variants, plain_variants): (Vec<_>, Vec<_>) = variants
        .into_iter()
        .filter_map(|variant| match &variant.kind {
            VariantKind::Arbitrary(_) => Some(variant),
            VariantKind::Literal(v) => ctx
                .variants
                .borrow()
                .contains_key(&v.value)
                .then_some(variant),
        })
        .partition(|variant| match &variant.kind {
            VariantKind::Arbitrary(ArbitraryVariant {
                kind: ArbitraryVariantKind::Nested,
                ..
            }) => true,
            VariantKind::Literal(v) => {
                ctx.variants.borrow().get(&v.value).is_some_and(|v| {
                    matches!(v.as_ref(), VariantHandler::Nested(_))
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
                let new_rule = (ctx.variants.borrow()[&v.value])(rule)?;
                rule = new_rule;
            }
        }
    }

    Some(rule)
}

pub fn parse<'c, 'i>(
    input: &'i str,
    ctx: &mut Context<'c>,
) -> Vec<CssRuleList<'i>> {
    let parts = EXTRACT_RE.split(input);
    let mut tokens: Vec<CssRuleList> = vec![];
    for token in parts.into_iter() {
        if token.is_empty() {
            continue;
        }
        // if ctx.tokens.borrow().contains_key(token) {
        //     continue;
        // }
        // let ctx_clone = ctx.clone();
        to_css_rule(token, ctx).map(|rule| tokens.push(rule));
        // ctx.tokens
        //     .borrow_mut()
        //     .insert(token.to_string(), to_css_rule(token, ctx_clone));
    }
    tokens
}
