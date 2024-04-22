use arrowcss_css_macro::css;
use lightningcss::traits::IntoOwned;

use crate::{
    add_theme_rule,
    context::Context,
    ordering::OrderingKey,
    parsing::UtilityBuilder,
    process::{ModifierProcessor, RuleMatchingFn},
    types::{CssDataType, CssProperty},
};

struct RuleAdder<'a, 'c> {
    ctx: &'a mut Context<'c>,
}

impl<'a, 'c> RuleAdder<'a, 'c> {
    pub fn new(ctx: &'a mut Context<'c>) -> Self {
        Self { ctx }
    }

    pub fn add<'b>(
        &'b mut self,
        key: &'b str,
        handler: impl RuleMatchingFn + 'static,
    ) -> UtilityBuilder<'b, 'c> {
        UtilityBuilder::new(self.ctx, key, handler)
    }
}

pub fn load_dynamic_utilities(ctx: &mut Context<'_>) {
    let line_height_map = ctx.get_theme("fontSize:lineHeight").unwrap();
    let line_height_map2 = ctx.get_theme("lineHeight").unwrap();
    let opacity = ctx.get_theme("opacity").unwrap();

    let mut rules = RuleAdder::new(ctx);

    rules
        .add("line-clamp", |_, value| {
            css! {
                "display": "-webkit-box";
                "-webkit-line-clamp": value;
                "-webkit-box-orient": "vertical";
                "overflow": "hidden";
            }
        })
        .with_validator(CssDataType::Number)
        .with_theme("lineClamp");

    rules
        .add("border-spacing", |_, value| {
            css! {
                "--tw-border-spacing-x": value.clone();
                "--tw-border-spacing-y": value.clone();
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_ordering(OrderingKey::BorderSpacing)
        .with_theme("spacing");

    rules
        .add("border-spacing-x", |_, value| {
            css! {
                "--tw-border-spacing-x": value;
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_ordering(OrderingKey::BorderSpacingAxis)
        .with_theme("spacing");

    rules
        .add("border-spacing-y", |_, value| {
            css! {
                "--tw-border-spacing-y": value;
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_ordering(OrderingKey::BorderSpacingAxis)
        .with_theme("spacing");

    rules
        .add("animate", |_, value| {
            css! {
                "animation": value;
            }
        })
        .with_theme("animate");

    rules
        .add("space-x", |_, value| {
            css! {
                "--tw-space-x-reverse": "0";
                "margin-right": format!("calc({value} * var(--tw-space-x-reverse))");
                "margin-left":  format!("calc({value} * calc(1 - var(--tw-space-x-reverse)))");
            }
        })
        .with_theme("spacing")
        .with_wrapper("& > :not([hidden]) ~ :not([hidden])")
        .support_negative();

    rules
        .add("space-y", |_, value| {
            css! {
                "--tw-space-y-reverse": "0";
                "margin-top": format!("calc({value} * calc(1 - var(--tw-space-y-reverse)))");
                "margin-bottom": format!("calc({value} * var(--tw-space-y-reverse))");
            }
        })
        .with_theme("spacing")
        .support_negative();

    rules.add("divide-x", |_, value| {
        css! {
            "--tw-divide-x-reverse": "0";
            "border-right-width": format!("calc({value} * var(--tw-divide-x-reverse))");
            "border-left-width": format!("calc({value} * calc(1 - var(--tw-divide-x-reverse)))");
        }
    })
    .with_theme("borderWidth")
    .with_validator(CssProperty::BorderRightWidth);

    rules
        .add("divide-y", |_, value| {
            css! {
                "--tw-divide-y-reverse": "0";
                "border-top-width": format!("calc({value} * calc(1 - var(--tw-divide-y-reverse)))");
                "border-bottom-width": format!("calc({value} * var(--tw-divide-y-reverse))");
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderTopWidth);

    rules
        .add(
            "divide",
            |meta, value| css!("border-color": as_color(&value, meta.modifier)),
        )
        .with_theme("colors")
        .with_validator(CssProperty::BorderColor)
        .with_modifier(ModifierProcessor {
            validator: Some(Box::new(CssProperty::Opacity)),
            allowed_values: Some(opacity.clone()),
        });

    rules
        .add(
            "border",
            |meta, value| css!("border-width": as_color(&value, meta.modifier)),
        )
        .with_theme("colors")
        .with_validator(CssProperty::BorderColor)
        .with_modifier(ModifierProcessor {
            validator: Some(Box::new(CssProperty::Opacity)),
            allowed_values: Some(opacity.clone()),
        });

    rules
        .add("from", |_, value| {
            css! {
                "--tw-gradient-from": format!("{value} var(--tw-gradient-from-position)");
                // TODO: --tw-gradient-to
                // TODO: properties
                "--tw-gradient-stops": "var(--tw-gradient-from), var(--tw-gradient-to)";
            }
        })
        .with_theme("colors")
        .with_validator(CssProperty::Color);

    rules
        .add(
            "from",
            |_, value| css!("--tw-gradient-from-position": value),
        )
        .with_theme("gradientColorStopPositions")
        .with_validator(CssDataType::LengthPercentage);

    rules.add("via", |_, value| {
        css! {
            "--tw-gradient-via": value;
            "--tw-gradient-via-stops": "var(--tw-gradient-from) var(--tw-gradient-from-position), var(--tw-gradient-via) var(--tw-gradient-via-position), var(--tw-gradient-to) var(--tw-gradient-to-position)";
            "--tw-gradient-stops": "var(--tw-gradient-via-stops)";
        }
    })
    .with_theme("colors")
    .with_validator(CssProperty::Color);

    rules
        .add("via", |_, value| css!("--tw-gradient-via-position": value))
        .with_theme("gradientColorStopPositions")
        .with_validator(CssDataType::LengthPercentage);

    rules.add("to", |_, value| {
        css! {
            "--tw-gradient-to": value;
            "--tw-gradient-stops": "var(--tw-gradient-via-stops, var(--tw-gradient-from) var(--tw-gradient-from-position), var(--tw-gradient-to) var(--tw-gradient-to-position))";
        }
    });

    // fill

    // stoke

    rules
        .add("to", |_, value| css!("--tw-gradient-to-position": value))
        .with_theme("gradientColorStopPositions")
        .with_validator(CssDataType::LengthPercentage);

    rules
        .add("bg", |_, value| css!("background-color": value))
        .with_theme("colors")
        .with_validator(CssProperty::Color);

    rules
        .add("bg", |_, value| css!("background-position": value))
        .with_theme("backgroundPosition")
        .with_validator(CssProperty::BackgroundPosition);

    rules
        .add("bg", |_, value| css!("background-size": value))
        .with_theme("backgroundSize")
        .with_validator(CssProperty::BackgroundSize);

    rules
        .add("bg", |_, value| css!("background-image": value))
        .with_theme("backgroundImage")
        .with_validator(CssProperty::BackgroundImage);

    rules
        .add("text", |_, value| css!("color": value))
        .with_theme("colors")
        .with_validator(CssProperty::Color);

    rules
        .add("text", move |meta, value| {
            let mut font_size = css!("font-size": value.clone());
            if let Some(modifier) = meta.modifier {
                font_size.extend(css!("line-height": modifier));
            } else if let Some(line_height) = meta
                .candidate
                .value
                .and_then(|v| line_height_map.get(v.take_named()?))
            {
                font_size.extend(css!("line-height": line_height.clone().into_owned()));
            }
            font_size
        })
        .with_theme("fontSize")
        .with_validator(CssProperty::FontSize)
        .with_modifier(ModifierProcessor {
            validator: Some(Box::new(CssProperty::LineHeight)),
            allowed_values: Some(line_height_map2),
        });

    use lightningcss::properties::PropertyId::*;
    add_theme_rule!(ctx, {
        "spacing" => {
            // TODO: types, order
            "m" : Margin       => ["margin"]                      in OrderingKey::Margin
            "mx": MarginLeft   => ["margin-left", "margin-right"] in OrderingKey::MarginAxis
            "my": MarginTop    => ["margin-top", "margin-bottom"] in OrderingKey::MarginAxis
            "mt": MarginTop    => ["margin-top"]                  in OrderingKey::MarginSide
            "mr": MarginRight  => ["margin-right"]                in OrderingKey::MarginSide
            "mb": MarginBottom => ["margin-bottom"]               in OrderingKey::MarginSide
            "ml": MarginRight  => ["margin-left"]                 in OrderingKey::MarginSide
            "ms": MarginRight  => ["margin-inline-start"]         in OrderingKey::MarginSide
            "me": MarginRight  => ["margin-inline-end"]           in OrderingKey::MarginSide

            "p" : Padding    => ["padding"]                       in OrderingKey::Padding
            "px": PaddingTop => ["padding-left", "padding-right"] in OrderingKey::PaddingAxis
            "py": PaddingTop => ["padding-top", "padding-bottom"] in OrderingKey::PaddingAxis
            "pt": PaddingTop => ["padding-top"]                   in OrderingKey::PaddingSide
            "pr": PaddingTop => ["padding-right"]                 in OrderingKey::PaddingSide
            "pb": PaddingTop => ["padding-bottom"]                in OrderingKey::PaddingSide
            "pl": PaddingTop => ["padding-left"]                  in OrderingKey::PaddingSide
            "ps": PaddingTop => ["padding-inline-start"]          in OrderingKey::PaddingSide
            "pe": PaddingTop => ["padding-inline-end"]            in OrderingKey::PaddingSide

            "inset"   : Inset => ["top", "right", "bottom", "left"] in OrderingKey::Inset
            "inset-x" : Left  => ["left", "right"]                  in OrderingKey::InsetAxis
            "inset-y" : Top   => ["top", "bottom"]                  in OrderingKey::InsetAxis

            "top":    Top => ["top"]    in OrderingKey::InsetSide
            "right":  Top => ["right"]  in OrderingKey::InsetSide
            "bottom": Top => ["bottom"] in OrderingKey::InsetSide
            "left":   Top => ["left"]   in OrderingKey::InsetSide

            "gap": Gap => ["gap"]

            "size" : Width => ["width", "height"] in OrderingKey::Size
            "w"    : Width => ["width"]           in OrderingKey::SizeAxis
            "h"    : Width => ["height"]          in OrderingKey::SizeAxis
        },
        "lineHeight" => {
            "leading": LineHeight => ["line-height"]
        },
        "colors" => {
            // TODO: as_color
            "border"  : BorderColor => ["border-color"]                            in OrderingKey::BorderColor
            "border-x": BorderColor => ["border-right-color", "border-left-color"] in OrderingKey::BorderColorAxis
            "border-y": BorderColor => ["border-top-color", "border-bottom-color"] in OrderingKey::BorderColorAxis
            "border-s": BorderColor => ["border-inline-start-color"]               in OrderingKey::BorderColorSide
            "border-e": BorderColor => ["border-inline-end-color"]                 in OrderingKey::BorderColorSide
            "border-t": BorderColor => ["border-top-color"]                        in OrderingKey::BorderColorSide
            "border-r": BorderColor => ["border-right-color"]                      in OrderingKey::BorderColorSide
            "border-b": BorderColor => ["border-bottom-color"]                     in OrderingKey::BorderColorSide
            "border-l": BorderColor => ["border-left-color"]                       in OrderingKey::BorderColorSide
        },
        "opacity" => {
            "opacity": Opacity => ["opacity"]
            "divide" => ["--tw-divide-opacity"]
        }
    });
}

fn as_color(value: &str, modifier: Option<String>) -> String {
    modifier
        .and_then(|m| m.parse::<f32>().ok())
        .map(|n| format!("color-mix(in srgb, {} {}%, transparent)", value, n * 100.0))
        .unwrap_or_else(|| value.to_string())
}
