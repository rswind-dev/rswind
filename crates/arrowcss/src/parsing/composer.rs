use smallvec::{smallvec, SmallVec};

use crate::css::rule::RuleList;
use crate::process::variant::VariantHandlerExt;
use crate::process::{ComposableHandler, Variant};

use super::VariantCandidate;

#[derive(Debug, Clone)]
pub struct Composer {
    layers: SmallVec<[ComposableHandler; 2]>,
    variant: Variant,
}

impl Composer {
    pub fn new(handler: ComposableHandler, variant: Variant) -> Self {
        Self {
            layers: smallvec![handler],
            variant,
        }
    }

    pub fn new_with_layers(
        layers: SmallVec<[ComposableHandler; 2]>,
        variant: Variant,
    ) -> Self {
        Self { layers, variant }
    }

    pub fn layer(&mut self, handler: ComposableHandler) -> &mut Self {
        self.layers.push(handler);
        self
    }

    pub fn handle<'a, 'b>(
        &self,
        candidate: VariantCandidate<'b>,
        rule: RuleList<'a>,
    ) -> RuleList<'a> {
        let rule = self.variant.handle(candidate.clone(), rule);
        self.layers.iter().rev().fold(rule, |rule, handler| {
            handler.handle(candidate.clone(), rule)
        })
    }
}

impl VariantHandlerExt for Composer {
    fn handle<'a, 'b>(
        &self,
        candidate: VariantCandidate<'b>,
        rule: RuleList<'a>,
    ) -> RuleList<'a> {
        self.handle(candidate, rule)
    }
}

#[cfg(test)]
mod tests {
    use either::Either::Left;

    use crate::{
        common::MaybeArbitrary,
        css::{Decl, Rule},
        process::Variant,
    };

    use super::*;

    #[test]
    fn test_compose() {
        let mut composer = ComposableHandler::new(|rule, _| {
            rule.modify_with(|s| format!("&:has({})", s.replace('&', "*")))
        })
        .compose(Variant::new_static(["&:hover"]));

        composer.layer(ComposableHandler::new(|rule, _| {
            rule.modify_with(|s| format!("&:not({})", s.replace('&', "*")))
        }));

        let rule =
            Rule::new_with_decls("&", vec![Decl::new("display", "flex")])
                .to_rule_list();

        let candidate = VariantCandidate {
            key: "has",
            value: Some(MaybeArbitrary::Named("hover")),
            modifier: None,
            arbitrary: false,
            compose: None,
            processor: Left(Variant::new_static(["&:hover"])),
        };

        let rule = composer.handle(candidate, rule);

        println!("{:#?}", rule)
    }
}
