pub mod rule;

#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key.to_string(), $value.to_string());
            )*
            m
        }
    };
}