use fxhash::FxHashMap as HashMap;

use crate::{
    config::ArrowConfig,
    css::{rule::RuleList, Decl, DeclList, Rule},
    process::{Utility, Variant, VariantMatchingFn},
    theme::{Theme, ThemeValue},
    themes::theme,
};

use self::utilities::{UtilityStorage, UtilityStorageImpl};

pub mod utilities;

#[derive(Default)]
pub struct Context<'c> {
    pub utilities: UtilityStorageImpl<'c>,
    pub variants: HashMap<String, Variant>,
    pub theme: Theme<'static>,
    pub cache: HashMap<String, Option<String>>,
}

impl<'c> Context<'c> {
    pub fn new(config: ArrowConfig<'static>) -> Self {
        Self {
            variants: HashMap::default(),
            utilities: UtilityStorageImpl::HashMap(Default::default()),
            theme: theme().merge(config.theme),
            cache: HashMap::default(),
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
        _key: &str,
        _func: impl VariantMatchingFn + 'static,
    ) -> &Self {
        // self.variants
        //     .insert(key.to_string(), VariantHandler::Nested(Box::new(func)));
        self
    }

    pub fn add_variant_composable(
        &mut self,
        key: &str,
        handler: fn(RuleList) -> RuleList,
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
        // typ: Option<impl TypeValidator>,
    ) -> &Self {
        for (k, v) in values {
            let theme = self
                .get_theme(key)
                .unwrap_or_else(|| panic!("Theme {} not found", &k));

            self.utilities.add(
                k,
                Utility::new(move |_, input| {
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
                .allow_values(theme),
            );
        }
        self
    }

    pub fn get_theme(&self, key: &str) -> Option<ThemeValue<'static>> {
        self.theme.get(key).cloned()
    }
}

#[macro_export]
macro_rules! add_theme_rule {
  ($ctx:expr, {
    $($theme_key:literal => {
      $($key:literal => [$($decl_key:literal),+])+
    })+
  }) => {
    $(
      $ctx.add_theme_rule($theme_key, vec![
        $(($key.to_string(), vec![$($decl_key.into()),+]),)+
      ]);
    )+
  };
}
