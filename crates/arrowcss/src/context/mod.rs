use fxhash::FxHashMap as HashMap;

mod static_rules;
mod utilities;

use crate::{
    config::ArrowConfig,
    css::{DeclList, NodeList},
    rule::UtilityProcessor,
    theme::{Theme, ThemeValue},
    themes::theme,
    utils::{create_variant_fn, VariantHandler},
};
use arrowcss_css_macro::css;

use self::{
    static_rules::StaticRuleStorage,
    utilities::{HashMapUtilityStorage, UtilityStorage},
};

pub trait VariantMatchingFn = Fn(NodeList) -> Option<NodeList> + Sync + Send;
pub trait ModifierMatchingFn = Fn(NodeList) -> Option<NodeList> + Sync + Send;

pub struct Context<'c> {
    pub static_rules: StaticRuleStorage,
    pub utilities: Box<dyn UtilityStorage<'c> + 'c>,

    pub variants: HashMap<String, VariantHandler>,

    pub theme: Theme<'static>,
    pub cache: HashMap<String, Option<String>>,
    // #[allow(dead_code)]
    // pub config: Config,
    // pub tokens: RefCell<HashMap<String, Option<CssRuleList<'c>>>>,
}

impl Default for Context<'_> {
    fn default() -> Self {
        Self::new(ArrowConfig::default())
    }
}

impl<'c> Context<'c> {
    pub fn new(config: ArrowConfig<'static>) -> Self {
        Self {
            // tokens: HashMap::new().into(),
            static_rules: StaticRuleStorage::new(),
            variants: HashMap::default(),
            utilities: Box::<HashMapUtilityStorage>::default(),
            theme: theme().merge(config.theme),
            cache: HashMap::default(),
            // config: config.config,
        }
    }

    pub fn add_static<S>(&mut self, pair: (S, DeclList<'static>)) -> &Self
    where
        S: Into<String>,
    {
        self.static_rules.insert(pair.0.into(), pair.1);
        self
    }

    pub fn get_static(&self, key: &str) -> Option<DeclList<'static>> {
        self.static_rules.get(key)
    }

    pub fn add_variant<T>(&mut self, key: &'c str, matcher: T) -> &mut Self
    where
        T: IntoIterator,
        T::Item: AsRef<str>,
        T::IntoIter: ExactSizeIterator,
    {
        create_variant_fn(key, matcher)
            .map(|func| self.variants.insert(key.to_string(), func));
        self
    }

    pub fn add_variant_fn(
        &mut self,
        key: &str,
        func: impl VariantMatchingFn + 'static,
    ) -> &Self {
        self.variants
            .insert(key.to_string(), VariantHandler::Nested(Box::new(func)));
        self
    }

    pub fn get_theme(&self, key: &str) -> Option<ThemeValue<'static>> {
        self.theme.get(key).cloned()
    }
}

pub trait AddRule<'c> {
    fn add_rule<'a: 'c>(&mut self, key: &str, rule: UtilityProcessor<'a>);
    fn add_theme_rule<'a: 'c>(
        &mut self,
        key: &'a str,
        values: Vec<(String, Vec<String>)>,
        // typ: Option<impl TypeValidator>,
    ) -> &Self;
}

impl<'c> AddRule<'c> for Context<'c> {
    fn add_rule<'a: 'c>(&mut self, key: &str, rule: UtilityProcessor<'a>) {
        self.utilities.insert(key.into(), rule);
    }

    fn add_theme_rule<'a: 'c>(
        &mut self,
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
                UtilityProcessor::new(move |_, input| {
                    v.clone()
                        .into_iter()
                        .flat_map(|k| css!(k: input.to_string()))
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
