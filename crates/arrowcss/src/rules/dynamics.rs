use std::ops::Deref;

use arrowcss_css_macro::css;
use lightningcss::{
    properties::{Property, PropertyId},
    traits::IntoOwned,
    values::{
        color::{CssColor, RGBA},
        string::CowArcStr,
    },
};

use crate::{
    add_theme_rule,
    context::{AddRule, Context},
    css::NodeList,
    rule::{MetaData, Utility},
};

struct PendingRule<'i> {
    key: &'i str,
    theme_key: Option<&'i str>,
    property_id: Option<PropertyId<'i>>,
    rule: Utility<'i>,
}

impl<'i> Deref for PendingRule<'i> {
    type Target = Utility<'i>;

    fn deref(&self) -> &Self::Target {
        &self.rule
    }
}

impl<'c> PendingRule<'c> {
    fn add_to(mut self, ctx: &mut Context<'c>) {
        if let Some(key) = self.theme_key {
            self.rule.allowed_values = ctx
                .get_theme(key)
                .unwrap_or_else(|| panic!("theme key `{key}` not found"))
                .clone()
                .into();
        }

        if let Some(id) = self.property_id {
            self.rule.infer_property_id = Some(Box::new(id.into_owned()));
        }

        ctx.add_rule(self.key, self.rule);
    }

    fn with_theme(mut self, key: &'c str) -> Self {
        self.theme_key = Some(key);
        self
    }

    fn with_type<'a: 'c>(mut self, property_id: PropertyId<'a>) -> Self {
        self.property_id = Some(property_id);
        self
    }
}

fn rule(
    key: &str,
    handler: fn(MetaData, CowArcStr) -> NodeList,
) -> PendingRule<'_> {
    PendingRule {
        key,
        rule: Utility::new(handler),
        theme_key: None,
        property_id: None,
    }
}

macro_rules! add_rules {
    ($ctx:expr => $($rule:expr)*) => {
        $(
            $rule.add_to($ctx);
        )*
    };
}

pub fn load_dynamic_rules(ctx: &mut Context) {
    add_rules! { ctx =>
        rule("line-clamp", |_, value| {
            css! {
                "display": "-webkit-box";
                "-webkit-line-clamp": value;
                "-webkit-box-orient": "vertical";
                "overflow": "hidden";
            }
        })
        .with_theme("lineClamp")
        .with_type(PropertyId::LineHeight)

        rule("border-spacing", |_, value| {
            css! {
                "--tw-border-spacing-x": value.clone();
                "--tw-border-spacing-y": value.clone();
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_theme("spacing")

        rule("border-spacing-x", |_, value| {
            css! {
                "--tw-border-spacing-x": value;
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_theme("spacing")

        rule("border-spacing-y", |_, value| {
            css! {
                "--tw-border-spacing-y": value;
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_theme("spacing")

        rule("animate", |_, value| {
            css! {
                "animation": value;
            }
        })
        .with_theme("animate")

        // TODO: return a CSSRule with "& > :not([hidden]) ~ :not([hidden])"
        rule("space-x", |_, value| {
            css! {
                "& > :not([hidden]) ~ :not([hidden])" {
                    "--tw-space-x-reverse": "0";
                    "margin-right": format!("calc({value} * var(--tw-space-x-reverse))");
                    "margin-left":  format!("calc({value} * calc(1 - var(--tw-space-x-reverse)))");
                }
            }
        })
        .with_theme("spacing")
        // .support_negative()

        rule("space-y", |_, value| {
            css! {
                "--tw-space-y-reverse": "0";
                "margin-top": format!("calc({value} * calc(1 - var(--tw-space-y-reverse)))");
                "margin-bottom": format!("calc({value} * var(--tw-space-y-reverse))");
            }
        })
        .with_theme("spacing")
        // .support_negative()

        rule("divide-x", |_, value| {
            css! {
                "--tw-divide-x-reverse": "0";
                "border-right-width": format!("calc({value} * var(--tw-divide-x-reverse))");
                "border-left-width": format!("calc({value} * calc(1 - var(--tw-divide-x-reverse)))");
            }
        })
        .with_theme("borderWidth")
        .with_type(PropertyId::BorderRightWidth)

        rule("divide-y", |_, value| {
            css! {
                "--tw-divide-y-reverse": "0";
                "border-top-width": format!("calc({value} * calc(1 - var(--tw-divide-y-reverse)))");
                "border-bottom-width": format!("calc({value} * var(--tw-divide-y-reverse))");
            }
        })
        .with_theme("borderWidth")
        .with_type(PropertyId::BorderTopWidth)

        rule("divide", |_, value| {
            // TODO: check corePlugins.divideOpacity
            let r = Property::parse_string(PropertyId::Color, value.as_ref(), Default::default()).unwrap();
            if let Property::Color(a) = r {
                if let Ok(CssColor::RGBA(RGBA { red, green, blue, alpha })) = a.to_rgb() {
                    return css! {
                        "--tw-divide-opacity": alpha.to_string();
                        "border-color": format!("rgb({} {} {} / var(--tw-divide-opacity))", red, green, blue);
                    }
                }
            }
            css! {
                "border-color": value.clone();
            }
        })

        rule("bg", |_, value| css!("background-color": value))
            .with_theme("colors")
            .with_type(PropertyId::Color)

        rule("bg", |_, value| css!("background-position": value))
            .with_theme("backgroundPosition")
            .with_type(PropertyId::BackgroundPosition)

        rule("bg", |_, value| css!("background-size": value))
            .with_theme("backgroundSize")
            .with_type(PropertyId::BackgroundSize)

        rule("bg", |_, value| css!("background-image": value))
            .with_theme("backgroundImage")
            .with_type(PropertyId::BackgroundImage)

        rule("text", |_, value| css!("color": value))
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("text", |_, value| css!("font-size": value))
        .with_theme("fontSize")
        .with_type(PropertyId::FontSize)
    }

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
