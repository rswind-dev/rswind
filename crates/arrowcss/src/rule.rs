use std::collections::{HashMap, HashSet};

use crate::{context::RuleMatchingFn, css::CSSDecls};

#[derive(Eq, Hash, PartialEq)]
pub enum DataType {
    Length,
    Percentage,
    LengthPercentage,
    Color,
    // Any,
}

pub struct Rule {
    pub handler: Box<dyn RuleMatchingFn>,
    pub supports_negative: bool,
    pub allowed_types: HashSet<DataType>,
    pub values: Option<HashMap<String, String>>,
    pub modifiers: Option<HashMap<String, String>>,
}

impl Rule {
    pub fn new<F: RuleMatchingFn>(handler: F) -> Self {
        Self {
            handler: Box::new(handler),
            supports_negative: false,
            allowed_types: HashSet::new(),
            values: None,
            modifiers: None,
        }
    }

    pub fn support_negative(mut self) -> Self {
        self.supports_negative = true;
        self
    }

    pub fn allow_type(mut self, ty: DataType) -> Self {
        self.allowed_types.insert(ty);
        self
    }

    pub fn apply_to(&self, value: &str) -> Option<CSSDecls> {
        (self.handler)(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_builder() {
        let rule = Rule::new(|_| None)
            .support_negative()
            .allow_type(DataType::LengthPercentage);

        assert!(rule.supports_negative);
        assert!(rule.allowed_types.contains(&DataType::LengthPercentage));
    }
}
