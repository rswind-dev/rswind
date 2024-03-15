pub fn strip_arbitrary(value: &str) -> Option<&str> {
    value.strip_prefix('[').and_then(|r| r.strip_suffix(']'))
}

pub trait StripArbitrary {
    fn strip_arbitrary(&self) -> Option<&str>;
}

impl StripArbitrary for str {
    fn strip_arbitrary(&self) -> Option<&str> {
        strip_arbitrary(self)
    }
}

pub fn extract_variants(value: &str) -> (Vec<String>, String) {
    // Step 1(todo): split the rules by `:`, get [...modifier, rule]
    let mut modifiers =
        value.split(':').map(String::from).collect::<Vec<String>>();

    let value = modifiers.pop().unwrap();

    (modifiers, value)
}