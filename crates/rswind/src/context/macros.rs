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
        Some(Box::new($inner_typ))
    };
    () => {
        None::<CssDataType>
    };
);

#[macro_export]
macro_rules! add_theme_utility {
    ($design:expr, {
        $($theme_key:literal => {
            $( $key:literal $(: $typ:expr)? => [$($decl_key:literal),+] $(in $ord:expr)? $(, $( negative: $negative: literal )? $( fraction: $fraction: literal )? )?  )*
        }),+
    }) => {
        $(
            $(
                let theme = $design
                    .get_theme($theme_key)
                    .unwrap_or_else(|| panic!("Theme {} not found", &$key));

                #[allow(unused_mut)]
                let mut utility = $crate::process::Utility::new(move |_meta, input| {
                    $crate::css::Rule::new_with_decls(
                        "&",
                        [$($decl_key),+].clone()
                            .into_iter()
                            .map(|k| $crate::css::Decl::new(k, input.clone()))
                            .collect(),
                    )
                });

                utility.value_repr.allowed_values = Some(theme);

                $(utility.supports_negative = get_bool!($($negative)?);)?

                $(utility.supports_fraction = get_bool!($($fraction)?);)?

                $(utility.ordering_key = get_ord!($ord);)?

                $(utility.value_repr.validator = get_typ!($typ);)?

                $design.utilities.add($key.into(), utility);
            )*
        )+
    };
}
