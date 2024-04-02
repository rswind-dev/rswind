pub mod rule;

#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut m = fxhash::FxHashMap::default();
            $(
                m.insert($key.to_string(), $value.into());
            )*
            m
        }
    };
}
