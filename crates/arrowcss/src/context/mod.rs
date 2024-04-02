use fxhash::FxHashMap as HashMap;
use std::{cell::RefCell, sync::Arc};

mod static_rules;
mod utilities;

use crate::{
    config::ArrowConfig,
    css::{DeclList, NodeList},
    rule::Utility,
    theme::{Theme, ThemeValue},
    themes::theme,
    utils::{create_variant_fn, VariantHandler},
};
use arrowcss_css_macro::css;

use self::{static_rules::StaticRuleStorage, utilities::UtilityStorage};

pub trait VariantMatchingFn = Fn(NodeList) -> Option<NodeList>;

#[derive(Default, Clone)]
pub struct Context<'c> {
    pub static_rules: StaticRuleStorage,
    pub utilities: UtilityStorage<'c>,

    pub variants: Arc<RefCell<HashMap<String, Box<VariantHandler>>>>,

    pub theme: Arc<RefCell<Theme<'static>>>,
    pub cache: HashMap<String, Option<String>>,
    // #[allow(dead_code)]
    // pub config: Config,
    // pub tokens: RefCell<HashMap<String, Option<CssRuleList<'c>>>>,
}

impl<'c> Context<'c> {
    pub fn new(config: ArrowConfig<'static>) -> Self {
        Self {
            // tokens: HashMap::new().into(),
            static_rules: StaticRuleStorage::new(),
            variants: Arc::new(HashMap::default().into()),
            utilities: UtilityStorage::new(),
            theme: Arc::new(RefCell::new(theme().merge(config.theme))),
            cache: HashMap::default(),
            // config: config.config,
        }
    }

    pub fn add_static<S>(&self, pair: (S, DeclList<'static>)) -> &Self
    where
        S: Into<String>,
    {
        self.static_rules.insert(pair.0.into(), pair.1);
        self
    }

    pub fn get_static(&self, key: &str) -> Option<DeclList<'static>> {
        self.static_rules.get(key)
    }

    pub fn add_variant<T>(&self, key: &'c str, matcher: T) -> &Self
    where
        T: IntoIterator,
        T::Item: AsRef<str>,
        T::IntoIter: ExactSizeIterator,
    {
        create_variant_fn(key, matcher).map(|func| {
            self.variants
                .borrow_mut()
                .insert(key.to_string(), func.into())
        });
        self
    }

    pub fn add_variant_fn<'a>(
        &self,
        key: &'a str,
        func: impl VariantMatchingFn + 'static,
    ) -> &Self {
        self.variants.borrow_mut().insert(
            key.to_string(),
            Box::new(VariantHandler::Nested(Box::new(func))),
        );
        self
    }

    pub fn get_theme(&self, key: &str) -> Option<ThemeValue<'c>> {
        self.theme.borrow().get(key).cloned()
    }
}

pub trait AddRule<'c> {
    fn add_rule(&self, key: &str, rule: Utility<'c>) -> &Self;
    fn add_theme_rule<'a: 'c>(
        &self,
        key: &'a str,
        values: Vec<(String, Vec<String>)>,
        // typ: Option<impl TypeValidator>,
    ) -> &Self;
}

impl<'c> AddRule<'c> for Context<'c> {
    fn add_rule(&self, key: &str, rule: Utility<'c>) -> &Self {
        self.utilities.insert(key.into(), rule);
        self
    }

    fn add_theme_rule<'a: 'c>(
        &self,
        key: &'a str,
        values: Vec<(String, Vec<String>)>,
        // typ: Option<impl TypeValidator>,
    ) -> &Self {
        for (k, v) in values {
            let theme = self
                .get_theme(key)
                .unwrap_or_else(|| panic!("Theme {} not found", &k));

            self.utilities.insert(
                k,
                Utility::new(move |_, input| {
                    v.clone()
                        .into_iter()
                        .map(|k| css!(k: input.to_string()))
                        .flatten()
                        .collect()
                })
                .allow_values(theme),
            );
        }
        self
    }
}

#[macro_export]
macro_rules! add_theme_rule {
  ($ctx:expr, {
    $($theme_key:literal => {
      $($key:literal => [$($decl_key:literal),+])+
    })+
  }) => {
    use $crate::context::AddRule;
    $(
      $ctx.add_theme_rule($theme_key, vec![
        $(($key.to_string(), vec![$($decl_key.into()),+]),)+
      ]);
    )+
  };
}
