use std::collections::HashMap;

use either::Either;
use rswind_common::impl_schemars;
use rswind_css::rule::RuleList;

use crate::{
    parsing::AdditionalCssHandler, process::RuleMatchingFn, theme::ThemeValue, types::TypeValidator,
};

use super::de::theme::FlattenedColors;

impl_schemars!(dyn RuleMatchingFn => HashMap<String, String>);

impl_schemars!(dyn TypeValidator => String);

impl_schemars!(dyn AdditionalCssHandler => RuleList);

impl_schemars!(ThemeValue => HashMap<String, String>);

impl_schemars!(FlattenedColors => HashMap<String, Either<String, HashMap<String, String>>>);
