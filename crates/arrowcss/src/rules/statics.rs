use crate::{context::Context, css::DeclList, static_rules};

fn accessibility<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn pointer_events<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "pointer-events-none" => { "pointer-events": "none"; }
        "pointer-events-auto" => { "pointer-events": "auto"; }
    }
}

fn visibility<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "visible" => { "visibility": "visible"; }
        "invisible" => { "visibility": "hidden"; }
        "collapse" => { "visibility": "collapse"; }
    }
}

fn position<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "static" => { "position": "static"; }
        "fixed" => { "position": "fixed"; }
        "absolute" => { "position": "absolute"; }
        "relative" => { "position": "relative"; }
        "sticky" => { "position": "sticky"; }
    }
}

fn isolation<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "isolate" => { "isolation": "isolate"; }
        "isolation-auto" => { "isolation": "auto"; }
    }
}

fn float<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "float-right" => { "float": "right"; }
        "float-left" => { "float": "left"; }
        "float-none" => { "float": "none"; }
    }
}

fn clear<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "clear-start" => { "clear": "inline-start"; }
        "clear-end" => { "clear": "inline-end"; }
        "clear-left" => { "clear": "left"; }
        "clear-right" => { "clear": "right"; }
        "clear-both" => { "clear": "both"; }
        "clear-none" => { "clear": "none"; }
    }
}

fn box_sizing<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "box-border" => { "box-sizing": "border-box"; }
        "box-content" => { "box-sizing": "content-box"; }
    }
}

fn display<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn table_layout<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "table-auto" => { "table-layout": "auto"; }
        "table-fixed" => { "table-layout": "fixed"; }
    }
}

fn caption_side<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "caption-top" => { "caption-side": "top"; }
        "caption-bottom" => { "caption-side": "bottom"; }
    }
}

fn border_collapse<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "border-collapse" => { "border-collapse": "collapse"; }
        "border-separate" => { "border-collapse": "separate"; }
    }
}

fn user_select<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "select-none" => { "user-select": "none"; }
        "select-text" => { "user-select": "text"; }
        "select-all" => { "user-select": "all"; }
        "select-auto" => { "user-select": "auto"; }
    }
}

fn resize<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "resize-none" => { "resize": "none"; }
        "resize-y" => { "resize": "vertical"; }
        "resize-x" => { "resize": "horizontal"; }
        "resize" => { "resize": "both"; }
    }
}

fn scroll_snap_align<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "snap-start" => { "scroll-snap-align": "start"; }
        "snap-end" => { "scroll-snap-align": "end"; }
        "snap-center" => { "scroll-snap-align": "center"; }
        "snap-align-none" => { "scroll-snap-align": "none"; }
    }
}

fn scroll_snap_stop<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "snap-normal" => { "scroll-snap-stop": "normal"; }
        "snap-always" => { "scroll-snap-stop": "always"; }
    }
}

fn list_style_position<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "list-inside" => { "list-style-position": "inside"; }
        "list-outside" => { "list-style-position": "outside"; }
    }
}

fn list_style_type<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "list-none" => { "list-style-type": "none"; }
        "list-disc" => { "list-style-type": "disc"; }
        "list-decimal" => { "list-style-type": "decimal"; }
    }
}

fn list_image<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "list-image-none" => { "list-style-image": "none"; }
    }
}

fn appearance<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "appearance-none" => { "appearance": "none"; }
        "appearance-auto" => { "appearance": "auto"; }
    }
}

fn columns<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "columns-auto" => { "columns": "auto"; }
    }
}

fn break_before<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn break_inside<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "break-inside-auto" => { "break-inside": "auto"; }
        "break-inside-avoid" => { "break-inside": "avoid"; }
        "break-inside-avoid-page" => { "break-inside": "avoid-page"; }
        "break-inside-avoid-column" => { "break-inside": "avoid-column"; }

        // new
        "break-inside-avoid-region" => { "break-inside": "avoid-region"; }
    }
}

fn break_after<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn grid_auto_flow<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "grid-flow-row" => { "grid-auto-flow": "row"; }
        "grid-flow-col" => { "grid-auto-flow": "column"; }
        "grid-flow-dense" => { "grid-auto-flow": "dense"; }
        "grid-flow-row-dense" => { "grid-auto-flow": "row dense"; }
        "grid-flow-col-dense" => { "grid-auto-flow": "column dense"; }
    }
}

fn flex<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn place_content<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn place_items<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "place-items-center" => { "place-items": "center"; }
        "place-items-start" => { "place-items": "start"; }
        "place-items-end" => { "place-items": "end"; }
        "place-items-baseline" => { "place-items": "baseline"; }
        "place-items-stretch" => { "place-items": "stretch"; }
    }
}

fn align_content<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn align_items<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "items-center" => { "align-items": "center"; }
        "items-start" => { "align-items": "start"; }
        "items-end" => { "align-items": "end"; }
        "items-baseline" => { "align-items": "baseline"; }
        "items-stretch" => { "align-items": "stretch"; }
    }
}

fn justify_content<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn justify_items<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "items-normal" => { "justify-items": "normal"; }
        "items-center" => { "justify-items": "center"; }
        "items-start" => { "justify-items": "start"; }
        "items-end" => { "justify-items": "end"; }
        "items-stretch" => { "justify-items": "stretch"; }
    }
}

fn object_fit<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "object-contain" => { "object-fit": "contain"; }
        "object-cover" => { "object-fit": "cover"; }
        "object-fill" => { "object-fit": "fill"; }
        "object-none" => { "object-fit": "none"; }
        "object-scale-down" => { "object-fit": "scale-down"; }
    }
}

fn object_position<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn text_align<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "text-left" => { "text-align": "left"; }
        "text-center" => { "text-align": "center"; }
        "text-right" => { "text-align": "right"; }
        "text-justify" => { "text-align": "justify"; }
        "text-start" => { "text-align": "start"; }
        "text-end" => { "text-align": "end"; }
    }
}

fn vertical_align<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn text_transform<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "uppercase" => { "text-transform": "uppercase"; }
        "lowercase" => { "text-transform": "lowercase"; }
        "capitalize" => { "text-transform": "capitalize"; }
        "normal-case" => { "text-transform": "none"; }
    }
}

fn text_italic<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "italic" => { "font-style": "italic"; }
        "not-italic" => { "font-style": "normal"; }
    }
}

fn font_stretch<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn text_decoration<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "underline" => { "text-decoration": "underline"; }
        "overline" => { "text-decoration": "overline"; }
        "line-through" => { "text-decoration": "line-through"; }
        "no-underline" => { "text-decoration": "none"; }
    }
}

fn text_decoration_style<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "decoration-solid" => { "text-decoration-style": "solid"; }
        "decoration-double" => { "text-decoration-style": "double"; }
        "decoration-dotted" => { "text-decoration-style": "dotted"; }
        "decoration-dashed" => { "text-decoration-style": "dashed"; }
        "decoration-wavy" => { "text-decoration-style": "wavy"; }
    }
}

fn text_decoration_thickness<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "decoration-auto" => { "text-decoration-thickness": "auto"; }
        "decoration-from-font" => { "text-decoration-thickness": "from-font"; }
    }
}

fn animate<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "animate-none" => { "animation": "none"; }
    }
}

fn filter<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "filter" => { "filter": "var(--tw-blur,) var(--tw-brightness,) var(--tw-contrast,) var(--tw-grayscale,) var(--tw-hue-rotate,) var(--tw-invert,) var(--tw-saturate,) var(--tw-sepia,) var(--tw-drop-shadow,)"; }
        "filter-none" => { "filter": "none"; }
    }
}

fn backdrop_filter<'a>() -> Vec<(&'static str, DeclList<'a>)> {
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

fn transform_origin<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "origin-center" => { "transform-origin": "center"; }
        "origin-top" => { "transform-origin": "top"; }
        "origin-top-right" => { "transform-origin": "top right"; }
        "origin-right" => { "transform-origin": "right"; }
        "origin-bottom-right" => { "transform-origin": "bottom right"; }
        "origin-bottom" => { "transform-origin": "bottom"; }
        "origin-bottom-left" => { "transform-origin": "bottom left"; }
        "origin-left" => { "transform-origin": "left"; }
        "origin-top-left" => { "transform-origin": "top left"; }
    }
}

fn perspective_origin<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "perspective-origin-center" => { "perspective-origin": "center"; }
        "perspective-origin-top" => { "perspective-origin": "top"; }
        "perspective-origin-top-right" => { "perspective-origin": "top right"; }
        "perspective-origin-right" => { "perspective-origin": "right"; }
        "perspective-origin-bottom-right" => { "perspective-origin": "bottom right"; }
        "perspective-origin-bottom" => { "perspective-origin": "bottom"; }
        "perspective-origin-bottom-left" => { "perspective-origin": "bottom left"; }
        "perspective-origin-left" => { "perspective-origin": "left"; }
        "perspective-origin-top-left" => { "perspective-origin": "top left"; }
    }
}

fn perspective<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "perspective-none" => { "perspective": "none"; }
    }
}

fn translate<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "translate-3d" => {
            "translate": "var(--tw-translate-x) var(--tw-translate-y) var(--tw-translate-z)";
        }
    }
}

fn scale<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "scale-3d" => {
            "scale": "var(--tw-scale-x) var(--tw-scale-y) var(--tw-scale-z)";
        }
    }
}

fn cursor<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "cursor-auto" => { "cursor": "auto"; }
        "cursor-default" => { "cursor": "default"; }
        "cursor-pointer" => { "cursor": "pointer"; }
        "cursor-wait" => { "cursor": "wait"; }
        "cursor-text" => { "cursor": "text"; }
        "cursor-move" => { "cursor": "move"; }
        "cursor-help" => { "cursor": "help"; }
        "cursor-not-allowed" => { "cursor": "not-allowed"; }
        "cursor-none" => { "cursor": "none"; }
        "cursor-context-menu" => { "cursor": "context-menu"; }
        "cursor-progress" => { "cursor": "progress"; }
        "cursor-cell" => { "cursor": "cell"; }
        "cursor-crosshair" => { "cursor": "crosshair"; }
        "cursor-vertical-text" => { "cursor": "vertical-text"; }
        "cursor-alias" => { "cursor": "alias"; }
        "cursor-copy" => { "cursor": "copy"; }
        "cursor-no-drop" => { "cursor": "no-drop"; }
        "cursor-grab" => { "cursor": "grab"; }
        "cursor-grabbing" => { "cursor": "grabbing"; }
        "cursor-all-scroll" => { "cursor": "all-scroll"; }
        "cursor-col-resize" => { "cursor": "col-resize"; }
        "cursor-row-resize" => { "cursor": "row-resize"; }
        "cursor-n-resize" => { "cursor": "n-resize"; }
        "cursor-e-resize" => { "cursor": "e-resize"; }
        "cursor-s-resize" => { "cursor": "s-resize"; }
        "cursor-w-resize" => { "cursor": "w-resize"; }
        "cursor-ne-resize" => { "cursor": "ne-resize"; }
        "cursor-nw-resize" => { "cursor": "nw-resize"; }
        "cursor-se-resize" => { "cursor": "se-resize"; }
        "cursor-sw-resize" => { "cursor": "sw-resize"; }
        "cursor-ew-resize" => { "cursor": "ew-resize"; }
        "cursor-ns-resize" => { "cursor": "ns-resize"; }
        "cursor-nesw-resize" => { "cursor": "nesw-resize"; }
        "cursor-nwse-resize" => { "cursor": "nwse-resize"; }
        "cursor-zoom-in" => { "cursor": "zoom-in"; }
        "cursor-zoom-out" => { "cursor": "zoom-out"; }
    }
}

fn touch_action<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "touch-auto" => { "touch-action": "auto"; }
        "touch-none" => { "touch-action": "none"; }
        "manipulation" => { "touch-action": "manipulation"; }
    }
}

fn transform<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "transform-cpu" => { "transform": "var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)"; }
        "transform-gpu" => { "transform": "translateZ(0) var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)"; }
        "transform-none" => { "transform": "none"; }
    }
}

fn transform_style<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "transform-flat" => { "transform-style": "flat"; }
        "transform-preserve-3d" => { "transform-style": "preserve-3d"; }
    }
}

fn transform_box<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "transform-content" => { "transform-box": "content-box"; }
        "transform-border" => { "transform-box": "border-box"; }
        "transform-fill" => { "transform-box": "fill-box"; }
        "transform-stroke" => { "transform-box": "stroke-box"; }
        "transform-view" => { "transform-box": "view-box"; }
    }
}

fn backface_visibility<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "backface-visible" => { "backface-visibility": "visible"; }
        "backface-hidden" => { "backface-visibility": "hidden"; }
    }
}

fn scroll_snap<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "snap-none" => { "scroll-snap-type": "none"; }
        // TODO: snapProperties
        "snap-x" => { "scroll-snap-type": "x var(--tw-scroll-snap-strictness)"; }
        "snap-y" => { "scroll-snap-type": "y var(--tw-scroll-snap-strictness)"; }
        "snap-both" => { "scroll-snap-type": "both var(--tw-scroll-snap-strictness)"; }

        "snap-mandatory" => { "scroll-snap-type": "mandatory"; }
        "snap-proximity" => { "scroll-snap-type": "proximity"; }
    }
}

fn touch_pan<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "touch-pan-x" => {
            "--tw-pan-x": "pan-x";
            "touch-action": "var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)";
        }
        "touch-pan-left" => {
            "--tw-pan-x": "pan-left";
            "touch-action": "var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)";
        }
        "touch-pan-right" => {
            "--tw-pan-x": "pan-right";
            "touch-action": "var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)";
        }
        "touch-pan-y" => {
            "--tw-pan-y": "pan-y";
            "touch-action": "var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)";
        }
        "touch-pan-up" => {
            "--tw-pan-y": "pan-up";
            "touch-action": "var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)";
        }
        "touch-pan-down" => {
            "--tw-pan-y": "pan-down";
            "touch-action": "var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)";
        }
        "touch-pinch-zoom" => {
            "--tw-pinch-zoom": "pinch-zoom";
            "touch-action": "var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)";
        }
    }
}

fn space<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    // TODO: :where(& > :not(:last-child)
    static_rules! {
        "space-x-reverse" => {
            "--tw-space-x-reverse": "0";
        }
        "space-y-reverse" => {
            "--tw-space-y-reverse": "0";
        }
    }
}

fn overflow<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "overflow-auto" => { "overflow": "auto"; }
        "overflow-hidden" => { "overflow": "hidden"; }
        "overflow-clip" => { "overflow": "clip"; }
        "overflow-visible" => { "overflow": "visible"; }
        "overflow-scroll" => { "overflow": "scroll"; }
    }
}

fn overscroll_behavior<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "overscroll-auto" => { "overscroll-behavior": "auto"; }
        "overscroll-contain" => { "overscroll-behavior": "contain"; }
        "overscroll-none" => { "overscroll-behavior": "none"; }
    }
}

fn scroll_behavior<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "scroll-auto" => { "scroll-behavior": "auto"; }
        "scroll-smooth" => { "scroll-behavior": "smooth"; }
    }
}

fn truncate<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "truncate" => {
            "overflow": "hidden";
            "text-overflow": "ellipsis";
            "white-space": "nowrap";
        }
    }
}

fn text_overflow<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "text-ellipsis" => { "text-overflow": "ellipsis"; }
        "text-clip" => { "text-overflow": "clip"; }
    }
}

fn hyphens<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "hyphens-none" => {
            "-webkit-hyphens": "none";
            "hyphens": "none";
        }
        "hyphens-manual" => {
            "-webkit-hyphens": "manual";
            "hyphens": "manual";
        }
        "hyphens-auto" => {
            "-webkit-hyphens": "auto";
            "hyphens": "auto";
        }
    }
}

fn white_space<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "whitespace-normal" => { "white-space": "normal"; }
        "whitespace-nowrap" => { "white-space": "nowrap"; }
        "whitespace-pre" => { "white-space": "pre"; }
        "whitespace-pre-line" => { "white-space": "pre-line"; }
        "whitespace-pre-wrap" => { "white-space": "pre-wrap"; }
        "whitespace-break-spaces" => { "white-space": "break-spaces"; }
    }
}

fn text_wrap<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "text-wrap" => { "overflow-wrap": "break-word"; }
        "text-nowrap" => { "overflow-wrap": "normal"; }
        "text-balance" => { "overflow-wrap": "balance"; }
        "text-pretty" => { "overflow-wrap": "pretty"; }
    }
}

fn word_break<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "break-normal" => {
            "overflow-wrap": "normal";
            "word-break": "normal";
        }
        "break-words" => { "overflow-wrap": "break-word"; }
        "break-all" => { "word-break": "break-all"; }
        "break-keep" => { "word-break": "break-keep"; }
    }
}

fn border_style<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "border-solid" => {
            "--tw-border-style": "solid";
            "border-style": "solid";
        }
        "border-dashed" => {
            "--tw-border-style": "dashed";
            "border-style": "dashed";
        }
        "border-dotted" => {
            "--tw-border-style": "dotted";
            "border-style": "dotted";
        }
        "border-double" => {
            "--tw-border-style": "double";
            "border-style": "double";
        }
        "border-hidden" => {
            "--tw-border-style": "hidden";
            "border-style": "hidden";
        }
        "border-none" => {
            "--tw-border-style": "none";
            "border-style": "none";
        }
    }
}

fn background_attachment<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "bg-fixed" => { "background-attachment": "fixed"; }
        "bg-local" => { "background-attachment": "local"; }
        "bg-scroll" => { "background-attachment": "scroll"; }
    }
}

fn background_repeat<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "bg-repeat" => { "background-repeat": "repeat"; }
        "bg-no-repeat" => { "background-repeat": "no-repeat"; }
        "bg-repeat-x" => { "background-repeat": "repeat-x"; }
        "bg-repeat-y" => { "background-repeat": "repeat-y"; }
        "bg-round" => { "background-repeat": "round"; }
        "bg-space" => { "background-repeat": "space"; }
    }
}

fn background_image<'a>() -> Vec<(&'static str, DeclList<'a>)> {
    static_rules! {
        "bg-none" => { "background-image": "none"; }
    }
}

fn get_all() -> Vec<(&'static str, DeclList<'static>)> {
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
        .chain(list_style_type())
        .chain(list_image())
        .chain(appearance())
        .chain(columns())
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
        .chain(text_decoration_style())
        .chain(text_decoration_thickness())
        .chain(animate())
        .chain(filter())
        .chain(backdrop_filter())
        .chain(transform_origin())
        .chain(perspective_origin())
        .chain(perspective())
        .chain(translate())
        .chain(scale())
        .chain(cursor())
        .chain(touch_action())
        .chain(transform())
        .chain(transform_style())
        .chain(transform_box())
        .chain(backface_visibility())
        .chain(scroll_snap())
        .chain(touch_pan())
        .chain(space())
        .chain(overflow())
        .chain(overscroll_behavior())
        .chain(scroll_behavior())
        .chain(truncate())
        .chain(text_overflow())
        .chain(hyphens())
        .chain(white_space())
        .chain(text_wrap())
        .chain(word_break())
        .chain(border_style())
        .chain(background_attachment())
        .chain(background_repeat())
        .chain(background_image())
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
