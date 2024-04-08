use lightningcss::{
    properties::{Property, PropertyId},
    traits::IntoOwned,
    values::color::{CssColor, RGBA},
};

use arrowcss_css_macro::css;

use crate::{
    add_theme_rule,
    context::{AddRule, Context},
    process::{ModifierProcessor, UtilityHandler, UtilityProcessor},
    types::{CssDataType, TypeValidator},
};

struct PendingRule<'i, 'c> {
    key: &'i str,
    theme_key: Option<&'i str>,
    handler: UtilityHandler,
    modifier: Option<ModifierProcessor<'c>>,
    validator: Option<Box<dyn TypeValidator>>,
    supports_negative: bool,
    supports_fraction: bool,
    ctx: &'i mut Context<'c>,
}

impl<'i, 'c> PendingRule<'i, 'c> {
    fn with_theme(mut self, key: &'i str) -> Self {
        self.theme_key = Some(key);
        self
    }

    fn support_negative(mut self) -> Self {
        self.supports_negative = true;
        self
    }

    #[allow(dead_code)]
    fn support_fraction(mut self) -> Self {
        self.supports_fraction = true;
        self
    }

    fn with_modifier(mut self, modifier: ModifierProcessor<'c>) -> Self {
        self.modifier = Some(modifier);
        self
    }

    fn with_validator(
        mut self,
        validator: impl TypeValidator + 'static,
    ) -> Self {
        self.validator = Some(Box::new(validator));
        self
    }
}

/// Automatically adds the rule to the context when dropped.
/// This is useful for defining rules in a more declarative way.
impl<'i, 'c> Drop for PendingRule<'i, 'c> {
    fn drop(&mut self) {
        let allowed_values = self.theme_key.map(|key| {
            self.ctx
                .get_theme(key)
                .unwrap_or_else(|| panic!("theme key `{key}` not found"))
                .clone()
        });
        let validator = std::mem::take(&mut self.validator);
        let handler = std::mem::take(&mut self.handler);
        let modifier = std::mem::take(&mut self.modifier);

        self.ctx.add_rule(
            self.key,
            UtilityProcessor {
                validator,
                allowed_values,
                handler,
                modifier,
                supports_negative: self.supports_negative,
                supports_fraction: self.supports_fraction,
            },
        );
    }
}

pub fn load_dynamic_rules(ctx: &mut Context<'_>) {
    macro_rules! add_rule {
        ($key:expr, $handler:expr) => {
            PendingRule {
                key: $key,
                handler: UtilityHandler::Dynamic(Box::new($handler)),
                ctx,
                theme_key: None,
                supports_negative: false,
                supports_fraction: false,
                modifier: None,
                validator: None,
            }
        };
    }

    add_rule!("line-clamp", |_, value| {
        css! {
            "display": "-webkit-box";
            "-webkit-line-clamp": value;
            "-webkit-box-orient": "vertical";
            "overflow": "hidden";
        }
    })
    .with_validator(CssDataType::Number)
    .with_theme("lineClamp");

    add_rule!("border-spacing", |_, value| {
            css! {
                "--tw-border-spacing-x": value.clone();
                "--tw-border-spacing-y": value.clone();
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_theme("spacing");
    // theme: for additional required properties
    // modifier;
    // options: feature flag
    add_rule!("border-spacing-x", |_, value| {
            css! {
                "--tw-border-spacing-x": value;
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_theme("spacing");

    add_rule!("border-spacing-y", |_, value| {
            css! {
                "--tw-border-spacing-y": value;
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_theme("spacing");

    add_rule!("animate", |_, value| {
        css! {
            "animation": value;
        }
    })
    .with_theme("animate");
    // TODO: return a CSSRule with "& > :not([hidden]) ~ :not([hidden])"
    add_rule!("space-x", |_, value| {
            css! {
                "& > :not([hidden]) ~ :not([hidden])" {
                    "--tw-space-x-reverse": "0";
                    "margin-right": format!("calc({value} * var(--tw-space-x-reverse))");
                    "margin-left":  format!("calc({value} * calc(1 - var(--tw-space-x-reverse)))");
                }
            }
        })
        .with_theme("spacing")
        .support_negative();

    add_rule!("space-y", |_, value| {
            css! {
                "--tw-space-y-reverse": "0";
                "margin-top": format!("calc({value} * calc(1 - var(--tw-space-y-reverse)))");
                "margin-bottom": format!("calc({value} * var(--tw-space-y-reverse))");
            }
        })
        .with_theme("spacing")
        .support_negative();

    add_rule!("divide-x", |_, value| {
            css! {
                "--tw-divide-x-reverse": "0";
                "border-right-width": format!("calc({value} * var(--tw-divide-x-reverse))");
                "border-left-width": format!("calc({value} * calc(1 - var(--tw-divide-x-reverse)))");
            }
        })
        .with_theme("borderWidth")
        .with_validator(PropertyId::BorderRightWidth);

    add_rule!("divide-y", |_, value| {
            css! {
                "--tw-divide-y-reverse": "0";
                "border-top-width": format!("calc({value} * calc(1 - var(--tw-divide-y-reverse)))");
                "border-bottom-width": format!("calc({value} * var(--tw-divide-y-reverse))");
            }
        })
        .with_theme("borderWidth")
        .with_validator(PropertyId::BorderTopWidth);

    add_rule!("divide", |_, value| {
        // TODO: check corePlugins.divideOpacity
        let r = Property::parse_string(
            PropertyId::Color,
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
    });

    add_rule!("bg", |_, value| css!("background-color": value))
        .with_theme("colors")
        .with_validator(PropertyId::Color);

    add_rule!("bg", |_, value| css!("background-position": value))
        .with_theme("backgroundPosition")
        .with_validator(PropertyId::BackgroundPosition);

    add_rule!("bg", |_, value| css!("background-size": value))
        .with_theme("backgroundSize")
        .with_validator(PropertyId::BackgroundSize);

    add_rule!("bg", |_, value| css!("background-image": value))
        .with_theme("backgroundImage")
        .with_validator(PropertyId::BackgroundImage);

    add_rule!("text", |_, value| css!("color": value))
        .with_theme("colors")
        .with_validator(PropertyId::Color);

    let line_height_map = ctx.get_theme("fontSize:lineHeight").unwrap();
    let line_height_map2 = ctx.get_theme("lineHeight").unwrap();
    add_rule!("text", move |meta, value| {
        let mut font_size = css!("font-size": value.clone());
        if let Some(modifier) = meta.modifier {
            font_size.extend(css!("line-height": modifier));
        } else if let Some(line_height) = meta
            .candidate
            .value
            .take_named()
            .and_then(|v| line_height_map.get(v))
        {
            font_size
                .extend(css!("line-height": line_height.clone().into_owned()));
        }
        font_size
    })
    .with_theme("fontSize")
    .with_validator(PropertyId::FontSize)
    .with_modifier(ModifierProcessor {
        validator: Some(Box::new(PropertyId::LineHeight)),
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
