use std::ops::Deref;

use lightningcss::{
    properties::{Property, PropertyId},
    traits::IntoOwned,
    values::color::{CssColor, RGBA},
};

use crate::{
    context::{AddRule, Context},
    decls,
    rule::{Rule, RuleMatchingFn},
};

struct PendingRule<'i> {
    key: &'i str,
    theme_key: Option<&'i str>,
    property_id: Option<PropertyId<'i>>,
    rule: Rule<'i>,
}

impl<'i> Deref for PendingRule<'i> {
    type Target = Rule<'i>;

    fn deref(&self) -> &Self::Target {
        &self.rule
    }
}

impl<'c> PendingRule<'c> {
    fn add_to(mut self, ctx: &mut Context<'c>) {
        self.theme_key.map(|key| {
            self.rule.allowed_values = ctx
                .get_theme(key)
                .expect(&format!("theme key `{key}` not found"))
                .clone()
                .into();
        });

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

fn rule<'c>(
    key: &'c str,
    handler: impl RuleMatchingFn + 'static,
) -> PendingRule<'c> {
    PendingRule {
        key,
        rule: Rule::new(handler),
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
            Some(decls! {
                "display" => "-webkit-box",
                "-webkit-line-clamp" => value,
                "-webkit-box-orient" => "vertical",
                "overflow" => "hidden",
            })
        })
        .with_theme("lineClamp")
        .with_type(PropertyId::LineHeight)

        rule("border-spacing", |_, value| {
            Some(decls! {
                "--tw-border-spacing-x" => value.clone(),
                "--tw-border-spacing-y" => value.clone(),
                "border-spacing" => "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
            })
        })
        .with_theme("spacing")

        rule("border-spacing-x", |_, value| {
            Some(decls! {
                "--tw-border-spacing-x" => value,
                "border-spacing" => "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
            })
        })
        .with_theme("spacing")

        rule("border-spacing-y", |_, value| {
            Some(decls! {
                "--tw-border-spacing-y" => value,
                "border-spacing" => "var(--tw-border-spacing-x) var(--tw-border-spacing-y)",
            })
        })
        .with_theme("spacing")

        rule("animate", |_, value| {
            // TODO: generate keyframes
            Some(decls! {
                "animation" => value,
            })
        })

        // TODO: return a CSSRule with "& > :not([hidden]) ~ :not([hidden])"
        rule("space-x", |_, value| {
            Some(decls! {
                "--tw-space-x-reverse" => "0",
                "margin-right" => format!("calc({value} * var(--tw-space-x-reverse))"),
                "margin-left" =>  format!("calc({value} * calc(1 - var(--tw-space-x-reverse)))"),
            })
        })
        .with_theme("spacing")
        // .support_negative()

        rule("space-y", |_, value| {
            Some(decls! {
                "--tw-space-y-reverse" => "0",
                "margin-top" => format!("calc({value} * calc(1 - var(--tw-space-y-reverse)))"),
                "margin-bottom" => format!("calc({value} * var(--tw-space-y-reverse))"),
            })
        })
        .with_theme("spacing")
        // .support_negative()

        rule("divide-x", |_, value| {
            Some(decls! {
                "--tw-divide-x-reverse" => "0",
                "border-right-width" => format!("calc({value} * var(--tw-divide-x-reverse))"),
                "border-left-width" => format!("calc({value} * calc(1 - var(--tw-divide-x-reverse)))"),
            })
        })
        .with_theme("borderWidth")
        .with_type(PropertyId::BorderRightWidth)

        rule("divide-y", |_, value| {
            Some(decls! {
                "--tw-divide-y-reverse" => "0",
                "border-top-width" => format!("calc({value} * calc(1 - var(--tw-divide-y-reverse)))"),
                "border-bottom-width" => format!("calc({value} * var(--tw-divide-y-reverse))"),
            })
        })
        .with_theme("borderWidth")
        .with_type(PropertyId::BorderTopWidth)

        rule("divide", |_, value| {
            // TODO: check corePlugins.divideOpacity
            let r = Property::parse_string(PropertyId::Color, value.as_ref(), Default::default()).ok()?;
            if let Property::Color(a) = r {
                if let Ok(CssColor::RGBA(RGBA { red, green, blue, alpha })) = a.to_rgb() {
                    return Some(decls! {
                        "--tw-divide-opacity" => alpha.to_string(),
                        "border-color" => format!("rgb({} {} {} / var(--tw-divide-opacity))", red, green, blue),
                    });
                }
            }
            Some(decls! {
                "border-color" => value.clone(),
            })
        })

        rule("divide-opacity", |_, value| {
            Some(decls! {
                "--tw-divide-opacity" => value,
            })
        })
        .with_theme("opacity")

        rule("border-x", |_, value| {
            Some(decls! {
                "border-right-color" => value.clone(),
                "border-left-color" => value.clone(),
            })
        })
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("border-y", |_, value| {
            Some(decls! {
                "border-top-color" => value.clone(),
                "border-bottom-color" => value.clone(),
            })
        })
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("border-s", |_, value| {
            Some(decls! {
                "border-inline-start-color" => value,
            })
        })
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("border-e", |_, value| {
            Some(decls! {
                "border-inline-end-color" => value,
            })
        })
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("border-t", |_, value| {
            Some(decls! {
                "border-top-color" => value,
            })
        })
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("border-r", |_, value| {
            Some(decls! {
                "border-right-color" => value,
            })
        })
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("border-b", |_, value| {
            Some(decls! {
                "border-bottom-color" => value,
            })
        })
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("border-l", |_, value| {
            Some(decls! {
                "border-left-color" => value,
            })
        })
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("bg", |_, value| {
            Some(decls! {
                "background-color" => value,
            })
        })
        .with_theme("colors")
        .with_type(PropertyId::Color)

        rule("bg", |_, value| {
            Some(decls! {
                "background-position" => value,
            })
        })
        .with_theme("backgroundPosition")
        .with_type(PropertyId::BackgroundPosition)

        rule("bg", |_, value| {
            Some(decls! {
                "background-size" => value,
            })
        })
        .with_theme("backgroundSize")
        .with_type(PropertyId::BackgroundSize)

        rule("bg", |_, value| {
            Some(decls! {
                "background-image" => value,
            })
        })
        .with_theme("backgroundImage")
        .with_type(PropertyId::BackgroundImage)

        rule("bg-opacity", |_, value| {
            Some(decls! {
                "--tw-bg-opacity" => value,
            })
        })
        .with_theme("opacity")
        .with_type(PropertyId::Opacity)


    }
}
