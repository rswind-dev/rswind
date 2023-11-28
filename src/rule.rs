use crate::css::CSSDecls;

#[macro_export]
macro_rules! static_rules {
  (
    $($key:literal => {
      $($name:literal: $value:literal;)+
    })+
  ) => {
    vec![
      $(
        ($key, CSSDecls::multi([
          $(
            ($name, $value),
          )+
        ])),
      )+
    ]
  };
}
