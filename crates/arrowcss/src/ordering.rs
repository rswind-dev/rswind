use std::hash::Hash;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
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

    Size,
    SizeAxis,

    #[default]
    Disorder,

    Grouped,
    Property,
}
