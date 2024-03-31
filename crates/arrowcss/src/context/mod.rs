use hashbrown::HashMap;
use std::{cell::RefCell, sync::Arc};

mod static_rules;
mod utilities;

use crate::{
    config::ArrowConfig,
    css::{decl::decl, CssDecls, CssRuleList},
    rule::Rule,
    theme::{Theme, ThemeValue},
    themes::theme,
    utils::{create_variant_fn, VariantHandler},
};

use self::{static_rules::StaticRuleStorage, utilities::UtilityStorage};

pub trait VariantMatchingFn = Fn(CssRuleList) -> Option<CssRuleList>;

#[derive(Default, Clone)]
pub struct Context<'c> {
    pub static_rules: StaticRuleStorage,
    pub utilities: UtilityStorage<'c>,

    pub variants: Arc<RefCell<HashMap<String, Box<VariantHandler>>>>,

    pub theme: Arc<RefCell<Theme<'static>>>,
    pub cache: HashMap<String, String>,
    // #[allow(dead_code)]
    // pub config: Config,
    // pub tokens: RefCell<HashMap<String, Option<CssRuleList<'c>>>>,
}

impl<'c> Context<'c> {
    pub fn new(config: ArrowConfig<'static>) -> Self {
        Self {
            // tokens: HashMap::new().into(),
            static_rules: StaticRuleStorage::new(),
            variants: Arc::new(HashMap::new().into()),
            utilities: UtilityStorage::new(),
            theme: Arc::new(RefCell::new(theme().merge(config.theme))),
            cache: HashMap::new(),
            // config: config.config,
        }
    }

    pub fn add_static<S>(&self, pair: (S, CssDecls<'static>)) -> &Self
    where
        S: Into<String>,
    {
        self.static_rules.insert(pair.0.into(), pair.1);
        self
    }

    pub fn get_static(&self, key: &str) -> Option<CssDecls<'static>> {
        self.static_rules.get(key)
    }

    pub fn add_variant<T>(&self, key: &'c str, matcher: T) -> &Self
    where
        T: IntoIterator,
        T::Item: AsRef<str>,
        T::IntoIter: ExactSizeIterator,
    {
        // let key_clone: String = key.into();
        create_variant_fn(key, matcher).map(|func| {
            self.variants
                .borrow_mut()
                .insert(key.to_string(), func.into())
        });
        self
    }

    pub fn get_theme<'b: 'c>(&self, key: &str) -> Option<ThemeValue<'b>> {
        self.theme.borrow().get(key).cloned()
    }
}

pub trait AddRule<'c> {
    fn add_rule(&self, key: &str, rule: Rule<'c>) -> &Self;
    fn add_theme_rule<'a: 'c>(
        &self,
        key: &'a str,
        values: Vec<(String, Vec<String>)>,
    ) -> &Self;
}

impl<'c> AddRule<'c> for Context<'c> {
    fn add_rule(&self, key: &str, rule: Rule<'c>) -> &Self {
        self.utilities.insert(key.into(), rule);
        self
    }

    fn add_theme_rule<'a: 'c>(
        &self,
        key: &'a str,
        values: Vec<(String, Vec<String>)>,
    ) -> &Self {
        for (k, v) in values {
            let theme = self
                .get_theme(key)
                .unwrap_or_else(|| panic!("Theme {} not found", &k));

            self.utilities.insert(
                k,
                Rule::new(move |_, input| {
                    Some(
                        v.clone()
                            .into_iter()
                            .map(|k| decl(k, input.to_string()))
                            .collect(),
                    )
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
