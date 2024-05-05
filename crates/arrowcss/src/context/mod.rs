use std::collections::BTreeSet;

use fxhash::FxHashMap as HashMap;
use smol_str::SmolStr;

use self::utilities::{UtilityStorage, UtilityStorageImpl};
use crate::{
    css::{rule::RuleList, Decl, DeclList, Rule},
    ordering::OrderingKey,
    parsing::VariantCandidate,
    process::{Utility, Variant},
    theme::{Theme, ThemeValue},
    themes::theme,
    types::TypeValidator,
};

pub mod utilities;

#[derive(Default)]
pub struct Context {
    pub utilities: UtilityStorageImpl,
    pub variants: HashMap<SmolStr, Variant>,
    pub theme: Theme,
    pub cache: HashMap<SmolStr, Option<String>>,
    pub seen_variants: BTreeSet<Variant>,
}

impl Context {
    pub fn new(t: Theme) -> Self {
        Self {
            variants: HashMap::default(),
            utilities: UtilityStorageImpl::HashMap(Default::default()),
            theme: theme().merge(t),
            cache: HashMap::default(),
            seen_variants: BTreeSet::new(),
        }
    }

    pub fn add_static(&mut self, pair: (&str, DeclList)) -> &Self {
        self.utilities.add_static(pair.0.into(), pair.1);
        self
    }

    pub fn add_variant<'a, T>(&mut self, key: &str, matcher: T) -> &mut Self
    where
        T: IntoIterator,
        T::Item: Into<SmolStr>,
        T::IntoIter: ExactSizeIterator,
    {
        self.variants
            .insert(key.into(), Variant::new_static(matcher));
        self
    }

    pub fn add_variant_fn(
        &mut self,
        key: &str,
        func: fn(RuleList, VariantCandidate) -> RuleList,
    ) -> &Self {
        self.variants.insert(key.into(), Variant::new_dynamic(func));
        self
    }

    pub fn add_variant_composable(
        &mut self,
        key: &str,
        handler: fn(RuleList, VariantCandidate) -> RuleList,
    ) -> &mut Self {
        self.variants
            .insert(key.into(), Variant::new_composable(handler));
        self
    }

    pub fn add_utility(&mut self, key: &str, utility: Utility) {
        self.utilities.add(key.into(), utility);
    }

    pub fn add_theme_utility(
        &mut self,
        key: &str,
        values: Vec<(SmolStr, Vec<SmolStr>)>,
        ord: Option<OrderingKey>,
        typ: Option<impl TypeValidator + 'static + Clone>,
    ) -> &Self {
        for (k, v) in values {
            let theme = self
                .get_theme(key)
                .unwrap_or_else(|| panic!("Theme {} not found", &k));

            let mut utility = Utility::new(move |_meta, input| {
                Rule::new_with_decls(
                    "&",
                    v.clone()
                        .into_iter()
                        .map(|k| Decl::new(k, input.clone()))
                        .collect(),
                )
            })
            .allow_values(theme);
            utility.ordering_key = ord;
            utility.validator = typ.clone().map(|v| Box::new(v) as _);

            self.utilities.add(k, utility);
        }
        self
    }

    pub fn get_theme(&self, key: &str) -> Option<ThemeValue> {
        self.theme.get(key).cloned()
    }
}

#[macro_export]
macro_rules! get_ord(
    ($ord:expr) => {
        Some($ord)
    };
    () => {
        None
    };
);

#[macro_export]
macro_rules! get_typ(
    ($typ:expr) => {
        Some($typ)
    };
    () => {
        None::<CssDataType>
    };
);

#[macro_export]
macro_rules! get_bool(
    ($bool:literal) => {
        true
    };
    () => {
        false
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
                    .ordering($crate::get_ord!($($ord)?))
                    .validator($crate::get_typ!($($typ)?))
                    $(.support_negative($crate::get_bool!($($negative)?)))?
                    $(.support_fraction($crate::get_bool!($($fraction)?)))?
                    ;

                $ctx.utilities.add($key.into(), utility.apply_options(options));
            )*
        )+
    };
}
