use std::fs::read_to_string;

use instance_code::InstanceCode;
use rswind::{
    codegen::{StaticUtilityConfig, UtilityInput},
    theme::codegen::ThemeCodegen,
};

#[test]
fn test_theme() {
    let file = read_to_string("preset/tailwind-theme.toml").expect("file not found");
    let theme: ThemeCodegen = toml::from_str(&file).unwrap();

    assert!(theme.0.colors.as_ref().unwrap().get("gray-100").is_some());
    assert!(theme.0.colors.as_ref().unwrap().get("white").is_some());

    assert!(theme.0.font_family.as_ref().unwrap().get("sans").is_some());
    assert!(theme.0.font_size.as_ref().unwrap().get("lg").is_some());
    assert!(theme.0.keyframes.as_ref().unwrap().get("spin").is_some());

    theme.instance_code();
}

#[test]
fn test_static_utilities() {
    let file = read_to_string("preset/static-utilities.toml").expect("file not found");
    let utilities: StaticUtilityConfig = toml::from_str(&file).unwrap();

    assert!(utilities.0.get("flex").is_some());

    utilities.instance_code();
}

#[test]
fn test_utilities() {
    let file = read_to_string("preset/utilities.yaml").expect("file not found");
    let utilities: UtilityInput = serde_yaml::from_str(&file).unwrap();

    assert!(utilities.utilities.iter().find(|u| u.key == "text").is_some());

    utilities.instance_code();
}
