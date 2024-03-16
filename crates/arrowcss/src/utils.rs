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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_modifiers() {
        assert_eq!(
            extract_variants("md:opacity-50"),
            (vec!["md".into()], "opacity-50".into())
        );
        assert_eq!(
            extract_variants("opacity-50"),
            (vec![], "opacity-50".into())
        );
        assert_eq!(
            extract_variants("md:disabled:hover:opacity-50"),
            (
                vec!["md".into(), "disabled".into(), "hover".into()],
                "opacity-50".into()
            )
        );
        assert_eq!(extract_variants(""), (vec![], "".into()));
    }
}