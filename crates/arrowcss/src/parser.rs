use std::{collections::BTreeSet, fmt::Write};

use cssparser::serialize_identifier;
use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::{
    context::{utilities::UtilityStorage, Context},
    css::rule::RuleList,
    ordering::OrderingKey,
    parsing::{UtilityParser, VariantParser},
    process::{StaticHandler, UtilityGroup, Variant, VariantHandler},
    utils::TopLevelPattern,
};

#[derive(Debug, Clone)]
pub struct GenerateResult {
    pub rule: RuleList,
    pub group: Option<UtilityGroup>,
    pub ordering: OrderingKey,
    pub variants: BTreeSet<Variant>,
}

pub fn to_css_rule(value: &str, ctx: &Context) -> Option<GenerateResult> {
    let mut parts: SmallVec<[&str; 2]> = value.split(TopLevelPattern::new(':')).collect();

    let utility = parts.pop()?;
    let utility_candidate = UtilityParser::new(utility).parse(ctx)?;

    let variants = parts;

    let vs = variants
        .into_iter()
        .map(|v| VariantParser::new(v).parse(ctx))
        .collect::<Option<SmallVec<[_; 2]>>>()?;

    let (nested, selector): (SmallVec<[_; 1]>, SmallVec<[_; 1]>) = vs.into_iter().partition(|v| {
        matches!(
            v.processor,
            Variant {
                handler: VariantHandler::Static(StaticHandler::Nested(_)),
                ..
            }
        )
    });

    let (node, ordering, group) = ctx.utilities.try_apply(utility_candidate)?;

    let mut node = selector
        .iter()
        .fold(node.to_rule_list(), |acc, cur| cur.handle(acc));

    let mut w = String::with_capacity(value.len() + 5);
    w.write_char('.').ok()?;
    serialize_identifier(value, &mut w).ok()?;

    node = node.modify_with(|s| SmolStr::from(s.replace('&', &w)));

    let node = nested.iter().fold(node, |acc, cur| cur.handle(acc));

    Some(GenerateResult {
        group,
        rule: node,
        ordering,
        variants: BTreeSet::new(),
        // variants: nested
        //     .into_iter()
        //     .chain(selector)
        //     .map(|p| p.processor)
        //     .collect(),
    })
}

#[cfg(test)]
mod tests {
    use arrowcss_css_macro::css;

    use super::*;
    use crate::process::Utility;

    #[test]
    fn test_to_css_rule() {
        let mut ctx = Context::default();
        ctx.add_utility("text", Utility::new(|_, v| css!("color": v)));
        ctx.add_variant("hover", ["&:hover"]);
        ctx.add_variant("marker", ["&::marker", "& > *::marker"]);

        let value = "hover:marker:text-[#123456]";
        let mut parts = value.split(TopLevelPattern::new(':')).rev();

        let utility = parts.next().unwrap();
        let u = UtilityParser::new(utility).parse(&ctx).unwrap();

        let variants = parts.rev().collect::<SmallVec<[_; 2]>>();

        let vs = variants
            .into_iter()
            .map(|v| VariantParser::new(v).parse(&ctx))
            .collect::<Option<Vec<_>>>()
            .unwrap();

        let (node, _, _) = ctx.utilities.try_apply(u).unwrap();
        let node = node.to_rule_list();

        let node = vs.into_iter().fold(node, |acc, cur| {
            let processor = ctx.variants.get(cur.key).unwrap();
            processor.process(cur, acc)
        });
        println!("{:#?}", node);
    }
}
