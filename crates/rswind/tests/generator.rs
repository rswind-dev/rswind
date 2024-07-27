#[cfg(test)]
mod generator_tests {
    use std::{env::current_dir, ops::Deref};

    use rswind::{
        config::GeneratorConfig, preset::preset_tailwind, processor::ResultKind, Generator,
    };
    use serde_json::json;

    #[test]
    fn test_generator_builder() {
        let generator = Generator::builder().with_base(Some("src".to_owned())).build().unwrap();

        assert_eq!(
            generator.base().canonicalize().unwrap().to_str().unwrap(),
            current_dir().unwrap().join("src").canonicalize().unwrap().to_str().unwrap()
        );
    }

    #[test]
    fn test_generator_builder2() {
        let generator = Generator::builder()
            .with_base(Some("src".to_owned()))
            .with_preset(preset_tailwind)
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

    #[test]
    fn test_generator_with_io() {
        let mut generator = Generator::builder()
            .with_base(Some(
                current_dir().unwrap().join("tests").join("fixtures").to_string_lossy().to_string(),
            ))
            .with_preset(preset_tailwind)
            .with_watch(true)
            .build()
            .unwrap();

        let res = generator.generate_contents();
        assert_eq!(&*res.css, ".flex {\n  display: flex;\n}\n");
        assert_eq!(res.kind, ResultKind::Generated);

        let res = generator.generate_contents();
        assert_eq!(&*res.css, ".flex {\n  display: flex;\n}\n");
        assert_eq!(res.kind, ResultKind::Cached);
    }

    #[test]
    fn test_generator_without_cache() {
        let mut generator = Generator::builder()
            .with_base(Some(
                current_dir().unwrap().join("tests").join("fixtures").to_string_lossy().to_string(),
            ))
            .with_preset(preset_tailwind)
            .build()
            .unwrap();

        let res = generator.generate_contents();
        assert_eq!(&*res.css, ".flex {\n  display: flex;\n}\n");
        assert_eq!(res.kind, ResultKind::Generated);

        let res = generator.generate_contents();
        assert_eq!(&*res.css, ".flex {\n  display: flex;\n}\n");
        assert_eq!(res.kind, ResultKind::Generated);
    }
}
