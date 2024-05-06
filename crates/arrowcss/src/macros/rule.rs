#[macro_export]
macro_rules! static_rules {
  (
    $($key:literal => {
      $($name:literal: $value:literal;)+
    })+
  ) => {
    [
      $(
        (smol_str::SmolStr::new_static($key), DeclList::multi([
          $(
            (
              smol_str::SmolStr::new_static($name),
              smol_str::SmolStr::new_static($value)
            ),
          )+
        ])),
      )+
    ]
  };
}
