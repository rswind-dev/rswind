use std::{
    fmt::Write,
    hash::{Hash, Hasher},
};

use cssparser::serialize_identifier;
use rustc_hash::FxHasher;
use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::{
    context::{utilities::UtilityStorage, Context},
    css::rule::RuleList,
    ordering::OrderingKey,
    parsing::{UtilityParser, VariantParser},
    process::UtilityGroup,
    utils::TopLevelPattern,
};

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
