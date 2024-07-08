use instance_code::InstanceCode;
use rswind_css::{Decl, Rule, RuleModifier};
use serde::Deserialize;
use smol_str::SmolStr;

#[derive(Clone, Debug, Deserialize, InstanceCode)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FontSize {
    Plain(SmolStr),
    WithLineHeight((SmolStr, SmolStr)),
    WithConfig((SmolStr, FontSizeConfig)),
}

#[derive(Clone, Debug, Default, Deserialize, InstanceCode)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FontSizeConfig {
    pub line_height: Option<SmolStr>,
    pub letter_spacing: Option<SmolStr>,
    pub font_weight: Option<SmolStr>,
}

impl RuleModifier for &FontSizeConfig {
    fn apply(&self, mut rule: Rule) -> Rule {
        rule.decls.extend(self.line_height.as_deref().map(|v| Decl::new("line-height", v)));
        rule.decls.extend(self.letter_spacing.as_deref().map(|v| Decl::new("letter-spacing", v)));
        rule.decls.extend(self.font_weight.as_deref().map(|v| Decl::new("font-weight", v)));
        rule
    }
}

#[derive(Clone, Debug, Deserialize, InstanceCode)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum FontFamily {
    Plain(SmolStr),
    Multi(Vec<SmolStr>),
    WithConfig((SmolStr, FontFamilyConfig)),
}

#[derive(Clone, Debug, Deserialize, InstanceCode)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FontFamilyConfig {
    pub font_feature_settings: Option<SmolStr>,
    pub font_variation_settings: Option<SmolStr>,
}

impl FontFamilyConfig {
    pub fn apply(&self, mut rule: Rule) -> Rule {
        rule.decls.extend(
            self.font_feature_settings.as_deref().map(|v| Decl::new("font-feature-settings", v)),
        );
        rule.decls.extend(
            self.font_variation_settings
                .as_deref()
                .map(|v| Decl::new("font-variation-settings", v)),
        );
        rule
    }
}
