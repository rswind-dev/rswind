use crate::{context::Context, css::DeclList, static_rules};

pub fn accessibility<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "sr-only" => {
            "position": "absolute";
            "width": "1px";
            "height": "1px";
            "padding": "0";
            "margin": "-1px";
            "overflow": "hidden";
            "clip": "rect(0, 0, 0, 0)";
            "white-space": "nowrap";
            "border-width": "0";
        }
        "not-sr-only" => {
            "position": "static";
            "width": "auto";
            "height": "auto";
            "padding": "0";
            "margin": "0";
            "overflow": "visible";
            "clip": "auto";
            "white-space": "normal";
        }
    }
}

pub fn pointer_events<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "pointer-events-none" => { "pointer-events": "none"; }
        "pointer-events-auto" => { "pointer-events": "auto"; }
    }
}

pub fn visibility<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "visible" => { "visibility": "visible"; }
        "invisible" => { "visibility": "hidden"; }
        "collapse" => { "visibility": "collapse"; }
    }
}

pub fn position<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "static" => { "position": "static"; }
        "fixed" => { "position": "fixed"; }
        "absolute" => { "position": "absolute"; }
        "relative" => { "position": "relative"; }
        "sticky" => { "position": "sticky"; }
    }
}

pub fn isolation<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "isolate" => { "isolation": "isolate"; }
        "isolation-auto" => { "isolation": "auto"; }
    }
}

pub fn float<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "float-right" => { "float": "right"; }
        "float-left" => { "float": "left"; }
        "float-none" => { "float": "none"; }
    }
}

pub fn clear<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "clear-start" => { "clear": "inline-start"; }
        "clear-end" => { "clear": "inline-end"; }
        "clear-left" => { "clear": "left"; }
        "clear-right" => { "clear": "right"; }
        "clear-both" => { "clear": "both"; }
        "clear-none" => { "clear": "none"; }
    }
}

pub fn box_sizing<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "box-border" => { "box-sizing": "border-box"; }
        "box-content" => { "box-sizing": "content-box"; }
    }
}

pub fn display<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "block" => { "display": "block"; }
        "inline-block" => { "display": "inline-block"; }
        "flex" => { "display": "flex"; }
        "inline-flex" => { "display": "inline-flex"; }
        "table" => { "display": "table"; }
        "inline-table" => { "display": "inline-table"; }
        "table-caption" => { "display": "table-caption"; }
        "table-cell" => { "display": "table-cell"; }
        "table-column" => { "display": "table-column"; }
        "table-column-group" => { "display": "table-column-group"; }
        "table-footer-group" => { "display": "table-footer-group"; }
        "table-header-group" => { "display": "table-header-group"; }
        "table-row-group" => { "display": "table-row-group"; }
        "table-row" => { "display": "table-row"; }
        "flow-root" => { "display": "flow-root"; }
        "grid" => { "display": "grid"; }
        "inline-grid" => { "display": "inline-grid"; }
        "contents" => { "display": "contents"; }
        "list-item" => { "display": "list-item"; }
        "hidden" => { "display": "none"; }
    }
}

pub fn table_layout<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "table-auto" => { "table-layout": "auto"; }
        "table-fixed" => { "table-layout": "fixed"; }
    }
}

pub fn caption_side<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "caption-top" => { "caption-side": "top"; }
        "caption-bottom" => { "caption-side": "bottom"; }
    }
}

pub fn border_collapse<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "border-collapse" => { "border-collapse": "collapse"; }
        "border-separate" => { "border-collapse": "separate"; }
    }
}

pub fn user_select<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "select-none" => { "user-select": "none"; }
        "select-text" => { "user-select": "text"; }
        "select-all" => { "user-select": "all"; }
        "select-auto" => { "user-select": "auto"; }
    }
}

pub fn resize<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "resize-none" => { "resize": "none"; }
        "resize-y" => { "resize": "vertical"; }
        "resize-x" => { "resize": "horizontal"; }
        "resize" => { "resize": "both"; }
    }
}

pub fn scroll_snap_align<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "snap-start" => { "scroll-snap-align": "start"; }
        "snap-end" => { "scroll-snap-align": "end"; }
        "snap-center" => { "scroll-snap-align": "center"; }
        "snap-align-none" => { "scroll-snap-align": "none"; }
    }
}

pub fn scroll_snap_stop<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "snap-normal" => { "scroll-snap-stop": "normal"; }
        "snap-always" => { "scroll-snap-stop": "always"; }
    }
}

pub fn list_style_position<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "list-inside" => { "list-style-position": "inside"; }
        "list-outside" => { "list-style-position": "outside"; }
    }
}

pub fn appearance<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "appearance-none" => { "appearance": "none"; }
        "appearance-auto" => { "appearance": "auto"; }
    }
}

pub fn break_before<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "break-before-auto" => { "break-before": "auto"; }
        "break-before-avoid" => { "break-before": "avoid"; }
        "break-before-always" => { "break-before": "always"; }
        "break-before-all" => { "break-before": "all"; }
        "break-before-avoid-page" => { "break-before": "avoid-page"; }
        "break-before-page" => { "break-before": "page"; }
        "break-before-left" => { "break-before": "left"; }
        "break-before-right" => { "break-before": "right"; }
        "break-before-recto" => { "break-before": "recto"; }
        "break-before-verso" => { "break-before": "verso"; }
        "break-before-avoid-column" => { "break-before": "avoid-column"; }
        "break-before-column" => { "break-before": "column"; }
        "break-before-avoid-region" => { "break-before": "avoid-region"; }
        "break-before-region" => { "break-before": "region"; }
    }
}

pub fn break_inside<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "break-inside-auto" => { "break-inside": "auto"; }
        "break-inside-avoid" => { "break-inside": "avoid"; }
        "break-inside-avoid-page" => { "break-inside": "avoid-page"; }
        "break-inside-avoid-column" => { "break-inside": "avoid-column"; }

        // new
        "break-inside-avoid-region" => { "break-inside": "avoid-region"; }
    }
}

pub fn break_after<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "break-after-auto" => { "break-after": "auto"; }
        "break-after-avoid" => { "break-after": "avoid"; }
        "break-after-all" => { "break-after": "all"; }
        "break-after-avoid-page" => { "break-after": "avoid-page"; }
        "break-after-page" => { "break-after": "page"; }
        "break-after-left" => { "break-after": "left"; }
        "break-after-right" => { "break-after": "right"; }
        "break-after-column" => { "break-after": "column"; }

        // new
        "break-after-always" => { "break-after": "always"; }
        "break-after-recto" => { "break-after": "recto"; }
        "break-after-verso" => { "break-after": "verso"; }
        "break-after-avoid-column" => { "break-after": "avoid-column"; }
        "break-after-avoid-region" => { "break-after": "avoid-region"; }
        "break-after-region" => { "break-after": "region"; }
    }
}

pub fn grid_auto_flow<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "grid-flow-row" => { "grid-auto-flow": "row"; }
        "grid-flow-col" => { "grid-auto-flow": "column"; }
        "grid-flow-dense" => { "grid-auto-flow": "dense"; }
        "grid-flow-row-dense" => { "grid-auto-flow": "row dense"; }
        "grid-flow-col-dense" => { "grid-auto-flow": "column dense"; }
    }
}

pub fn flex<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "flex-row" => { "flex-direction": "row"; }
        "flex-row-reverse" => { "flex-direction": "row-reverse"; }
        "flex-col" => { "flex-direction": "column"; }
        "flex-col-reverse" => { "flex-direction": "column-reverse"; }
        "flex-wrap" => { "flex-wrap": "wrap"; }
        "flex-wrap-reverse" => { "flex-wrap": "wrap-reverse"; }
        "flex-nowrap" => { "flex-wrap": "nowrap"; }
    }
}

pub fn place_content<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "place-content-center" => { "place-content": "center"; }
        "place-content-start" => { "place-content": "start"; }
        "place-content-end" => { "place-content": "end"; }
        "place-content-space-between" => { "place-content": "space-between"; }
        "place-content-space-around" => { "place-content": "space-around"; }
        "place-content-space-evenly" => { "place-content": "space-evenly"; }
        "place-content-baseline" => { "place-content": "baseline"; }
        "place-content-stretch" => { "place-content": "stretch"; }
    }
}

pub fn place_items<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "place-items-center" => { "place-items": "center"; }
        "place-items-start" => { "place-items": "start"; }
        "place-items-end" => { "place-items": "end"; }
        "place-items-baseline" => { "place-items": "baseline"; }
        "place-items-stretch" => { "place-items": "stretch"; }
    }
}

pub fn align_content<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "content-normal" => { "align-content": "normal"; }
        "content-center" => { "align-content": "center"; }
        "content-start" => { "align-content": "start"; }
        "content-end" => { "align-content": "end"; }
        "content-between" => { "align-content": "space-between"; }
        "content-around" => { "align-content": "space-around"; }
        "content-evenly" => { "align-content": "space-evenly"; }
        "content-baseline" => { "align-content": "baseline"; }
        "content-stretch" => { "align-content": "stretch"; }
    }
}

pub fn align_items<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "items-center" => { "align-items": "center"; }
        "items-start" => { "align-items": "start"; }
        "items-end" => { "align-items": "end"; }
        "items-baseline" => { "align-items": "baseline"; }
        "items-stretch" => { "align-items": "stretch"; }
    }
}

pub fn justify_content<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "justify-normal" => { "justify-content": "normal"; }
        "justify-center" => { "justify-content": "center"; }
        "justify-start" => { "justify-content": "start"; }
        "justify-end" => { "justify-content": "end"; }
        "justify-between" => { "justify-content": "space-between"; }
        "justify-around" => { "justify-content": "space-around"; }
        "justify-evenly" => { "justify-content": "space-evenly"; }
        "justify-baseline" => { "justify-content": "baseline"; }
        "justify-stretch" => { "justify-content": "stretch"; }
    }
}

pub fn justify_items<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "items-normal" => { "justify-items": "normal"; }
        "items-center" => { "justify-items": "center"; }
        "items-start" => { "justify-items": "start"; }
        "items-end" => { "justify-items": "end"; }
        "items-stretch" => { "justify-items": "stretch"; }
    }
}

pub fn object_fit<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "object-contain" => { "object-fit": "contain"; }
        "object-cover" => { "object-fit": "cover"; }
        "object-fill" => { "object-fit": "fill"; }
        "object-none" => { "object-fit": "none"; }
        "object-scale-down" => { "object-fit": "scale-down"; }
    }
}

pub fn object_position<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "object-bottom" => { "object-position": "bottom"; }
        "object-center" => { "object-position": "center"; }
        "object-left" => { "object-position": "left"; }
        "object-left-bottom" => { "object-position": "left bottom"; }
        "object-left-top" => { "object-position": "left top"; }
        "object-right" => { "object-position": "right"; }
        "object-right-bottom" => { "object-position": "right bottom"; }
        "object-right-top" => { "object-position": "right top"; }
        "object-top" => { "object-position": "top"; }
    }
}

pub fn text_align<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "text-left" => { "text-align": "left"; }
        "text-center" => { "text-align": "center"; }
        "text-right" => { "text-align": "right"; }
        "text-justify" => { "text-align": "justify"; }
        "text-start" => { "text-align": "start"; }
        "text-end" => { "text-align": "end"; }
    }
}

pub fn vertical_align<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "align-baseline" => { "vertical-align": "baseline"; }
        "align-top" => { "vertical-align": "top"; }
        "align-middle" => { "vertical-align": "middle"; }
        "align-bottom" => { "vertical-align": "bottom"; }
        "align-text-top" => { "vertical-align": "text-top"; }
        "align-text-bottom" => { "vertical-align": "text-bottom"; }
        "align-sub" => { "vertical-align": "sub"; }
        "align-super" => { "vertical-align": "super"; }
    }
}

pub fn text_transform<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "uppercase" => { "text-transform": "uppercase"; }
        "lowercase" => { "text-transform": "lowercase"; }
        "capitalize" => { "text-transform": "capitalize"; }
        "normal-case" => { "text-transform": "none"; }
    }
}

pub fn text_italic<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "italic" => { "font-style": "italic"; }
        "not-italic" => { "font-style": "normal"; }
    }
}

pub fn font_stretch<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "font-stretch-normal" => { "font-stretch": "normal"; }
        "font-stretch-ultra-condensed" => { "font-stretch": "ultra-condensed"; }
        "font-stretch-extra-condensed" => { "font-stretch": "extra-condensed"; }
        "font-stretch-condensed" => { "font-stretch": "condensed"; }
        "font-stretch-semi-condensed" => { "font-stretch": "semi-condensed"; }
        "font-stretch-semi-expanded" => { "font-stretch": "semi-expanded"; }
        "font-stretch-expanded" => { "font-stretch": "expanded"; }
        "font-stretch-extra-expanded" => { "font-stretch": "extra-expanded"; }
        "font-stretch-ultra-expanded" => { "font-stretch": "ultra-expanded"; }
    }
}

pub fn text_decoration<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "underline" => { "text-decoration": "underline"; }
        "overline" => { "text-decoration": "overline"; }
        "line-through" => { "text-decoration": "line-through"; }
        "no-underline" => { "text-decoration": "none"; }
    }
}

pub fn text_decoration_style<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "decoration-solid" => { "text-decoration-style": "solid"; }
        "decoration-double" => { "text-decoration-style": "double"; }
        "decoration-dotted" => { "text-decoration-style": "dotted"; }
        "decoration-dashed" => { "text-decoration-style": "dashed"; }
        "decoration-wavy" => { "text-decoration-style": "wavy"; }
    }
}

pub fn text_decoration_thickness<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "decoration-auto" => { "text-decoration-thickness": "auto"; }
        "decoration-from-font" => { "text-decoration-thickness": "from-font"; }
    }
}

pub fn animate<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "animate-none" => { "animation": "none"; }
    }
}

pub fn filter<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "filter" => { "filter": "var(--tw-blur,) var(--tw-brightness,) var(--tw-contrast,) var(--tw-grayscale,) var(--tw-hue-rotate,) var(--tw-invert,) var(--tw-saturate,) var(--tw-sepia,) var(--tw-drop-shadow,)"; }
        "filter-none" => { "filter": "none"; }
    }
}

pub fn backdrop_filter<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "backdrop-filter" => {
            "-webkit-backdrop-filter": "var(--tw-backdrop-blur,) var(--tw-backdrop-brightness,) var(--tw-backdrop-contrast,) var(--tw-backdrop-grayscale,) var(--tw-backdrop-hue-rotate,) var(--tw-backdrop-invert,) var(--tw-backdrop-opacity,) var(--tw-backdrop-saturate,) var(--tw-backdrop-sepia,)";
            "backdrop-filter": "var(--tw-backdrop-blur,) var(--tw-backdrop-brightness,) var(--tw-backdrop-contrast,) var(--tw-backdrop-grayscale,) var(--tw-backdrop-hue-rotate,) var(--tw-backdrop-invert,) var(--tw-backdrop-opacity,) var(--tw-backdrop-saturate,) var(--tw-backdrop-sepia,)";
        }
        "backdrop-filter-none" => {
            "-webkit-backdrop-filter": "none";
            "backdrop-filter": "none";
        }
    }
}

pub fn get_all() -> Vec<(&'static str, DeclList<'static>)> {
    vec![]
        .into_iter()
        .chain(accessibility())
        .chain(pointer_events())
        .chain(visibility())
        .chain(position())
        .chain(isolation())
        .chain(float())
        .chain(clear())
        .chain(box_sizing())
        .chain(display())
        .chain(table_layout())
        .chain(caption_side())
        .chain(border_collapse())
        .chain(user_select())
        .chain(resize())
        .chain(scroll_snap_align())
        .chain(scroll_snap_stop())
        .chain(list_style_position())
        .chain(appearance())
        .chain(break_before())
        .chain(break_inside())
        .chain(break_after())
        .chain(grid_auto_flow())
        .chain(flex())
        .chain(place_content())
        .chain(place_items())
        .chain(align_content())
        .chain(align_items())
        .chain(justify_content())
        .chain(justify_items())
        .chain(object_fit())
        .chain(object_position())
        .chain(text_align())
        .chain(vertical_align())
        .chain(text_transform())
        .chain(text_italic())
        .chain(font_stretch())
        .chain(text_decoration())
        .collect()
}

pub fn load_static_utilities(ctx: &mut Context) {
    get_all().into_iter().for_each(|(key, value)| {
        ctx.add_static((key, value));
    });
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let rules = display();
        assert_eq!(rules.len(), 4);
        assert_eq!(rules[0].0, "block");
        assert_eq!(rules[1].0, "inline-block");
        assert_eq!(rules[2].0, "flex");
        assert_eq!(rules[3].0, "inline-flex");
    }
}
