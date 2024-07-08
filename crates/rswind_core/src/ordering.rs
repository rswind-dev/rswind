use std::{hash::Hash, str::FromStr};

use serde::Deserialize;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "build", derive(instance_code::InstanceCode), instance(path = rswind_core::ordering))]
pub enum OrderingKey {
    Translate,
    TranslateAxis,
    Scale,
    ScaleAxis,
    Rotate,
    RotateAxis,
    Skew,
    SkewAxis,
    Transform,

    Margin,
    MarginAxis,
    MarginSide,

    Padding,
    PaddingAxis,
    PaddingSide,

    SpaceAxis,

    Rounded,
    RoundedSide,
    RoundedCorner,

    Inset,
    InsetAxis,
    InsetSide,
    PositionSide,

    BorderSpacing,
    BorderSpacingAxis,

    BorderColor,
    BorderColorAxis,
    BorderColorSide,

    BorderWidth,
    BorderWidthAxis,
    BorderWidthSide,

    BackgroundImage,
    FromColor,
    FromPosition,
    ViaColor,
    ViaPosition,
    ToColor,
    ToPosition,

    Size,
    SizeAxis,

    #[default]
    Disorder,

    Grouped,
    Property,
}

impl FromStr for OrderingKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "translate" => Self::Translate,
            "translate-x" | "translate-y" => Self::TranslateAxis,
            "scale" => Self::Scale,
            "scale-x" | "scale-y" => Self::ScaleAxis,
            "rotate" => Self::Rotate,
            "rotate-x" | "rotate-y" => Self::RotateAxis,
            "skew" => Self::Skew,
            "skew-x" | "skew-y" => Self::SkewAxis,
            "transform" => Self::Transform,

            "margin" => Self::Margin,
            "margin-x" | "margin-y" => Self::MarginAxis,
            "margin-top" | "margin-right" | "margin-bottom" | "margin-left" => Self::MarginSide,

            "padding" => Self::Padding,
            "padding-x" | "padding-y" => Self::PaddingAxis,
            "padding-top" | "padding-right" | "padding-bottom" | "padding-left" => {
                Self::PaddingSide
            }

            "space-x" | "space-y" => Self::SpaceAxis,

            "rounded" => Self::Rounded,
            "rounded-top-left"
            | "rounded-top-right"
            | "rounded-bottom-right"
            | "rounded-bottom-left" => Self::RoundedCorner,
            "rounded-t" | "rounded-r" | "rounded-b" | "rounded-l" => Self::RoundedSide,

            "inset" => Self::Inset,
            "inset-x" | "inset-y" => Self::InsetAxis,
            "inset-top" | "inset-right" | "inset-bottom" | "inset-left" => Self::InsetSide,
            "top" | "right" | "bottom" | "left" => Self::PositionSide,

            "border-spacing" => Self::BorderSpacing,
            "border-spacing-x" | "border-spacing-y" => Self::BorderSpacingAxis,

            "border-color" => Self::BorderColor,
            "border-color-x" | "border-color-y" => Self::BorderColorAxis,
            "border-color-top"
            | "border-color-right"
            | "border-color-bottom"
            | "border-color-left" => Self::BorderColorSide,

            "border-width" => Self::BorderWidth,
            "border-right-width"
            | "border-left-width"
            | "border-top-width"
            | "border-bottom-width" => Self::BorderWidthAxis,
            "border-top-right-width"
            | "border-top-left-width"
            | "border-bottom-right-width"
            | "border-bottom-left-width" => Self::BorderWidthSide,

            "w" | "h" => Self::SizeAxis,

            _ => return Err(()),
        })
    }
}
