use std::fmt::Write;

use cssparser::serialize_identifier;
use either::Either;
use lazy_static::lazy_static;
use regex::Regex;
use smallvec::SmallVec;

use crate::context::utilities::UtilityStorage;
use crate::css::rule::RuleList;
use crate::process::{StaticHandler, VariantProcessor};
use crate::{
    context::Context, parsing::UtilityParser, parsing::VariantParser,
    utils::TopLevelPattern,
};

lazy_static! {
    pub static ref EXTRACT_RE: Regex = Regex::new(r#"[\s"';{}`]+"#).unwrap();
}

pub fn to_css_rule<'c>(value: &str, ctx: &Context<'c>) -> Option<RuleList<'c>> {
    let mut parts = value.split(TopLevelPattern::new(':')).rev();

    let utility = parts.next().unwrap();
    let u = UtilityParser::new(utility).parse(ctx)?;

    let variants = parts.rev().collect::<SmallVec<[_; 2]>>();

    let vs = variants
        .into_iter()
        .map(|v| VariantParser::new(v).parse(ctx))
        .collect::<Option<Vec<_>>>()?;

    let (nested, selector): (Vec<_>, Vec<_>) = vs.into_iter().partition(|v| {
        matches!(
            v.processor,
            VariantProcessor {
                handler: Either::Left(StaticHandler::Nested(_)),
                ..
            }
        )
    });

    let node = ctx
        .utilities
        .try_apply(u)
        .or(ctx.static_rules.try_apply(u))?;

    let mut node =
        selector.into_iter().fold(node.to_rule_list(), |acc, cur| {
            let processor = ctx.variants.get(cur.key).unwrap();
            processor.process(cur, acc)
        });

    let mut w = String::with_capacity(value.len() + 5);
    w.write_char('.').ok()?;
    serialize_identifier(value, &mut w).ok()?;

    node.iter_mut().for_each(|rule| {
        rule.selector = rule.selector.replace('&', &w);
    });

    let node = nested.into_iter().fold(node, |acc, cur| {
        let processor = ctx.variants.get(cur.key).unwrap();
        processor.process(cur, acc)
    });

    node.into()
    // node.modify_with(|s| s.replace("&", &w))
    //     .to_rule_list()
    //     .into()
}

#[cfg(test)]
mod tests {
    use arrowcss_css_macro::css;

    use crate::{context::AddRule, process::UtilityProcessor};

    use super::*;

    #[test]
    fn test_to_css_rule() {
        let mut ctx = Context::default();
        ctx.add_rule("text", UtilityProcessor::new(|_, v| css!("color": v)));
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

        let node: RuleList = ctx
            .utilities
            .try_apply(u)
            .or(ctx.static_rules.try_apply(u))
            .unwrap()
            .into();

        let node = vs.into_iter().fold(node, |acc, cur| {
            let processor = ctx.variants.get(cur.key).unwrap();
            processor.process(cur, acc)
        });
        println!("{:#?}", node);
    }
}
