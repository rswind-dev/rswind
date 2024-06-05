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

forward_impl!(Box<dyn crate::process::RuleMatchingFn> => std::collections::HashMap<String, String>);

forward_impl!(Box<dyn crate::types::TypeValidator> => String);

forward_impl!(crate::css::decl::DeclList => std::collections::HashMap<String, String>);

forward_impl!(crate::css::rule::RuleList => std::collections::HashMap<String, either::Either<String, crate::css::DeclList>>);

forward_impl!(Box<dyn crate::parsing::AdditionalCssHandler> => crate::css::rule::RuleList);

forward_impl!(crate::theme::ThemeValue => std::collections::HashMap<String, String>);
