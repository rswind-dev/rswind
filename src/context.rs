use std::{collections::HashMap, rc::Rc};

use crate::{css::CSSDecls, theme::Theme};

type RuleMatchingFn<'a> = Box<dyn Fn(&'a str) -> Option<CSSDecls>>;

pub struct Context<'a> {
  pub static_rules: HashMap<String, CSSDecls>,
  pub arbitrary_rules: HashMap<String, RuleMatchingFn<'a>>,
  pub rules: HashMap<String, RuleMatchingFn<'a>>,
  pub theme: Rc<Theme>,
  pub config: String,
}

impl<'a> Context<'a> {
  pub fn add_rule<F, S>(&mut self, key: S, func: F) -> &mut Self
  where
      F: Fn(&str, Rc<Theme>) -> Option<CSSDecls> + 'static,
      S: Into<String>,
  {
    let theme_clone = Rc::clone(&self.theme);
    self.rules.insert(key.into(), Box::new(move |input| func(input, theme_clone.clone())));
    self
  }
  pub fn add_static<S>(&mut self, key: S, decls: CSSDecls) -> &mut Self
  where
      S: Into<String>,
  {
    self.static_rules.insert(key.into(), decls);
    self
  }
}

// usage:
// add_static!(ctx, {
//   block => {
//     display: block;
//   }
//   flex => display: flex;
//   xxx => {
//     display: flex; color: red;
//   }
// });
// #[macro_export]
// macro_rules! add_static {
//   ($ctx:ident, {
//     $($key:ident => {
//       $($name:ident: $value:ident;)+
//     })+
//   }) => {
//     $(
//       $ctx.add_static(stringify!($key), CSSDecls::multi([
//         $((stringify!($name), stringify!($value)),)+
//       ]));
//     )+
//   };
// }

#[macro_export]
macro_rules! add_static {
  ($ctx:ident, {
    $($key:literal => {
      $($name:literal: $value:literal;)+
    })+
  }) => {
    $(
      $ctx.add_static($key, CSSDecls::multi([
        $(($name, $value),)+
      ]));
    )+
  };
}
