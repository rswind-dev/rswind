macro_rules! get_ord(
    ($ord:expr) => {
        Some($ord)
    };
    () => {
        None
    };
);

macro_rules! get_bool(
    ($bool:literal) => {
        true
    };
    () => {
        false
    };
);

macro_rules! get_typ(
    ($inner_typ:expr) => {
        Some($inner_typ)
    };
    () => {
        None::<CssDataType>
    };
);

#[macro_export]
macro_rules! add_theme_utility {
    ($ctx:expr, {
        $($theme_key:literal => {
            $( $key:literal $(: $typ:expr)? => [$($decl_key:literal),+] $(in $ord:expr)? $(, $( negative: $negative: literal )? $( fraction: $fraction: literal )? )?  )*
        }),+
    }) => {
        use $crate::context::utilities::UtilityStorage;

        $(
            $(
                let theme = $ctx
                    .get_theme($theme_key)
                    .unwrap_or_else(|| panic!("Theme {} not found", &$key));

                let utility = $crate::process::Utility::new(move |_meta, input| {
                    $crate::css::Rule::new_with_decls(
                        "&",
                        [$($decl_key),+].clone()
                            .into_iter()
                            .map(|k| $crate::css::Decl::new(k, input.clone()))
                            .collect(),
                    )
                })
                .allow_values(theme);

                let mut options = $crate::process::UtilityOptions::new();
                options
                    .ordering(get_ord!($($ord)?))
                    .validator(get_typ!($($typ)?))
                    $(.support_negative(get_bool!($($negative)?)))?
                    $(.support_fraction(get_bool!($($fraction)?)))?
                    ;

                $ctx.utilities.add($key.into(), utility.apply_options(options));
            )*
        )+
    };
}
