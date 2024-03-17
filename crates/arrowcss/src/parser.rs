
use crate::{
    context::Context,
    css::{CSSRule, CSSStyleRule},
    variant_parse::{
        ArbitraryVariant, ArbitraryVariantKind, MatchVariant, Variant,
        VariantKind,
    },
};
use cssparser::{BasicParseError, BasicParseErrorKind, Parser, ParserInput};
use lazy_static::lazy_static;
use regex::Regex;

pub trait Parse<T> {
    fn parse(ctx: &Context, input: T) -> Option<Self>
    where
        Self: Sized;
}

lazy_static! {
    static ref EXTRACT_RE: Regex = Regex::new(r#"[\\:]?[\s'"`;{}]+"#).unwrap();
}

fn to_css_rule<'a>(value: &'a str, ctx: &Context<'a>) -> Option<CSSRule> {
    let mut input = ParserInput::new(value);
    let mut parser = Parser::new(&mut input);

    let mut variants = vec![];
    while let Ok(variant) = parser.try_parse(Variant::parse) {
        variants.push(variant);
    }

    let start = parser.position();
    let rule;
    loop {
        if let Err(BasicParseError {
            kind: BasicParseErrorKind::EndOfInput,
            ..
        }) = parser.next() {
            rule = parser.slice(start..parser.position()).to_owned();
            break;
        }
    }

    // Step 2: try static match
    let mut decls: Vec<CSSRule> = vec![];
    if let Some(static_rule) = ctx.static_rules.borrow().get(&rule) {
        decls = static_rule
            .to_vec()
            .into_iter()
            .map(CSSRule::Decl)
            .collect();
    } else {
        // Step 3: get all index of `-`
        for (i, _) in rule.match_indices('-') {
            if let Some(v) = ctx
                .rules
                .borrow()
                .get(rule.get(..i)?)
                .and_then(|func_vec| {
                    func_vec
                        .iter()
                        .find_map(|func| func(rule.get((i + 1)..)?))
                })
            {
                decls.append(
                    &mut v.to_vec().into_iter().map(CSSRule::Decl).collect(),
                );
                break;
            }
        }
    }

    if decls.is_empty() {
        return None;
    }

    let mut rule = CSSRule::Style(CSSStyleRule {
        selector: rule.to_string(),
        nodes: decls,
    });

    // Step 4: apply variants
    let (at_rules_variants, plain_variants): (Vec<_>, Vec<_>) = variants
        .iter()
        .filter_map(|variant| match &variant.kind {
            VariantKind::Arbitrary(_) => Some(variant),
            VariantKind::Literal(v) => {
                ctx.variants.borrow().contains_key(&v.value).then_some(variant)
            }
        })
        .partition(|variant| match &variant.kind {
            VariantKind::Arbitrary(ArbitraryVariant {
                kind: ArbitraryVariantKind::Nested,
                ..
            }) => true,
            VariantKind::Literal(v) => {
                ctx.variants.borrow().get(&v.value).is_some_and(|v| v.needs_nesting)
            }
            _ => false,
        });

    for variant in plain_variants.iter().chain(at_rules_variants.iter()) {
        match &variant.kind {
            VariantKind::Arbitrary(arbitrary_variant) => {
                let new_rule = arbitrary_variant.match_variant(rule)?;
                rule = new_rule;
            }
            VariantKind::Literal(v) => {
                let new_rule = (ctx.variants.borrow()[&v.value].handler)(rule)?;
                rule = new_rule;
            }
        }
    }

    Some(rule)
}

pub fn parse<'b>(input: &'b str, ctx: &Context<'b>) {
    let parts = EXTRACT_RE.split(input);
    for token in parts.into_iter() {
        if token.is_empty() {
            continue;
        }
        if ctx.tokens.borrow().contains_key(token) {
            continue;
        }
        let ctx_clone = ctx.clone();
        ctx.tokens
            .borrow_mut()
            .insert(token, to_css_rule(token, &ctx_clone));
    }
}
