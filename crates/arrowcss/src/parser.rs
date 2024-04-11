use std::fmt::Write;

use cssparser::serialize_identifier;
use lazy_static::lazy_static;
use regex::Regex;
use smallvec::SmallVec;

use crate::context::utilities::UtilityStorage;
use crate::css::rule::RuleList;
use crate::css::Rule;
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
    #[allow(unused_variables)]
    let vs = variants
        .into_iter()
        .map(|v| VariantParser::new(v).parse(ctx))
        .collect::<Vec<_>>();

    let node = ctx.utilities.try_apply(u);

    let mut w = String::with_capacity(utility.len() + 5);
    w.write_char('.').ok()?;
    serialize_identifier(utility, &mut w).ok()?;

    Some(
        Rule {
            selector: w,
            rules: vec![node?].into(),
            ..Default::default()
        }
        .into(),
    )
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

        let node: RuleList = ctx.utilities.try_apply(u).unwrap().into();

        let node = vs.into_iter().fold(node, |acc, cur| {
            let processor = ctx.variants.get(cur.key).unwrap();
            processor.process(cur, acc)
        });
        println!("{:#?}", node);
    }
}
