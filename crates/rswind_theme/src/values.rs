use instance_code::InstanceCode;
use rswind_css::{Decl, Rule};
use serde::Deserialize;
use smol_str::SmolStr;

#[derive(Clone, Debug, Deserialize, InstanceCode)]
#[serde(untagged)]
pub enum FontSize {
    Plain(SmolStr),
    WithLineHeight((SmolStr, SmolStr)),
    WithConfig((SmolStr, FontSizeConfig)),
}

#[derive(Clone, Debug, Default, Deserialize, InstanceCode)]
#[serde(rename_all = "camelCase")]
pub struct FontSizeConfig {
    pub line_height: Option<SmolStr>,
    pub letter_spacing: Option<SmolStr>,
    pub font_weight: Option<SmolStr>,
}

impl FontSizeConfig {
    pub fn apply(&self, mut rule: Rule) -> Rule {
        if let Some(v) = self.line_height.as_deref() {
            rule.decls.push(Decl::new("line-height", v));
        }
        if let Some(v) = self.letter_spacing.as_deref() {
            rule.decls.push(Decl::new("letter-spacing", v));
        }
        if let Some(v) = self.font_weight.as_deref() {
            rule.decls.push(Decl::new("font-weight", v));
        }
        rule
    }
}

#[derive(Clone, Debug, Deserialize, InstanceCode)]
#[serde(untagged)]
pub enum FontFamily {
    Plain(SmolStr),
    Multi(Vec<SmolStr>),
    WithConfig((SmolStr, FontFamilyConfig)),
}

#[derive(Clone, Debug, Deserialize, InstanceCode)]
#[serde(rename_all = "camelCase")]
pub struct FontFamilyConfig {
    pub font_feature_settings: Option<SmolStr>,
    pub font_variation_settings: Option<SmolStr>,
}

impl FontFamilyConfig {
    pub fn apply(&self, mut rule: Rule) -> Rule {
        if let Some(v) = self.font_feature_settings.as_deref() {
            rule.decls.push(Decl::new("font-feature-settings", v));
        }
        if let Some(v) = self.font_variation_settings.as_deref() {
            rule.decls.push(Decl::new("font-variation-settings", v));
        }
        rule
    }
}

// #[derive(Clone, Debug, Deserialize, InstanceCode)]
// #[serde(untagged)]
// pub enum ScreenValue {
//     Raw { raw: SmolStr },
//     MinMax { max: Option<SmolStr>, min: Option<SmolStr> },
// }

// impl ScreenValue {
//     pub fn normalize(&self) -> SmolStr {
//         match self {
//             ScreenValue::Raw { raw } => raw.clone(),
//             ScreenValue::MinMax { min, max } => match (min, max) {
//                 (Some(min), None) => format_smolstr!("@media (min-width: {min})"),
//                 (None, Some(max)) => format_smolstr!("@media (max-width: {max})"),
//                 (Some(min), Some(max)) => {
//                     format_smolstr!("@media (min-width: {min}) and (max-width: {max})")
//                 }
//                 _ => SmolStr::default(),
//             },
//         }
//     }
// }

// #[derive(Clone, Debug, Deserialize, InstanceCode)]
// #[serde(untagged)]
// pub enum ScreensConfigValue {
//     Plain(SmolStr),
//     Screen(ScreenValue),
//     Screens(Vec<ScreenValue>),
// }

// impl ScreensConfigValue {
//     pub fn normalize(&self) -> SmolStr {
//         match self {
//             ScreensConfigValue::Plain(screen) => {
//                 format_smolstr!("@media (width >= {screen})")
//             }
//             ScreensConfigValue::Screen(screen) => screen.normalize(),
//             ScreensConfigValue::Screens(screens) => {
//                 screens.iter().fold(SmolStr::default(), |acc, screen| {
//                     format_smolstr!("{acc} {}", screen.normalize())
//                 })
//             }
//         }
//     }
// }

// #[derive(Clone, Debug, Deserialize, InstanceCode)]
// #[serde(untagged)]
// pub enum Screens {
//     List(Vec<SmolStr>),
//     Map(FxHashMap<SmolStr, ScreensConfigValue>),
// }

// pub struct NormalizedScreens {
//     pub raw: SmolStr,
//     pub key: SmolStr,
//     pub value: SmolStr,
// }

// impl Screens {
//     pub fn normalize(&self) -> Vec<NormalizedScreens> {
//         match self {
//             Screens::Map(map) => map
//                 .iter()
//                 .map(|(key, value)| NormalizedScreens {
//                     key: key.clone(),
//                     value: value.normalize(),
//                     raw: SmolStr::from("0px"),
//                 })
//                 .filter(|s| !s.value.is_empty())
//                 .collect(),
//             _ => {
//                 unreachable!()
//             }
//         }
//     }
// }

// #[derive(Clone, Debug, Deserialize, InstanceCode)]
// pub struct Container {
//     pub screens: Option<Screens>,
//     pub center: Option<bool>,
//     pub padding: Option<ContainerPadding>,
// }

// #[derive(Clone, Debug, Deserialize, InstanceCode)]
// #[serde(untagged)]
// pub enum ContainerPadding {
//     Plain(SmolStr),
//     Screen(FxHashMap<SmolStr, SmolStr>),
// }

// #[derive(Clone, Debug, Deserialize, InstanceCode)]
// #[serde(untagged)]
// pub enum DropShadow {
//     Plain(SmolStr),
//     List(Vec<SmolStr>),
// }
