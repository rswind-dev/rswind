use std::collections::BTreeSet;

use fxhash::FxHashMap as HashMap;

use self::utilities::{UtilityStorage, UtilityStorageImpl};
use crate::{
    config::ArrowConfig,
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
pub struct Context<'c> {
    pub utilities: UtilityStorageImpl<'c>,
    pub variants: HashMap<String, Variant>,
    pub theme: Theme<'static>,
    pub cache: HashMap<String, Option<String>>,
    pub seen_variants: BTreeSet<Variant>,
}

impl<'c> Context<'c> {
    pub fn new(config: ArrowConfig<'static>) -> Self {
        Self {
            variants: HashMap::default(),
            utilities: UtilityStorageImpl::HashMap(Default::default()),
            theme: theme().merge(config.theme),
            cache: HashMap::default(),
            seen_variants: BTreeSet::new(),
        }
    }

    pub fn add_static<S>(&mut self, pair: (S, DeclList<'static>)) -> &Self
    where
        S: Into<String>,
    {
        self.utilities.add_static(pair.0.into(), pair.1);
        self
    }

    pub fn add_variant<T>(&mut self, key: &str, matcher: T) -> &mut Self
    where
        T: IntoIterator,
        T::Item: Into<String>,
        T::IntoIter: ExactSizeIterator,
    {
        self.variants
            .insert(key.to_string(), Variant::new_static(matcher));
        self
    }

    pub fn add_variant_fn(
        &mut self,
        key: &str,
        func: for<'a> fn(RuleList<'a>, VariantCandidate) -> RuleList<'a>,
    ) -> &Self {
        self.variants
            .insert(key.to_string(), Variant::new_dynamic(func));
        self
    }

    pub fn add_variant_composable(
        &mut self,
        key: &str,
        handler: for<'a> fn(RuleList<'a>, VariantCandidate) -> RuleList<'a>,
    ) -> &mut Self {
        self.variants
            .insert(key.to_string(), Variant::new_composable(handler));
        self
    }

    pub fn add_utility<'a: 'c>(&mut self, key: &str, utility: Utility<'a>) {
        self.utilities.add(key.into(), utility);
    }

    pub fn add_theme_rule<'a: 'c>(
        &mut self,
        key: &'a str,
        values: Vec<(String, Vec<String>)>,
        ord: Option<OrderingKey>,
        typ: Option<impl TypeValidator + 'static + Clone>,
    ) -> &Self {
        for (k, v) in values {
            let theme = self
                .get_theme(key)
                .unwrap_or_else(|| panic!("Theme {} not found", &k));

            let mut utility = Utility::new(move |_, input| {
                let decls = v
                    .clone()
                    .into_iter()
                    .map(|k| Decl {
                        name: k.into(),
                        value: input.clone(),
                    })
                    .collect();

                Rule {
                    selector: "&".into(),
                    decls,
                    rules: vec![].into(),
                }
            })
            .allow_values(theme);

            if let Some(ord) = ord {
                utility.ordering(ord);
            }
            if let Some(ref ty) = typ {
                utility.validator(ty.clone());
            }

            self.utilities.add(k, utility);
        }
        self
    }

    pub fn get_theme(&self, key: &str) -> Option<ThemeValue<'static>> {
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
macro_rules! add_theme_rule {
    ($ctx:expr, {
        $($theme_key:literal => {
            $( $key:literal $(: $typ:expr)? => [$($decl_key:literal),+] $(in $ord:expr)? )*
        }),+
    }) => {
        $(
            $(
                $ctx.add_theme_rule(
                    $theme_key,
                    vec![($key.to_string(), vec![$($decl_key.into()),+])],
                    $crate::get_ord!($($ord)?),
                    $crate::get_typ!($($typ)?),
                );
            )*
        )+
    };
}
