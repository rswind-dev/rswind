use std::{cell::RefCell, collections::HashMap, rc::Rc};

// use crate::rule::VariantMatchingFn;
// use crate::rule::RuleMatchingFn;
use crate::{
    css::{Container, CSSDecls},
    theme::Theme,
    utils::{create_variant_fn, Matcher},
};

pub trait RuleMatchingFn = Fn(&str) -> Option<CSSDecls> + 'static;

pub trait VariantMatchingFn = Fn(Container) -> Option<Container> + 'static;

pub struct Variant {
    pub needs_nesting: bool,
    pub handler: Rc<dyn VariantMatchingFn>,
}

impl Variant {
    pub fn plain(handler: Rc<dyn VariantMatchingFn>) -> Self {
        Self {
            needs_nesting: false,
            handler,
        }
    }
    pub fn at_rule(handler: Rc<dyn VariantMatchingFn>) -> Self {
        Self {
            needs_nesting: true,
            handler,
        }
    }
}

#[derive(Default, Clone)]
pub struct Context {
    pub static_rules: RefCell<HashMap<String, CSSDecls>>,
    pub rules: RefCell<HashMap<String, Vec<Rc<dyn RuleMatchingFn>>>>,
    pub arbitrary_rules: Rc<HashMap<String, Rc<dyn RuleMatchingFn>>>,

    pub variants: RefCell<HashMap<String, Rc<Variant>>>,

    pub theme: RefCell<Rc<Theme>>,
    pub config: String,
    pub tokens: RefCell<HashMap<String, Option<Container>>>,
}

pub struct ThemeValue<S: Into<String>> {
    pub key: S,
    pub decl_key: Vec<String>,
}

impl<S: Into<String>> ThemeValue<S> {
    pub fn new(key: S, decl_key: Vec<String>) -> Self {
        Self { key, decl_key }
    }
}

impl<'a> Context {
    pub fn new(theme: Rc<Theme>) -> Self {
        Self {
            tokens: HashMap::new().into(),
            static_rules: HashMap::new().into(),
            arbitrary_rules: HashMap::new().into(),
            variants: HashMap::new().into(),
            rules: HashMap::new().into(),
            theme: Rc::clone(&theme).into(),
            config: "config".into(),
        }
    }

    pub fn add_rule<F, S>(&mut self, key: S, func: F) -> &mut Self
    where
        F: Fn(&str, &Self) -> Option<CSSDecls> + 'static,
        S: Into<String> + ToString,
    {
        let self_clone = self.clone();
        let key_clone: String = key.to_string();
        if self.rules.borrow().contains_key(&key_clone) {
            self.rules
                .borrow_mut()
                .get_mut(&key_clone)
                .unwrap()
                .push(Rc::new(move |input| func(input, &self_clone)));
        } else {
            self.rules.borrow_mut().insert(
                key_clone,
                vec![Rc::new(move |input| func(input, &self_clone))],
            );
        }
        self
    }

    pub fn add_static<S>(&self, pair: (S, CSSDecls)) -> &Self
    where
        S: Into<String>,
    {
        self.static_rules.borrow_mut().insert(pair.0.into(), pair.1);
        self
    }

    pub fn add_theme_rule<S, T>(
        &self,
        _theme_key: T,
        values: Vec<ThemeValue<S>>,
    ) -> &Self
    where
        S: Into<String> + 'a,
        T: Into<String> + 'a,
    {
        let theme_key: String = _theme_key.into();
        for value in values {
            // TODO: use theme_key
            let theme_key = theme_key.clone();
            let theme_clone = Rc::clone(&self.theme.borrow());
            self.rules.borrow_mut().insert(
                value.key.into(),
                vec![Rc::new(move |input| {
                    theme_clone
                        .get(theme_key.as_str())
                        .and_then(|theme| theme.get(input))
                        .map(|theme_val| {
                            theme_rule_handler(
                                value.decl_key.clone(),
                                theme_val.into(),
                            )
                        })
                })],
            );
        }
        self
    }

    pub fn add_variant_fn<S, F>(&mut self, key: S, func: F) -> &Self
    where
        S: Into<String>,
        F: VariantMatchingFn,
    {
        self.variants
            .borrow_mut()
            .insert(key.into(), Variant::plain(Rc::new(func)).into());
        self
    }

    pub fn add_variant<S, M>(&self, key: S, matcher: M) -> &Self
    where
        M: Matcher<'a>,
        S: Into<String>,
    {
        let key_clone: String = key.into();
        create_variant_fn(&key_clone, matcher).map(
            |func: Rc<dyn VariantMatchingFn>| {
                self.variants.borrow_mut().insert(
                    key_clone.clone(),
                    // if matcher.starts_with('@') {
                    //     Variant::at_rule(func).into()
                    // } else {
                    Variant::plain(func).into(), // },
                )
            },
        );
        self
    }

    pub fn get_theme_value<'b>(
        &'a self,
        key: &'b str,
        value: &'b str,
    ) -> Option<String> {
        self.theme
            .borrow()
            .get(key)
            .and_then(|theme| theme.get(value))
            .map(|s| s.to_owned())
    }
}

fn theme_rule_handler(decl_keys: Vec<String>, value: String) -> CSSDecls {
    decl_keys
        .into_iter()
        .map(|decl_key| (decl_key, value.to_owned()))
        .collect::<CSSDecls>()
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
