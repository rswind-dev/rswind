pub mod rule;

#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut m = hashbrown::HashMap::new();
            $(
                m.insert($key.to_string(), $value.into());
            )*
            m
        }
    };
}
