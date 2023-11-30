use std::{collections::HashMap, rc::Rc};

use crate::{css::{CSSDecls, CSSStyleRule, CSSRule}, theme::Theme};

type RuleMatchingFn<'a> = Box<dyn Fn(&'a str) -> Option<CSSDecls> + 'static>;

type VariantMatchingFn = dyn Fn(CSSRule) -> Option<CSSRule> + 'static;

pub struct Context<'a> {
  pub static_rules: HashMap<String, CSSDecls>,
  pub arbitrary_rules: HashMap<String, RuleMatchingFn<'a>>,
  pub rules: HashMap<String, RuleMatchingFn<'a>>,

  pub variants: HashMap<String, Box<VariantMatchingFn>>,

  pub theme: Rc<Theme>,
  pub config: String,
  pub tokens: HashMap<&'a str, Option<CSSStyleRule>>
}

pub struct ThemeValue<S: Into<String>> {
  pub key: S,
  pub decl_key: Vec<String>
}

impl<S: Into<String>> ThemeValue<S> {
    pub fn new(key: S, decl_key: Vec<String>) -> Self {
        Self {
            key,
            decl_key
        }
    }
}

impl<'a> Context<'a> {

  pub fn new(theme: Rc<Theme>) -> Self {
    Self {
      tokens: HashMap::new(),
      static_rules: HashMap::new(),
      arbitrary_rules: HashMap::new(),
      variants: HashMap::new(),
      rules: HashMap::new(),
      theme: Rc::clone(&theme),
      config: "config".into(),
    }
  }

  pub fn add_rule<F, S>(&mut self, key: S, func: F) -> &mut Self
  where
      F: Fn(&str, Rc<Theme>) -> Option<CSSDecls> + 'static,
      S: Into<String>,
  {
    let theme_clone = Rc::clone(&self.theme);
    self.rules.insert(key.into(), Box::new(move |input| func(input, theme_clone.clone())));
    self
  }
  pub fn add_static<S>(&mut self, pair: (S, CSSDecls)) -> &mut Self
  where
      S: Into<String>,
  {
    self.static_rules.insert(pair.0.into(), pair.1);
    self
  }
  pub fn add_theme_rule<S, T>(&mut self, _theme_key: T, values: Vec<ThemeValue<S>>) -> &mut Self
  where
      S: Into<String>,
      T: Into<String>,
  {
    for value in values {
      // TODO: use theme_key
      let theme_clone = Rc::clone(&self.theme);

      self.rules.insert(
        value.key.into(),
        Box::new(move |input| {
          theme_clone.spacing.get(input).and_then(|theme_val| {
            Some(theme_rule_handler(value.decl_key.clone(), theme_val.into()))
          })
      })
      );
    }
    self
  }
  pub fn add_variant<S, F>(&mut self, key: S, func: F) -> &mut Self
  where
      S: Into<String>,
      F: Fn(CSSRule) -> Option<CSSRule> + 'static,
  {
    self.variants.insert(key.into(), Box::new(func));
    self
  }
}


fn theme_rule_handler(decl_keys: Vec<String>, value: String) -> CSSDecls {
  CSSDecls::multi(
    decl_keys
      .into_iter()
      .map(|decl_key| (decl_key, value.to_owned()))
      .collect::<Vec<(_, _)>>()
  )
}

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
  ($ctx:ident, {
    $($theme_key:literal => {
      $($key:literal => [$($decl_key:literal),+])+
    })+
  }) => {
    $(
      $ctx.add_theme_rule($theme_key, vec![
        $(ThemeValue::new($key, vec![$($decl_key.into()),+]),)+
      ]);
    )+
  };
}