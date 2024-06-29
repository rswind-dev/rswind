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
        $($theme_key:expr => {
            $( $key:literal $(: $typ:expr)? => [$($decl_key:literal),+] $(in $ord:expr)? $(, $( negative: $negative: literal )? $( fraction: $fraction: literal )? )?  )*
        }),+
    }) => {
        $(
            $(
                let theme = $crate::parsing::ThemeKey::from($theme_key)
                    .parse(& $design.theme)
                    .unwrap_or_else(|_| panic!("{}", $key));

                #[allow(unused_mut)]
                let mut utility = $crate::process::Utility::new(move |_meta, input| {
                    Rule::new_with_decls(
                        "&",
                        [$($decl_key),+].clone()
                            .into_iter()
                            .map(|k| rswind_css::Decl::new(k, input.clone()))
                            .collect(),
                    )
                });

                utility.value_def.allowed_values = Some(theme);

                $(utility.supports_negative = get_bool!($($negative)?);)?

                $(utility.supports_fraction = get_bool!($($fraction)?);)?

                $(utility.ordering_key = get_ord!($ord);)?

                $(utility.value_def.validator = get_typ!($typ);)?

                $design.utilities.add($key.into(), utility);
            )*
        )+
    };
}
