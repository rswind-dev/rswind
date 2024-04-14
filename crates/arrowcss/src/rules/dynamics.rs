use lightningcss::{
    properties::Property,
    traits::IntoOwned,
    values::color::{CssColor, RGBA},
};

use arrowcss_css_macro::css;

use crate::{
    add_theme_rule,
    context::Context,
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

pub fn load_dynamic_rules(ctx: &mut Context<'_>) {
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

    rules.add("border-spacing", |_, value| {
        css! {
            "--tw-border-spacing-x": value.clone();
            "--tw-border-spacing-y": value.clone();
            "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
        }
    })
    .with_theme("spacing");

    rules.add("border-spacing-x", |_, value| {
        css! {
            "--tw-border-spacing-x": value;
            "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
        }
    })
    .with_theme("spacing");

    rules.add("border-spacing-y", |_, value| {
        css! {
            "--tw-border-spacing-y": value;
            "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
        }
    })
    .with_theme("spacing");

    rules
        .add("animate", |_, value| {
            css! {
                "animation": value;
            }
        })
        .with_theme("animate");

    rules.add("space-x", |_, value| {
        css! {
            "--tw-space-x-reverse": "0";
            "margin-right": format!("calc({value} * var(--tw-space-x-reverse))");
            "margin-left":  format!("calc({value} * calc(1 - var(--tw-space-x-reverse)))");
        }
    })
    .with_theme("spacing")
    .with_wrapper("& > :not([hidden]) ~ :not([hidden])")
    .support_negative();

    rules.add("space-y", |_, value| {
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

    rules.add("divide-y", |_, value| {
        css! {
            "--tw-divide-y-reverse": "0";
            "border-top-width": format!("calc({value} * calc(1 - var(--tw-divide-y-reverse)))");
            "border-bottom-width": format!("calc({value} * var(--tw-divide-y-reverse))");
        }
    })
    .with_theme("borderWidth")
    .with_validator(CssProperty::BorderTopWidth);

    rules.add("divide", |_, value| {
        let r = Property::parse_string(
            CssProperty::Color,
            value.as_ref(),
            Default::default(),
        )
        .unwrap();
        if let Property::Color(a) = r {
            if let Ok(CssColor::RGBA(RGBA {
                red,
                green,
                blue,
                alpha,
            })) = a.to_rgb()
            {
                return css! {
                    "--tw-divide-opacity": alpha.to_string();
                    "border-color": format!("rgb({} {} {} / var(--tw-divide-opacity))", red, green, blue);
                };
            }
        }
        css! {
            "border-color": value.clone();
        }
    })
    .with_theme("colors")
    .with_validator(CssProperty::BorderColor)
    .with_modifier(ModifierProcessor {
        validator: Some(Box::new(CssProperty::Opacity)),
        allowed_values: Some(opacity.clone()),
    });

    rules
        .add("border", |meta, value| {
            // TODO: use color with opacity
            if let Some(opacity) = meta.modifier {
                return css! {
                    "border-color": value;
                    "border-opacity": opacity;
                };
            }
            css!("border-color": value;)
        })
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
                font_size.extend(
                    css!("line-height": line_height.clone().into_owned()),
                );
            }
            font_size
        })
        .with_theme("fontSize")
        .with_validator(CssProperty::FontSize)
        .with_modifier(ModifierProcessor {
            validator: Some(Box::new(CssProperty::LineHeight)),
            allowed_values: Some(line_height_map2),
        });

    add_theme_rule!(ctx, {
        "spacing" => {
            "m" => ["margin"]
            "mx" => ["margin-left", "margin-right"]
            "my" => ["margin-top", "margin-bottom"]
            "mt" => ["margin-top"]
            "mr" => ["margin-right"]
            "mb" => ["margin-bottom"]
            "ml" => ["margin-left"]
            "ms" => ["margin-inline-start"]
            "me" => ["margin-inline-end"]

            "p" => ["padding"]
            "px" => ["padding-left", "padding-right"]
            "py" => ["padding-top", "padding-bottom"]
            "pt" => ["padding-top"]
            "pr" => ["padding-right"]
            "pb" => ["padding-bottom"]
            "pl" => ["padding-left"]
            "ps" => ["padding-inline-start"]
            "pe" => ["padding-inline-end"]

            "inset" => ["top", "right", "bottom", "left"]
            "inset-x" => ["left", "right"]
            "inset-y" => ["top", "bottom"]

            "top" => ["top"]
            "right" => ["right"]
            "bottom" => ["bottom"]
            "left" => ["left"]

            "gap" => ["gap"]

            "w" => ["width"]
            "h" => ["height"]
            "size" => ["width", "height"]
        }
        "lineHeight" => {
            "leading" => ["line-height"]
        }
        "colors" => {
            "border" => ["border-color"]
            "border-x" => ["border-right-color", "border-left-color"]
            "border-y" => ["border-top-color", "border-bottom-color"]
            "border-s" => ["border-inline-start-color"]
            "border-e" => ["border-inline-end-color"]
            "border-t" => ["border-top-color"]
            "border-r" => ["border-right-color"]
            "border-b" => ["border-bottom-color"]
            "border-l" => ["border-left-color"]
        }
        "opacity" => {
            "opacity" => ["opacity"]
            "text" => ["text-opacity"]
            "bg" => ["background-opacity"]
            "border" => ["border-opacity"]
            "divide" => ["--tw-divide-opacity"]
        }
    });
}
