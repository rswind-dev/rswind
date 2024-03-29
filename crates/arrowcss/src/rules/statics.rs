use crate::{css::CssDecls, static_rules};
use lazy_static::lazy_static;

pub fn accessibility<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
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

pub fn pointer_events<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "pointer-events-none" => { "pointer-events": "none"; }
        "pointer-events-auto" => { "pointer-events": "auto"; }
    }
}

pub fn visibility<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "visible" => { "visibility": "visible"; }
        "invisible" => { "visibility": "hidden"; }
        "collapse" => { "visibility": "collapse"; }
    }
}

pub fn position<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "static" => { "position": "static"; }
        "fixed" => { "position": "fixed"; }
        "absolute" => { "position": "absolute"; }
        "relative" => { "position": "relative"; }
        "sticky" => { "position": "sticky"; }
    }
}

pub fn isolation<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "isolate" => { "isolation": "isolate"; }
        "isolation-auto" => { "isolation": "auto"; }
    }
}

pub fn float<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "float-right" => { "float": "right"; }
        "float-left" => { "float": "left"; }
        "float-none" => { "float": "none"; }
    }
}

pub fn clear<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "clear-start" => { "clear": "inline-start"; }
        "clear-end" => { "clear": "inline-end"; }
        "clear-left" => { "clear": "left"; }
        "clear-right" => { "clear": "right"; }
        "clear-both" => { "clear": "both"; }
        "clear-none" => { "clear": "none"; }
    }
}

pub fn box_sizing<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "box-border" => { "box-sizing": "border-box"; }
        "box-content" => { "box-sizing": "content-box"; }
    }
}

pub fn display<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
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

pub fn table_layout<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "table-auto" => { "table-layout": "auto"; }
        "table-fixed" => { "table-layout": "fixed"; }
    }
}

pub fn caption_side<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "caption-top" => { "caption-side": "top"; }
        "caption-bottom" => { "caption-side": "bottom"; }
    }
}

pub fn border_collapse<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "border-collapse" => { "border-collapse": "collapse"; }
        "border-separate" => { "border-collapse": "separate"; }
    }
}

pub fn user_select<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "select-none" => { "user-select": "none"; }
        "select-text" => { "user-select": "text"; }
        "select-all" => { "user-select": "all"; }
        "select-auto" => { "user-select": "auto"; }
    }
}

pub fn resize<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "resize-none" => { "resize": "none"; }
        "resize-y" => { "resize": "vertical"; }
        "resize-x" => { "resize": "horizontal"; }
        "resize" => { "resize": "both"; }
    }
}

pub fn scroll_snap_align<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "snap-start" => { "scroll-snap-align": "start"; }
        "snap-end" => { "scroll-snap-align": "end"; }
        "snap-center" => { "scroll-snap-align": "center"; }
        "snap-align-none" => { "scroll-snap-align": "none"; }
    }
}

pub fn scroll_snap_stop<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "snap-normal" => { "scroll-snap-stop": "normal"; }
        "snap-always" => { "scroll-snap-stop": "always"; }
    }
}

pub fn list_style_position<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "list-inside" => { "list-style-position": "inside"; }
        "list-outside" => { "list-style-position": "outside"; }
    }
}

pub fn appearance<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "appearance-none" => { "appearance": "none"; }
        "appearance-auto" => { "appearance": "auto"; }
    }
}

pub fn break_before<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
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

pub fn break_inside<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "break-inside-auto" => { "break-inside": "auto"; }
        "break-inside-avoid" => { "break-inside": "avoid"; }
        "break-inside-avoid-page" => { "break-inside": "avoid-page"; }
        "break-inside-avoid-column" => { "break-inside": "avoid-column"; }

        // new
        "break-inside-avoid-region" => { "break-inside": "avoid-region"; }
    }
}

pub fn break_after<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
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

pub fn grid_auto_flow<'a>() -> Vec<(&'static str, CssDecls<'a>)> {
    static_rules! {
        "grid-flow-row" => { "grid-auto-flow": "row"; }
        "grid-flow-col" => { "grid-auto-flow": "column"; }
        "grid-flow-dense" => { "grid-auto-flow": "dense"; }
        "grid-flow-row-dense" => { "grid-auto-flow": "row dense"; }
        "grid-flow-col-dense" => { "grid-auto-flow": "column dense"; }
    }
}

pub fn get_all() -> Vec<(&'static str, CssDecls<'static>)> {
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
        .collect()
}

lazy_static! {
    pub static ref STATIC_RULES: Vec<(&'static str, CssDecls<'static>)> =
        get_all();
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
