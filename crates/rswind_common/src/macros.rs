#[macro_export]
macro_rules! impl_schemars {
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
        rswind_common::impl_schemars!((schemars::JsonSchema for $ty) => $target);
    };
}
