use instance_code::inject_instance;
use rswind_core::codegen::UtilityInput;
use rswind_core::css::css;
use rswind_core::process::RawValueDef;
use rswind_core::theme::values::{FontFamily, FontSize};
use rswind_core::theme::ThemeValue;
use rswind_core::types::CssProperty;
use rswind_core::{config::StaticUtilityConfig, parse::UtilityBuilder};
use rswind_css::Rule;
use smol_str::{format_smolstr, SmolStr};
use std::sync::Arc;

use rswind_core::design::DesignSystem;

#[allow(clippy::disallowed_types)]
pub fn load_static_utilities(design: &mut DesignSystem) {
    let static_utilities: StaticUtilityConfig = inject_instance!("static_utilities");

    design.utilities.extend(static_utilities.0);
}

#[allow(clippy::disallowed_types)]
pub fn load_dynamic_utilities(design: &mut DesignSystem) {
    let utilities: UtilityInput = inject_instance!("utilities");

    design.utilities.extend(
        utilities
            .utilities
            .into_iter()
            .map(|u| u.parse(&design.theme).unwrap_or_else(|e| panic!("{e}"))),
    );

    let keyframes = design.get_theme("keyframes").unwrap_or_default();

    design.utilities.extend(
        [
            // font-size
            UtilityBuilder::new("text", move |meta, value| match meta.theme_value {
                ThemeValue::FontSize(FontSize::Plain(value)) => {
                    css!("font-size": value.as_str())
                }
                ThemeValue::FontSize(FontSize::WithLineHeight((size, line_height))) => {
                    css! {
                        "font-size": size.as_str();
                        "line-height": line_height.as_str();
                    }
                }
                ThemeValue::FontSize(FontSize::WithConfig((value, config))) => {
                    css!("font-size": value.as_str()).apply(config)
                }
                _ => css!("font-size": value),
            })
            .with_theme("fontSize")
            .with_validator(CssProperty::FontSize)
            .with_modifier(RawValueDef::new("lineHeight").with_validator(CssProperty::LineHeight)),
            // font-family
            UtilityBuilder::new("font", |meta, value| match meta.theme_value {
                ThemeValue::FontFamily(font_family) => match font_family {
                    FontFamily::Plain(value) => css!("font-family": value.as_str()),
                    FontFamily::Multi(value) => css!("font-family": value.join(", ")),
                    FontFamily::WithConfig((value, config)) => {
                        config.apply(css!("font-family": value.as_str()))
                    }
                },
                _ => css!("font-family": value),
            })
            .with_theme("fontFamily")
            .with_validator(CssProperty::FontWeight),
            // animation
            UtilityBuilder::new("animate", |_, value| css!("animation": value))
                .with_theme("animation")
                .with_additional_css(move |value: SmolStr| {
                    keyframes.get_rule_list(&value).cloned().map(|f| {
                        Rule::new_with_rules(format_smolstr!("@keyframes {}", value), f)
                            .to_rule_list()
                    })
                }),
        ]
        .into_iter()
        .map(|u| u.parse(&design.theme).unwrap_or_else(|e| panic!("{e}"))),
    );
}
