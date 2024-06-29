#[cfg(test)]
mod generator_tests {
    use std::ops::Deref;

    use rswind::{config::GeneratorConfig, preset::preset_tailwind, Generator};
    use serde_json::json;

    #[test]
    fn test_generator_builder() {
        let generator = Generator::builder().with_base(Some("src".to_owned())).build().unwrap();

        assert_eq!(generator.base().to_str().unwrap(), "src");
    }

    #[test]
    fn test_generator_builder2() {
        let generator = Generator::builder()
            .with_base(Some("src".to_owned()))
            .with_config(
                GeneratorConfig::from_value(json!({
                    "theme": {
                        "extend": {
                            "colors": {
                                "primary": "#3490dc",
                            }
                        }
                    }
                }))
                .unwrap(),
            )
            .build()
            .unwrap();
        assert_eq!(generator.theme().get_value("colors", "primary").as_deref(), Some("#3490dc"));

        assert_eq!(generator.theme().get_value("colors", "red-500").as_deref(), Some("#ef4444"));
    }

    #[test]
    fn test_generator_builder_with_preset() {
        let generator = Generator::builder()
            .with_base(Some("src".to_owned()))
            .with_preset(preset_tailwind)
            .with_config(
                GeneratorConfig::from_value(json!({
                    "theme": {
                        "spacing": {
                            "1": "0.25rem",
                        },
                        "keyframes": {},
                        "extend": {
                            "colors": {
                                "primary": "#3490dc",
                            }
                        }
                    }
                }))
                .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(generator.theme().get_value("colors", "primary").as_deref(), Some("#3490dc"));
        assert_eq!(generator.theme().get_value("colors", "red-500").as_deref(), Some("#ef4444"));

        assert_eq!(generator.theme().get_value("spacing", "1").as_deref(), Some("0.25rem"));
        assert_eq!(generator.theme().get("spacing").unwrap().deref().len(), 1);
        assert!(generator.theme().get("keyframes").unwrap().deref().is_empty());
    }
}
