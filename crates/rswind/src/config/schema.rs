use std::collections::HashMap;

use either::Either;

use crate::{
    css::{rule::RuleList, DeclList},
    parsing::AdditionalCssHandler,
    process::RuleMatchingFn,
    theme::ThemeValue,
    types::TypeValidator,
};

use super::de::theme::FlattenedColors;

macro_rules! forward_impl {
    (($($impl:tt)+) => $target:ty) => {
        impl $($impl)+ {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                <$target>::schema_name()
            }

            fn schema_id() -> std::borrow::Cow<'static, str> {
                <$target>::schema_id()
            }

            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::Schema {
                <$target>::json_schema(gen)
            }

            fn _schemars_private_non_optional_json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::Schema {
                <$target>::_schemars_private_non_optional_json_schema(gen)
            }

            fn _schemars_private_is_option() -> bool {
                <$target>::_schemars_private_is_option()
            }
        }
    };
    ($ty:ty => $target:ty) => {
        forward_impl!((schemars::JsonSchema for $ty) => $target);
    };
}

forward_impl!(dyn RuleMatchingFn => HashMap<String, String>);

forward_impl!(dyn TypeValidator => String);

forward_impl!(DeclList => HashMap<String, String>);

forward_impl!(RuleList => HashMap<String, either::Either<String, DeclList>>);

forward_impl!(dyn AdditionalCssHandler => crate::css::rule::RuleList);

forward_impl!(ThemeValue => HashMap<String, String>);

forward_impl!(FlattenedColors => HashMap<String, Either<String, HashMap<String, String>>>);
