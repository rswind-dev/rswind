use std::collections::HashMap;

use rswind_common::impl_schemars;
use rswind_css::rule::RuleList;

use crate::{parse::AdditionalCssHandler, process::RuleMatchingFn, types::TypeValidator};

impl_schemars!(dyn RuleMatchingFn => HashMap<String, String>);

impl_schemars!(dyn TypeValidator => String);

impl_schemars!(dyn AdditionalCssHandler => RuleList);
