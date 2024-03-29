use std::{cell::RefCell, collections::HashMap, sync::Arc};

mod rule;
mod static_rules;

use crate::{
    config::{ArrowConfig, Config},
    css::{decl::decl, CssDecls, CssRuleList},
    rule::Rule,
    theme::{Theme, ThemeValue},
    themes::theme,
    utils::{create_variant_fn, Matcher, VariantHandler},
};

use self::{rule::RuleStorage, static_rules::StaticRuleStorage};

pub trait VariantMatchingFn = Fn(CssRuleList) -> Option<CssRuleList>;

#[derive(Default, Clone)]
pub struct Context<'c> {
    pub static_rules: StaticRuleStorage,
    pub rules: RuleStorage<'c>,

    pub variants: Arc<RefCell<HashMap<String, Box<VariantHandler>>>>,

    pub theme: Arc<RefCell<Theme<'static>>>,
    #[allow(dead_code)]
    pub config: Config,
    pub tokens: RefCell<HashMap<String, Option<CssRuleList<'c>>>>,
}

impl<'c> Context<'c> {
    pub fn new(config: ArrowConfig<'static>) -> Self {
        Self {
            tokens: HashMap::new().into(),
            static_rules: StaticRuleStorage::new(),
            variants: Arc::new(HashMap::new().into()),
            rules: RuleStorage::new(),
            theme: Arc::new(RefCell::new(theme().merge(config.theme))),
            config: config.config,
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

    // pub fn add_theme_rule(
    //     &self,
    //     _theme_key: String,
    //     values: Vec<(String, Vec<String>)>,
    // ) -> &Self {
    //     let theme = self.get_theme(&_theme_key).clone();
    //     let mut rules = self.rules.borrow_mut();
    //     for value in values {
    //         let theme_clone = Arc::clone(&theme); // Clone theme inside the loop
    //         rules.insert(
    //             value.0.into(),
    //             vec![Arc::new(Rule::new(move |_, input| {
    //                 let theme_clone = Arc::clone(&theme_clone); // Clone theme inside the closure
    //                 theme_clone
    //                     .as_ref()
    //                     .and_then(|theme| theme.get::<str>(&input))
    //                     .map(|theme_val| {
    //                         theme_rule_handler(
    //                             value.1.clone(),
    //                             theme_val.to_string(),
    //                         )
    //                     })
    //             }))],
    //         );
    //     }
    //     self
    // }

    pub fn add_variant<M: Matcher<'c>>(
        &self,
        key: &'c str,
        matcher: M,
    ) -> &Self {
        // let key_clone: String = key.into();
        create_variant_fn(key, matcher).map(|func| {
            self.variants
                .borrow_mut()
                .insert(key.to_string(), func.into())
        });
        self
    }

    // pub fn get_theme_value(
    //     &self,
    //     key: &str,
    //     value: &str,
    // ) -> Option<CowArcStr<'c>> {
    //     self.theme
    //         .borrow()
    //         .get(key)
    //         .and_then(|theme| theme.get(value))
    //         .map(|s| s.to_owned())
    // }

    pub fn get_theme<'a, 'b: 'c>(
        &self,
        key: &'a str,
    ) -> Option<ThemeValue<'b>> {
        self.theme.borrow().get(key).map(Clone::clone)
    }

    // pub fn try_apply(&self, key: &str, value: &'c str) -> Option<CssDecls<'c>> {
    //     for func in self.rules.borrow().get(key)? {
    //         if let Some(d) = func.apply_to(value) {
    //             return Some(d);
    //         }
    //     }
    //     None
    // }
}

pub trait AddRule<'c> {
    fn add_rule(self, key: &str, rule: Rule<'c>) -> Self;
    fn add_theme_rule<'a: 'c>(
        self,
        key: &'a str,
        values: Vec<(String, Vec<String>)>,
    ) -> Self;
}

impl<'c> AddRule<'c> for Arc<Context<'c>> {
    fn add_rule(self, key: &str, rule: Rule<'c>) -> Self {
        self.rules.insert(key.into(), rule);
        self
    }

    fn add_theme_rule<'a: 'c>(
        self,
        key: &'a str,
        values: Vec<(String, Vec<String>)>,
    ) -> Self {
        for (k, v) in values {
            let theme = self
                .get_theme(&key)
                .expect(&format!("Theme {} not found", k));
            let rule = Rule::new(move |_, input| {
                let value = v.clone();
                Some(CssDecls::multi(
                    value
                        .into_iter()
                        .map(|decl_key| decl(decl_key, input.to_string())),
                ))
            })
            .allow_values(theme);
            self.rules.insert(k.clone(), rule);
        }
        self
    }
}

// fn theme_rule_handler<'a>(
//     decl_keys: Vec<String>,
//     value: String,
// ) -> CssDecls<'a> {
//     decl_keys
//         .into_iter()
//         .map(|decl_key| (decl_key, value.to_owned()))
//         .collect::<CssDecls>()
// }

#[macro_export]
macro_rules! add_static {
  ($ctx:ident, {
    $($key:literal => {
      $($name:literal: $value:literal;)+
    })+
  }) => {
    $(
      $ctx.add_static(($key, CSSDecls::multi([
        $(($name, $value),)+
      ])));
    )+
  };
}

#[macro_export]
macro_rules! add_theme_rule {
  ($ctx:expr, {
    $($theme_key:literal => {
      $($key:literal => [$($decl_key:literal),+])+
    })+
  }) => {
    use crate::context::AddRule;
    $(
      $ctx.add_theme_rule($theme_key, vec![
        $(($key.to_string(), vec![$($decl_key.into()),+]),)+
      ]);
    )+
  };
}
