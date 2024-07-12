use std::ops::Deref;

use lightningcss::properties::PropertyId;
use serde::Deserialize;
use smol_str::SmolStr;

use crate::{
    parse::{AdditionalCssHandler, UtilityBuilder},
    process::{MetaData, UtilityHandler},
    types::CssTypeValidator,
};

pub use crate::config::StaticUtilityConfig;

#[derive(Debug, Deserialize, Default, instance_code::InstanceCode)]
#[instance(path = rswind_core::codegen)]
pub struct UtilityInput {
    pub utilities: Vec<UtilityBuilder>,
}

impl instance_code::InstanceCode for Box<dyn AdditionalCssHandler> {
    fn instance_code(&self) -> instance_code::TokenStream {
        let css = self
            .handle(SmolStr::default())
            .expect("InstanceCode of AdditionalCssHandler should return Some");

        let rule_list = css.deref().instance_code();
        instance_code::quote! {
            std::boxed::Box::new(Arc::new(#rule_list))
        }
    }
}

impl instance_code::InstanceCode for UtilityHandler {
    fn instance_code(&self) -> instance_code::TokenStream {
        use instance_code::quote;
        let color = crate::common::as_color("$0", Some("1"));

        let decls = self.0(MetaData::default(), SmolStr::new("$0")).decls;
        let with_modifier = self.0(MetaData::modifier("1"), SmolStr::new("$0")).decls;

        let value_count = decls.iter().filter(|d| d.value.contains("$0")).count();

        let decls = decls
            .into_iter()
            .zip(with_modifier)
            .map(|(decl, with_modifier)| {
                let name = decl.name.as_str();
                let value = decl.value.as_str();
                if value.contains("$0") {
                    let template = value.replace("$0", "{}");
                    let value = match () {
                        _ if with_modifier.value.contains(color.as_str()) => {
                            quote!(rswind_core::common::as_color(&value, _meta.modifier.as_deref()))
                        }
                        _ if value_count > 1 => {
                            quote!(value.clone())
                        }
                        _ => {
                            quote!(value)
                        }
                    };
                    quote!(#name: smol_str::format_smolstr!(#template, #value);)
                } else {
                    quote!(#name: #value;)
                }
            })
            .collect::<Vec<_>>();
        quote! {
            rswind_core::process::UtilityHandler(
                Box::new(|_meta, value| {
                    rswind_core::css::css! {
                        #(#decls)*
                    }
                })
            )
        }
    }
}

impl instance_code::InstanceCode for CssTypeValidator {
    fn instance_code(&self) -> instance_code::TokenStream {
        use instance_code::quote;
        match self {
            CssTypeValidator::Property(prop) => {
                #[rustfmt::skip]
                let prop = match prop {
                    PropertyId::Flex(_) => quote!(lightningcss::properties::PropertyId::Flex(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexShrink(_) => quote!(lightningcss::properties::PropertyId::FlexShrink(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexGrow(_) => quote!(lightningcss::properties::PropertyId::FlexGrow(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexBasis(_) => quote!(lightningcss::properties::PropertyId::FlexBasis(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TransformOrigin(_) => quote!(lightningcss::properties::PropertyId::TransformOrigin(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Rotate => quote!(lightningcss::properties::PropertyId::Rotate),
                    PropertyId::Transform(_) => quote!(lightningcss::properties::PropertyId::Transform(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BorderRightWidth => quote!(lightningcss::properties::PropertyId::BorderRightWidth),
                    PropertyId::BorderTopWidth => quote!(lightningcss::properties::PropertyId::BorderTopWidth),
                    PropertyId::BackgroundPosition => quote!(lightningcss::properties::PropertyId::BackgroundPosition),
                    PropertyId::BackgroundSize => quote!(lightningcss::properties::PropertyId::BackgroundSize),
                    PropertyId::BackgroundImage => quote!(lightningcss::properties::PropertyId::BackgroundImage),
                    PropertyId::FontSize => quote!(lightningcss::properties::PropertyId::FontSize),
                    PropertyId::LineHeight => quote!(lightningcss::properties::PropertyId::LineHeight),
                    PropertyId::FontWeight => quote!(lightningcss::properties::PropertyId::FontWeight),
                    PropertyId::TextIndent => quote!(lightningcss::properties::PropertyId::TextIndent),
                    PropertyId::FontStretch => quote!(lightningcss::properties::PropertyId::FontStretch),
                    PropertyId::Cursor => quote!(lightningcss::properties::PropertyId::Cursor),
                    PropertyId::ListStyleType => quote!(lightningcss::properties::PropertyId::ListStyleType),
                    PropertyId::ListStyleImage => quote!(lightningcss::properties::PropertyId::ListStyleImage),
                    PropertyId::GridAutoColumns => quote!(lightningcss::properties::PropertyId::GridAutoColumns),
                    PropertyId::GridAutoRows => quote!(lightningcss::properties::PropertyId::GridAutoRows),
                    PropertyId::Gap => quote!(lightningcss::properties::PropertyId::Gap),
                    PropertyId::AccentColor => quote!(lightningcss::properties::PropertyId::AccentColor),
                    PropertyId::BorderWidth => quote!(lightningcss::properties::PropertyId::BorderWidth),
                    PropertyId::OutlineWidth => quote!(lightningcss::properties::PropertyId::OutlineWidth),
                    PropertyId::BoxShadow(_) => quote!(lightningcss::properties::PropertyId::BoxShadow(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BackgroundColor => quote!(lightningcss::properties::PropertyId::BackgroundColor),
                    PropertyId::BackgroundPositionX => quote!(lightningcss::properties::PropertyId::BackgroundPositionX),
                    PropertyId::BackgroundPositionY => quote!(lightningcss::properties::PropertyId::BackgroundPositionY),
                    PropertyId::BackgroundRepeat => quote!(lightningcss::properties::PropertyId::BackgroundRepeat),
                    PropertyId::BackgroundAttachment => quote!(lightningcss::properties::PropertyId::BackgroundAttachment),
                    PropertyId::BackgroundClip(_) => quote!(lightningcss::properties::PropertyId::BackgroundClip(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BackgroundOrigin => quote!(lightningcss::properties::PropertyId::BackgroundOrigin),
                    PropertyId::Background => quote!(lightningcss::properties::PropertyId::Background),
                    PropertyId::Opacity => quote!(lightningcss::properties::PropertyId::Opacity),
                    PropertyId::Color => quote!(lightningcss::properties::PropertyId::Color),
                    PropertyId::Display => quote!(lightningcss::properties::PropertyId::Display),
                    PropertyId::Visibility => quote!(lightningcss::properties::PropertyId::Visibility),
                    PropertyId::Width => quote!(lightningcss::properties::PropertyId::Width),
                    PropertyId::Height => quote!(lightningcss::properties::PropertyId::Height),
                    PropertyId::MinWidth => quote!(lightningcss::properties::PropertyId::MinWidth),
                    PropertyId::MinHeight => quote!(lightningcss::properties::PropertyId::MinHeight),
                    PropertyId::MaxWidth => quote!(lightningcss::properties::PropertyId::MaxWidth),
                    PropertyId::MaxHeight => quote!(lightningcss::properties::PropertyId::MaxHeight),
                    PropertyId::BlockSize => quote!(lightningcss::properties::PropertyId::BlockSize),
                    PropertyId::InlineSize => quote!(lightningcss::properties::PropertyId::InlineSize),
                    PropertyId::MinBlockSize => quote!(lightningcss::properties::PropertyId::MinBlockSize),
                    PropertyId::MinInlineSize => quote!(lightningcss::properties::PropertyId::MinInlineSize),
                    PropertyId::MaxBlockSize => quote!(lightningcss::properties::PropertyId::MaxBlockSize),
                    PropertyId::MaxInlineSize => quote!(lightningcss::properties::PropertyId::MaxInlineSize),
                    PropertyId::BoxSizing(_) => quote!(lightningcss::properties::PropertyId::BoxSizing(_)),
                    PropertyId::AspectRatio => quote!(lightningcss::properties::PropertyId::AspectRatio),
                    PropertyId::Overflow => quote!(lightningcss::properties::PropertyId::Overflow),
                    PropertyId::OverflowX => quote!(lightningcss::properties::PropertyId::OverflowX),
                    PropertyId::OverflowY => quote!(lightningcss::properties::PropertyId::OverflowY),
                    PropertyId::TextOverflow(_) => quote!(lightningcss::properties::PropertyId::TextOverflow(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Position => quote!(lightningcss::properties::PropertyId::Position),
                    PropertyId::Top => quote!(lightningcss::properties::PropertyId::Top),
                    PropertyId::Bottom => quote!(lightningcss::properties::PropertyId::Bottom),
                    PropertyId::Left => quote!(lightningcss::properties::PropertyId::Left),
                    PropertyId::Right => quote!(lightningcss::properties::PropertyId::Right),
                    PropertyId::InsetBlockStart => quote!(lightningcss::properties::PropertyId::InsetBlockStart),
                    PropertyId::InsetBlockEnd => quote!(lightningcss::properties::PropertyId::InsetBlockEnd),
                    PropertyId::InsetInlineStart => quote!(lightningcss::properties::PropertyId::InsetInlineStart),
                    PropertyId::InsetInlineEnd => quote!(lightningcss::properties::PropertyId::InsetInlineEnd),
                    PropertyId::InsetBlock => quote!(lightningcss::properties::PropertyId::InsetBlock),
                    PropertyId::InsetInline => quote!(lightningcss::properties::PropertyId::InsetInline),
                    PropertyId::Inset => quote!(lightningcss::properties::PropertyId::Inset),
                    PropertyId::BorderSpacing => quote!(lightningcss::properties::PropertyId::BorderSpacing),
                    PropertyId::BorderTopColor => quote!(lightningcss::properties::PropertyId::BorderTopColor),
                    PropertyId::BorderBottomColor => quote!(lightningcss::properties::PropertyId::BorderBottomColor),
                    PropertyId::BorderLeftColor => quote!(lightningcss::properties::PropertyId::BorderLeftColor),
                    PropertyId::BorderRightColor => quote!(lightningcss::properties::PropertyId::BorderRightColor),
                    PropertyId::BorderBlockStartColor => quote!(lightningcss::properties::PropertyId::BorderBlockStartColor),
                    PropertyId::BorderBlockEndColor => quote!(lightningcss::properties::PropertyId::BorderBlockEndColor),
                    PropertyId::BorderInlineStartColor => quote!(lightningcss::properties::PropertyId::BorderInlineStartColor),
                    PropertyId::BorderInlineEndColor => quote!(lightningcss::properties::PropertyId::BorderInlineEndColor),
                    PropertyId::BorderTopStyle => quote!(lightningcss::properties::PropertyId::BorderTopStyle),
                    PropertyId::BorderBottomStyle => quote!(lightningcss::properties::PropertyId::BorderBottomStyle),
                    PropertyId::BorderLeftStyle => quote!(lightningcss::properties::PropertyId::BorderLeftStyle),
                    PropertyId::BorderRightStyle => quote!(lightningcss::properties::PropertyId::BorderRightStyle),
                    PropertyId::BorderBlockStartStyle => quote!(lightningcss::properties::PropertyId::BorderBlockStartStyle),
                    PropertyId::BorderBlockEndStyle => quote!(lightningcss::properties::PropertyId::BorderBlockEndStyle),
                    PropertyId::BorderInlineStartStyle => quote!(lightningcss::properties::PropertyId::BorderInlineStartStyle),
                    PropertyId::BorderInlineEndStyle => quote!(lightningcss::properties::PropertyId::BorderInlineEndStyle),
                    PropertyId::BorderBottomWidth => quote!(lightningcss::properties::PropertyId::BorderBottomWidth),
                    PropertyId::BorderLeftWidth => quote!(lightningcss::properties::PropertyId::BorderLeftWidth),
                    PropertyId::BorderBlockStartWidth => quote!(lightningcss::properties::PropertyId::BorderBlockStartWidth),
                    PropertyId::BorderBlockEndWidth => quote!(lightningcss::properties::PropertyId::BorderBlockEndWidth),
                    PropertyId::BorderInlineStartWidth => quote!(lightningcss::properties::PropertyId::BorderInlineStartWidth),
                    PropertyId::BorderInlineEndWidth => quote!(lightningcss::properties::PropertyId::BorderInlineEndWidth),
                    PropertyId::BorderTopLeftRadius(_) => quote!(lightningcss::properties::PropertyId::BorderTopLeftRadius(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BorderTopRightRadius(_) => quote!(lightningcss::properties::PropertyId::BorderTopRightRadius(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BorderBottomLeftRadius(_) => quote!(lightningcss::properties::PropertyId::BorderBottomLeftRadius(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BorderBottomRightRadius(_) => quote!(lightningcss::properties::PropertyId::BorderBottomRightRadius(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BorderStartStartRadius => quote!(lightningcss::properties::PropertyId::BorderStartStartRadius),
                    PropertyId::BorderStartEndRadius => quote!(lightningcss::properties::PropertyId::BorderStartEndRadius),
                    PropertyId::BorderEndStartRadius => quote!(lightningcss::properties::PropertyId::BorderEndStartRadius),
                    PropertyId::BorderEndEndRadius => quote!(lightningcss::properties::PropertyId::BorderEndEndRadius),
                    PropertyId::BorderRadius(_) => quote!(lightningcss::properties::PropertyId::BorderRadius(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BorderImageSource => quote!(lightningcss::properties::PropertyId::BorderImageSource),
                    PropertyId::BorderImageOutset => quote!(lightningcss::properties::PropertyId::BorderImageOutset),
                    PropertyId::BorderImageRepeat => quote!(lightningcss::properties::PropertyId::BorderImageRepeat),
                    PropertyId::BorderImageWidth => quote!(lightningcss::properties::PropertyId::BorderImageWidth),
                    PropertyId::BorderImageSlice => quote!(lightningcss::properties::PropertyId::BorderImageSlice),
                    PropertyId::BorderImage(_) => quote!(lightningcss::properties::PropertyId::BorderImage(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BorderColor => quote!(lightningcss::properties::PropertyId::BorderColor),
                    PropertyId::BorderStyle => quote!(lightningcss::properties::PropertyId::BorderStyle),
                    PropertyId::BorderBlockColor => quote!(lightningcss::properties::PropertyId::BorderBlockColor),
                    PropertyId::BorderBlockStyle => quote!(lightningcss::properties::PropertyId::BorderBlockStyle),
                    PropertyId::BorderBlockWidth => quote!(lightningcss::properties::PropertyId::BorderBlockWidth),
                    PropertyId::BorderInlineColor => quote!(lightningcss::properties::PropertyId::BorderInlineColor),
                    PropertyId::BorderInlineStyle => quote!(lightningcss::properties::PropertyId::BorderInlineStyle),
                    PropertyId::BorderInlineWidth => quote!(lightningcss::properties::PropertyId::BorderInlineWidth),
                    PropertyId::Border => quote!(lightningcss::properties::PropertyId::Border),
                    PropertyId::BorderTop => quote!(lightningcss::properties::PropertyId::BorderTop),
                    PropertyId::BorderBottom => quote!(lightningcss::properties::PropertyId::BorderBottom),
                    PropertyId::BorderLeft => quote!(lightningcss::properties::PropertyId::BorderLeft),
                    PropertyId::BorderRight => quote!(lightningcss::properties::PropertyId::BorderRight),
                    PropertyId::BorderBlock => quote!(lightningcss::properties::PropertyId::BorderBlock),
                    PropertyId::BorderBlockStart => quote!(lightningcss::properties::PropertyId::BorderBlockStart),
                    PropertyId::BorderBlockEnd => quote!(lightningcss::properties::PropertyId::BorderBlockEnd),
                    PropertyId::BorderInline => quote!(lightningcss::properties::PropertyId::BorderInline),
                    PropertyId::BorderInlineStart => quote!(lightningcss::properties::PropertyId::BorderInlineStart),
                    PropertyId::BorderInlineEnd => quote!(lightningcss::properties::PropertyId::BorderInlineEnd),
                    PropertyId::Outline => quote!(lightningcss::properties::PropertyId::Outline),
                    PropertyId::OutlineColor => quote!(lightningcss::properties::PropertyId::OutlineColor),
                    PropertyId::OutlineStyle => quote!(lightningcss::properties::PropertyId::OutlineStyle),
                    PropertyId::FlexDirection(_) => quote!(lightningcss::properties::PropertyId::FlexDirection(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexWrap(_) => quote!(lightningcss::properties::PropertyId::FlexWrap(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexFlow(_) => quote!(lightningcss::properties::PropertyId::FlexFlow(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Order(_) => quote!(lightningcss::properties::PropertyId::Order(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AlignContent(_) => quote!(lightningcss::properties::PropertyId::AlignContent(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::JustifyContent(_) => quote!(lightningcss::properties::PropertyId::JustifyContent(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::PlaceContent => quote!(lightningcss::properties::PropertyId::PlaceContent),
                    PropertyId::AlignSelf(_) => quote!(lightningcss::properties::PropertyId::AlignSelf(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::JustifySelf => quote!(lightningcss::properties::PropertyId::JustifySelf),
                    PropertyId::PlaceSelf => quote!(lightningcss::properties::PropertyId::PlaceSelf),
                    PropertyId::AlignItems(_) => quote!(lightningcss::properties::PropertyId::AlignItems(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::JustifyItems => quote!(lightningcss::properties::PropertyId::JustifyItems),
                    PropertyId::PlaceItems => quote!(lightningcss::properties::PropertyId::PlaceItems),
                    PropertyId::RowGap => quote!(lightningcss::properties::PropertyId::RowGap),
                    PropertyId::ColumnGap => quote!(lightningcss::properties::PropertyId::ColumnGap),
                    PropertyId::BoxOrient(_) => quote!(lightningcss::properties::PropertyId::BoxOrient(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BoxDirection(_) => quote!(lightningcss::properties::PropertyId::BoxDirection(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BoxOrdinalGroup(_) => quote!(lightningcss::properties::PropertyId::BoxOrdinalGroup(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BoxAlign(_) => quote!(lightningcss::properties::PropertyId::BoxAlign(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BoxFlex(_) => quote!(lightningcss::properties::PropertyId::BoxFlex(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BoxFlexGroup(_) => quote!(lightningcss::properties::PropertyId::BoxFlexGroup(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BoxPack(_) => quote!(lightningcss::properties::PropertyId::BoxPack(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BoxLines(_) => quote!(lightningcss::properties::PropertyId::BoxLines(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexPack(_) => quote!(lightningcss::properties::PropertyId::FlexPack(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexOrder(_) => quote!(lightningcss::properties::PropertyId::FlexOrder(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexAlign(_) => quote!(lightningcss::properties::PropertyId::FlexAlign(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexItemAlign(_) => quote!(lightningcss::properties::PropertyId::FlexItemAlign(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexLinePack(_) => quote!(lightningcss::properties::PropertyId::FlexLinePack(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexPositive(_) => quote!(lightningcss::properties::PropertyId::FlexPositive(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexNegative(_) => quote!(lightningcss::properties::PropertyId::FlexNegative(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::FlexPreferredSize(_) => quote!(lightningcss::properties::PropertyId::FlexPreferredSize(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::GridTemplateColumns => quote!(lightningcss::properties::PropertyId::GridTemplateColumns),
                    PropertyId::GridTemplateRows => quote!(lightningcss::properties::PropertyId::GridTemplateRows),
                    PropertyId::GridAutoFlow => quote!(lightningcss::properties::PropertyId::GridAutoFlow),
                    PropertyId::GridTemplateAreas => quote!(lightningcss::properties::PropertyId::GridTemplateAreas),
                    PropertyId::GridTemplate => quote!(lightningcss::properties::PropertyId::GridTemplate),
                    PropertyId::Grid => quote!(lightningcss::properties::PropertyId::Grid),
                    PropertyId::GridRowStart => quote!(lightningcss::properties::PropertyId::GridRowStart),
                    PropertyId::GridRowEnd => quote!(lightningcss::properties::PropertyId::GridRowEnd),
                    PropertyId::GridColumnStart => quote!(lightningcss::properties::PropertyId::GridColumnStart),
                    PropertyId::GridColumnEnd => quote!(lightningcss::properties::PropertyId::GridColumnEnd),
                    PropertyId::GridRow => quote!(lightningcss::properties::PropertyId::GridRow),
                    PropertyId::GridColumn => quote!(lightningcss::properties::PropertyId::GridColumn),
                    PropertyId::GridArea => quote!(lightningcss::properties::PropertyId::GridArea),
                    PropertyId::MarginTop => quote!(lightningcss::properties::PropertyId::MarginTop),
                    PropertyId::MarginBottom => quote!(lightningcss::properties::PropertyId::MarginBottom),
                    PropertyId::MarginLeft => quote!(lightningcss::properties::PropertyId::MarginLeft),
                    PropertyId::MarginRight => quote!(lightningcss::properties::PropertyId::MarginRight),
                    PropertyId::MarginBlockStart => quote!(lightningcss::properties::PropertyId::MarginBlockStart),
                    PropertyId::MarginBlockEnd => quote!(lightningcss::properties::PropertyId::MarginBlockEnd),
                    PropertyId::MarginInlineStart => quote!(lightningcss::properties::PropertyId::MarginInlineStart),
                    PropertyId::MarginInlineEnd => quote!(lightningcss::properties::PropertyId::MarginInlineEnd),
                    PropertyId::MarginBlock => quote!(lightningcss::properties::PropertyId::MarginBlock),
                    PropertyId::MarginInline => quote!(lightningcss::properties::PropertyId::MarginInline),
                    PropertyId::Margin => quote!(lightningcss::properties::PropertyId::Margin),
                    PropertyId::PaddingTop => quote!(lightningcss::properties::PropertyId::PaddingTop),
                    PropertyId::PaddingBottom => quote!(lightningcss::properties::PropertyId::PaddingBottom),
                    PropertyId::PaddingLeft => quote!(lightningcss::properties::PropertyId::PaddingLeft),
                    PropertyId::PaddingRight => quote!(lightningcss::properties::PropertyId::PaddingRight),
                    PropertyId::PaddingBlockStart => quote!(lightningcss::properties::PropertyId::PaddingBlockStart),
                    PropertyId::PaddingBlockEnd => quote!(lightningcss::properties::PropertyId::PaddingBlockEnd),
                    PropertyId::PaddingInlineStart => quote!(lightningcss::properties::PropertyId::PaddingInlineStart),
                    PropertyId::PaddingInlineEnd => quote!(lightningcss::properties::PropertyId::PaddingInlineEnd),
                    PropertyId::PaddingBlock => quote!(lightningcss::properties::PropertyId::PaddingBlock),
                    PropertyId::PaddingInline => quote!(lightningcss::properties::PropertyId::PaddingInline),
                    PropertyId::Padding => quote!(lightningcss::properties::PropertyId::Padding),
                    PropertyId::ScrollMarginTop => quote!(lightningcss::properties::PropertyId::ScrollMarginTop),
                    PropertyId::ScrollMarginBottom => quote!(lightningcss::properties::PropertyId::ScrollMarginBottom),
                    PropertyId::ScrollMarginLeft => quote!(lightningcss::properties::PropertyId::ScrollMarginLeft),
                    PropertyId::ScrollMarginRight => quote!(lightningcss::properties::PropertyId::ScrollMarginRight),
                    PropertyId::ScrollMarginBlockStart => quote!(lightningcss::properties::PropertyId::ScrollMarginBlockStart),
                    PropertyId::ScrollMarginBlockEnd => quote!(lightningcss::properties::PropertyId::ScrollMarginBlockEnd),
                    PropertyId::ScrollMarginInlineStart => quote!(lightningcss::properties::PropertyId::ScrollMarginInlineStart),
                    PropertyId::ScrollMarginInlineEnd => quote!(lightningcss::properties::PropertyId::ScrollMarginInlineEnd),
                    PropertyId::ScrollMarginBlock => quote!(lightningcss::properties::PropertyId::ScrollMarginBlock),
                    PropertyId::ScrollMarginInline => quote!(lightningcss::properties::PropertyId::ScrollMarginInline),
                    PropertyId::ScrollMargin => quote!(lightningcss::properties::PropertyId::ScrollMargin),
                    PropertyId::ScrollPaddingTop => quote!(lightningcss::properties::PropertyId::ScrollPaddingTop),
                    PropertyId::ScrollPaddingBottom => quote!(lightningcss::properties::PropertyId::ScrollPaddingBottom),
                    PropertyId::ScrollPaddingLeft => quote!(lightningcss::properties::PropertyId::ScrollPaddingLeft),
                    PropertyId::ScrollPaddingRight => quote!(lightningcss::properties::PropertyId::ScrollPaddingRight),
                    PropertyId::ScrollPaddingBlockStart => quote!(lightningcss::properties::PropertyId::ScrollPaddingBlockStart),
                    PropertyId::ScrollPaddingBlockEnd => quote!(lightningcss::properties::PropertyId::ScrollPaddingBlockEnd),
                    PropertyId::ScrollPaddingInlineStart => quote!(lightningcss::properties::PropertyId::ScrollPaddingInlineStart),
                    PropertyId::ScrollPaddingInlineEnd => quote!(lightningcss::properties::PropertyId::ScrollPaddingInlineEnd),
                    PropertyId::ScrollPaddingBlock => quote!(lightningcss::properties::PropertyId::ScrollPaddingBlock),
                    PropertyId::ScrollPaddingInline => quote!(lightningcss::properties::PropertyId::ScrollPaddingInline),
                    PropertyId::ScrollPadding => quote!(lightningcss::properties::PropertyId::ScrollPadding),
                    PropertyId::FontFamily => quote!(lightningcss::properties::PropertyId::FontFamily),
                    PropertyId::FontStyle => quote!(lightningcss::properties::PropertyId::FontStyle),
                    PropertyId::FontVariantCaps => quote!(lightningcss::properties::PropertyId::FontVariantCaps),
                    PropertyId::Font => quote!(lightningcss::properties::PropertyId::Font),
                    PropertyId::VerticalAlign => quote!(lightningcss::properties::PropertyId::VerticalAlign),
                    PropertyId::FontPalette => quote!(lightningcss::properties::PropertyId::FontPalette),
                    PropertyId::TransitionProperty(_) => quote!(lightningcss::properties::PropertyId::TransitionProperty(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TransitionDuration(_) => quote!(lightningcss::properties::PropertyId::TransitionDuration(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TransitionDelay(_) => quote!(lightningcss::properties::PropertyId::TransitionDelay(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TransitionTimingFunction(_) => quote!(lightningcss::properties::PropertyId::TransitionTimingFunction(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Transition(_) => quote!(lightningcss::properties::PropertyId::Transition(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AnimationName(_) => quote!(lightningcss::properties::PropertyId::AnimationName(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AnimationDuration(_) => quote!(lightningcss::properties::PropertyId::AnimationDuration(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AnimationTimingFunction(_) => quote!(lightningcss::properties::PropertyId::AnimationTimingFunction(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AnimationIterationCount(_) => quote!(lightningcss::properties::PropertyId::AnimationIterationCount(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AnimationDirection(_) => quote!(lightningcss::properties::PropertyId::AnimationDirection(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AnimationPlayState(_) => quote!(lightningcss::properties::PropertyId::AnimationPlayState(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AnimationDelay(_) => quote!(lightningcss::properties::PropertyId::AnimationDelay(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AnimationFillMode(_) => quote!(lightningcss::properties::PropertyId::AnimationFillMode(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::AnimationComposition => quote!(lightningcss::properties::PropertyId::AnimationComposition),
                    PropertyId::AnimationTimeline => quote!(lightningcss::properties::PropertyId::AnimationTimeline),
                    PropertyId::Animation(_) => quote!(lightningcss::properties::PropertyId::Animation(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TransformStyle(_) => quote!(lightningcss::properties::PropertyId::TransformStyle(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TransformBox => quote!(lightningcss::properties::PropertyId::TransformBox),
                    PropertyId::BackfaceVisibility(_) => quote!(lightningcss::properties::PropertyId::BackfaceVisibility(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Perspective(_) => quote!(lightningcss::properties::PropertyId::Perspective(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::PerspectiveOrigin(_) => quote!(lightningcss::properties::PropertyId::PerspectiveOrigin(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Translate => quote!(lightningcss::properties::PropertyId::Translate),
                    PropertyId::Scale => quote!(lightningcss::properties::PropertyId::Scale),
                    PropertyId::TextTransform => quote!(lightningcss::properties::PropertyId::TextTransform),
                    PropertyId::WhiteSpace => quote!(lightningcss::properties::PropertyId::WhiteSpace),
                    PropertyId::TabSize(_) => quote!(lightningcss::properties::PropertyId::TabSize(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::WordBreak => quote!(lightningcss::properties::PropertyId::WordBreak),
                    PropertyId::LineBreak => quote!(lightningcss::properties::PropertyId::LineBreak),
                    PropertyId::Hyphens(_) => quote!(lightningcss::properties::PropertyId::Hyphens(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::OverflowWrap => quote!(lightningcss::properties::PropertyId::OverflowWrap),
                    PropertyId::WordWrap => quote!(lightningcss::properties::PropertyId::WordWrap),
                    PropertyId::TextAlign => quote!(lightningcss::properties::PropertyId::TextAlign),
                    PropertyId::TextAlignLast(_) => quote!(lightningcss::properties::PropertyId::TextAlignLast(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextJustify => quote!(lightningcss::properties::PropertyId::TextJustify),
                    PropertyId::WordSpacing => quote!(lightningcss::properties::PropertyId::WordSpacing),
                    PropertyId::LetterSpacing => quote!(lightningcss::properties::PropertyId::LetterSpacing),
                    PropertyId::TextDecorationLine(_) => quote!(lightningcss::properties::PropertyId::TextDecorationLine(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextDecorationStyle(_) => quote!(lightningcss::properties::PropertyId::TextDecorationStyle(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextDecorationColor(_) => quote!(lightningcss::properties::PropertyId::TextDecorationColor(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextDecorationThickness => quote!(lightningcss::properties::PropertyId::TextDecorationThickness),
                    PropertyId::TextDecoration(_) => quote!(lightningcss::properties::PropertyId::TextDecoration(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextDecorationSkipInk(_) => quote!(lightningcss::properties::PropertyId::TextDecorationSkipInk(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextEmphasisStyle(_) => quote!(lightningcss::properties::PropertyId::TextEmphasisStyle(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextEmphasisColor(_) => quote!(lightningcss::properties::PropertyId::TextEmphasisColor(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextEmphasis(_) => quote!(lightningcss::properties::PropertyId::TextEmphasis(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextEmphasisPosition(_) => quote!(lightningcss::properties::PropertyId::TextEmphasisPosition(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::TextShadow => quote!(lightningcss::properties::PropertyId::TextShadow),
                    PropertyId::TextSizeAdjust(_) => quote!(lightningcss::properties::PropertyId::TextSizeAdjust(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Direction => quote!(lightningcss::properties::PropertyId::Direction),
                    PropertyId::UnicodeBidi => quote!(lightningcss::properties::PropertyId::UnicodeBidi),
                    PropertyId::BoxDecorationBreak(_) => quote!(lightningcss::properties::PropertyId::BoxDecorationBreak(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Resize => quote!(lightningcss::properties::PropertyId::Resize),
                    PropertyId::CaretColor => quote!(lightningcss::properties::PropertyId::CaretColor),
                    PropertyId::CaretShape => quote!(lightningcss::properties::PropertyId::CaretShape),
                    PropertyId::Caret => quote!(lightningcss::properties::PropertyId::Caret),
                    PropertyId::UserSelect(_) => quote!(lightningcss::properties::PropertyId::UserSelect(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Appearance(_) => quote!(lightningcss::properties::PropertyId::Appearance(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::ListStylePosition => quote!(lightningcss::properties::PropertyId::ListStylePosition),
                    PropertyId::ListStyle => quote!(lightningcss::properties::PropertyId::ListStyle),
                    PropertyId::MarkerSide => quote!(lightningcss::properties::PropertyId::MarkerSide),
                    PropertyId::Composes => quote!(lightningcss::properties::PropertyId::Composes),
                    PropertyId::Fill => quote!(lightningcss::properties::PropertyId::Fill),
                    PropertyId::FillRule => quote!(lightningcss::properties::PropertyId::FillRule),
                    PropertyId::FillOpacity => quote!(lightningcss::properties::PropertyId::FillOpacity),
                    PropertyId::Stroke => quote!(lightningcss::properties::PropertyId::Stroke),
                    PropertyId::StrokeOpacity => quote!(lightningcss::properties::PropertyId::StrokeOpacity),
                    PropertyId::StrokeWidth => quote!(lightningcss::properties::PropertyId::StrokeWidth),
                    PropertyId::StrokeLinecap => quote!(lightningcss::properties::PropertyId::StrokeLinecap),
                    PropertyId::StrokeLinejoin => quote!(lightningcss::properties::PropertyId::StrokeLinejoin),
                    PropertyId::StrokeMiterlimit => quote!(lightningcss::properties::PropertyId::StrokeMiterlimit),
                    PropertyId::StrokeDasharray => quote!(lightningcss::properties::PropertyId::StrokeDasharray),
                    PropertyId::StrokeDashoffset => quote!(lightningcss::properties::PropertyId::StrokeDashoffset),
                    PropertyId::MarkerStart => quote!(lightningcss::properties::PropertyId::MarkerStart),
                    PropertyId::MarkerMid => quote!(lightningcss::properties::PropertyId::MarkerMid),
                    PropertyId::MarkerEnd => quote!(lightningcss::properties::PropertyId::MarkerEnd),
                    PropertyId::Marker => quote!(lightningcss::properties::PropertyId::Marker),
                    PropertyId::ColorInterpolation => quote!(lightningcss::properties::PropertyId::ColorInterpolation),
                    PropertyId::ColorInterpolationFilters => quote!(lightningcss::properties::PropertyId::ColorInterpolationFilters),
                    PropertyId::ColorRendering => quote!(lightningcss::properties::PropertyId::ColorRendering),
                    PropertyId::ShapeRendering => quote!(lightningcss::properties::PropertyId::ShapeRendering),
                    PropertyId::TextRendering => quote!(lightningcss::properties::PropertyId::TextRendering),
                    PropertyId::ImageRendering => quote!(lightningcss::properties::PropertyId::ImageRendering),
                    PropertyId::ClipPath(_) => quote!(lightningcss::properties::PropertyId::ClipPath(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::ClipRule => quote!(lightningcss::properties::PropertyId::ClipRule),
                    PropertyId::MaskImage(_) => quote!(lightningcss::properties::PropertyId::MaskImage(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::MaskMode => quote!(lightningcss::properties::PropertyId::MaskMode),
                    PropertyId::MaskRepeat(_) => quote!(lightningcss::properties::PropertyId::MaskRepeat(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::MaskPositionX => quote!(lightningcss::properties::PropertyId::MaskPositionX),
                    PropertyId::MaskPositionY => quote!(lightningcss::properties::PropertyId::MaskPositionY),
                    PropertyId::MaskPosition(_) => quote!(lightningcss::properties::PropertyId::MaskPosition(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::MaskClip(_) => quote!(lightningcss::properties::PropertyId::MaskClip(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::MaskOrigin(_) => quote!(lightningcss::properties::PropertyId::MaskOrigin(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::MaskSize(_) => quote!(lightningcss::properties::PropertyId::MaskSize(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::MaskComposite => quote!(lightningcss::properties::PropertyId::MaskComposite),
                    PropertyId::MaskType => quote!(lightningcss::properties::PropertyId::MaskType),
                    PropertyId::Mask(_) => quote!(lightningcss::properties::PropertyId::Mask(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::MaskBorderSource => quote!(lightningcss::properties::PropertyId::MaskBorderSource),
                    PropertyId::MaskBorderMode => quote!(lightningcss::properties::PropertyId::MaskBorderMode),
                    PropertyId::MaskBorderSlice => quote!(lightningcss::properties::PropertyId::MaskBorderSlice),
                    PropertyId::MaskBorderWidth => quote!(lightningcss::properties::PropertyId::MaskBorderWidth),
                    PropertyId::MaskBorderOutset => quote!(lightningcss::properties::PropertyId::MaskBorderOutset),
                    PropertyId::MaskBorderRepeat => quote!(lightningcss::properties::PropertyId::MaskBorderRepeat),
                    PropertyId::MaskBorder => quote!(lightningcss::properties::PropertyId::MaskBorder),
                    PropertyId::WebKitMaskComposite => quote!(lightningcss::properties::PropertyId::WebKitMaskComposite),
                    PropertyId::WebKitMaskSourceType(_) => quote!(lightningcss::properties::PropertyId::WebKitMaskSourceType(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::WebKitMaskBoxImage(_) => quote!(lightningcss::properties::PropertyId::WebKitMaskBoxImage(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::WebKitMaskBoxImageSource(_) => quote!(lightningcss::properties::PropertyId::WebKitMaskBoxImageSource(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::WebKitMaskBoxImageSlice(_) => quote!(lightningcss::properties::PropertyId::WebKitMaskBoxImageSlice(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::WebKitMaskBoxImageWidth(_) => quote!(lightningcss::properties::PropertyId::WebKitMaskBoxImageWidth(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::WebKitMaskBoxImageOutset(_) => quote!(lightningcss::properties::PropertyId::WebKitMaskBoxImageOutset(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::WebKitMaskBoxImageRepeat(_) => quote!(lightningcss::properties::PropertyId::WebKitMaskBoxImageRepeat(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::Filter(_) => quote!(lightningcss::properties::PropertyId::Filter(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::BackdropFilter(_) => quote!(lightningcss::properties::PropertyId::BackdropFilter(lightningcss::vendor_prefix::VendorPrefix::None)),
                    PropertyId::ZIndex => quote!(lightningcss::properties::PropertyId::ZIndex),
                    PropertyId::ContainerType => quote!(lightningcss::properties::PropertyId::ContainerType),
                    PropertyId::ContainerName => quote!(lightningcss::properties::PropertyId::ContainerName),
                    PropertyId::Container => quote!(lightningcss::properties::PropertyId::Container),
                    PropertyId::ViewTransitionName => quote!(lightningcss::properties::PropertyId::ViewTransitionName),
                    PropertyId::ColorScheme => quote!(lightningcss::properties::PropertyId::ColorScheme),
                    PropertyId::All => quote!(lightningcss::properties::PropertyId::All),
                    PropertyId::Custom(_) => quote!(lightningcss::properties::PropertyId::Custom(lightningcss::vendor_prefix::VendorPrefix::None)),
                };
                quote! {
                    rswind_core::types::CssTypeValidator::Property(#prop)
                }
            }
            CssTypeValidator::DataType(typ) => {
                let typ = typ.instance_code();
                quote! {
                    rswind_core::types::CssTypeValidator::DataType(#typ)
                }
            }
        }
    }
}
