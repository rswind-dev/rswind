
macro_rules! forward_impl {
    (($($impl:tt)+) => $target:ty) => {
        impl $($impl)+ {
            fn is_referenceable() -> bool {
                <$target>::is_referenceable()
            }

            fn schema_name() -> String {
                <$target>::schema_name()
            }

            fn schema_id() -> std::borrow::Cow<'static, str> {
                <$target>::schema_id()
            }

            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                <$target>::json_schema(gen)
            }

            fn _schemars_private_non_optional_json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
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

forward_impl!(crate::theme::ThemeValue => std::collections::HashMap<String, String>);
