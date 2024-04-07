use std::fmt::Write;

use crate::{
    context::Context,
    css::{AstNode, NodeList},
    utility::UtilityParser,
    utils::TopLevelPattern,
    variant::VariantParser,
};

use cssparser::serialize_identifier;

use lazy_static::lazy_static;
use regex::Regex;
use smallvec::SmallVec;

lazy_static! {
    pub static ref EXTRACT_RE: Regex = Regex::new(r#"[\s"';{}`]+"#).unwrap();
}

pub fn to_css_rule<'c>(value: &str, ctx: &Context<'c>) -> Option<NodeList<'c>> {
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

    Some(vec![AstNode::rule(&w, node?)])
}

#[cfg(test)]
mod tests {
    use arrowcss_css_macro::css;

    use crate::{context::AddRule, rule::UtilityProcessor};

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
        println!("{:#?}", u);

        let variants = parts.rev().collect::<SmallVec<[_; 2]>>();
        let vs = variants
            .into_iter()
            .map(|v| VariantParser::new(v).parse(&ctx))
            .collect::<Vec<_>>();
        println!("{:#?}", vs);

        let node = ctx.utilities.try_apply(u);

        println!("{:#?}", node);
    }
}
