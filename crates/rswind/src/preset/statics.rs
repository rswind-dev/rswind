use smol_str::SmolStr;

use crate::{
    context::{utilities::StaticUtility, Context},
    css::DeclList,
};

// TODO: replace all presets into codegen

#[macro_export]
macro_rules! static_utilities_macro {
  (
    $($key:literal => {
      $($name:literal: $value:literal;)+
    })+
  ) => {
    [
      $(
        (smol_str::SmolStr::new_static($key), DeclList(vec![
          $(
            $crate::css::Decl {
                name: smol_str::SmolStr::new_static($name),
                value: smol_str::SmolStr::new_static($value),
            },
          )+
        ])),
      )+
    ]
  };
}

fn static_utilities() -> [(SmolStr, DeclList); 352] {
    static_utilities_macro! {
        // accessibility
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

        // pointer_events
        "pointer-events-none" => { "pointer-events": "none"; }
        "pointer-events-auto" => { "pointer-events": "auto"; }

        // visibility
        "visible" => { "visibility": "visible"; }
        "invisible" => { "visibility": "hidden"; }
        "collapse" => { "visibility": "collapse"; }

        // position
        "static" => { "position": "static"; }
        "fixed" => { "position": "fixed"; }
        "absolute" => { "position": "absolute"; }
        "relative" => { "position": "relative"; }
        "sticky" => { "position": "sticky"; }

        // isolation
        "isolate" => { "isolation": "isolate"; }
        "isolation-auto" => { "isolation": "auto"; }

        // float
        "float-right" => { "float": "right"; }
        "float-left" => { "float": "left"; }
        "float-none" => { "float": "none"; }

        // clear
        "clear-start" => { "clear": "inline-start"; }
        "clear-end" => { "clear": "inline-end"; }
        "clear-left" => { "clear": "left"; }
        "clear-right" => { "clear": "right"; }
        "clear-both" => { "clear": "both"; }
        "clear-none" => { "clear": "none"; }

        // box_sizing
        "box-border" => { "box-sizing": "border-box"; }
        "box-content" => { "box-sizing": "content-box"; }

        // display
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

        // table_layout
        "table-auto" => { "table-layout": "auto"; }
        "table-fixed" => { "table-layout": "fixed"; }

        // caption_side
        "caption-top" => { "caption-side": "top"; }
        "caption-bottom" => { "caption-side": "bottom"; }

        // border_collapse
        "border-collapse" => { "border-collapse": "collapse"; }
        "border-separate" => { "border-collapse": "separate"; }

        // user_select
        "select-none" => { "user-select": "none"; }
        "select-text" => { "user-select": "text"; }
        "select-all" => { "user-select": "all"; }
        "select-auto" => { "user-select": "auto"; }

        // resize
        "resize-none" => { "resize": "none"; }
        "resize-y" => { "resize": "vertical"; }
        "resize-x" => { "resize": "horizontal"; }
        "resize" => { "resize": "both"; }

        // scroll_snap_align
        "snap-start" => { "scroll-snap-align": "start"; }
        "snap-end" => { "scroll-snap-align": "end"; }
        "snap-center" => { "scroll-snap-align": "center"; }
        "snap-align-none" => { "scroll-snap-align": "none"; }

        // scroll_snap_stop
        "snap-normal" => { "scroll-snap-stop": "normal"; }
        "snap-always" => { "scroll-snap-stop": "always"; }

        // list_style_position
        "list-inside" => { "list-style-position": "inside"; }
        "list-outside" => { "list-style-position": "outside"; }

        // list_style_type
        "list-none" => { "list-style-type": "none"; }
        "list-disc" => { "list-style-type": "disc"; }
        "list-decimal" => { "list-style-type": "decimal"; }

        // list_image
        "list-image-none" => { "list-style-image": "none"; }

        // appearance
        "appearance-none" => { "appearance": "none"; }
        "appearance-auto" => { "appearance": "auto"; }

        // columns
        "columns-auto" => { "columns": "auto"; }

        // break_before
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

        // break_inside
        "break-inside-auto" => { "break-inside": "auto"; }
        "break-inside-avoid" => { "break-inside": "avoid"; }
        "break-inside-avoid-page" => { "break-inside": "avoid-page"; }
        "break-inside-avoid-column" => { "break-inside": "avoid-column"; }
        // new
        "break-inside-avoid-region" => { "break-inside": "avoid-region"; }

        // break_after
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

        // grid_auto_flow
        "grid-flow-row" => { "grid-auto-flow": "row"; }
        "grid-flow-col" => { "grid-auto-flow": "column"; }
        "grid-flow-dense" => { "grid-auto-flow": "dense"; }
        "grid-flow-row-dense" => { "grid-auto-flow": "row dense"; }
        "grid-flow-col-dense" => { "grid-auto-flow": "column dense"; }

        // flex
        "flex-row" => { "flex-direction": "row"; }
        "flex-row-reverse" => { "flex-direction": "row-reverse"; }
        "flex-col" => { "flex-direction": "column"; }
        "flex-col-reverse" => { "flex-direction": "column-reverse"; }
        "flex-wrap" => { "flex-wrap": "wrap"; }
        "flex-wrap-reverse" => { "flex-wrap": "wrap-reverse"; }
        "flex-nowrap" => { "flex-wrap": "nowrap"; }

        // place_content
        "place-content-center" => { "place-content": "center"; }
        "place-content-start" => { "place-content": "start"; }
        "place-content-end" => { "place-content": "end"; }
        "place-content-space-between" => { "place-content": "space-between"; }
        "place-content-space-around" => { "place-content": "space-around"; }
        "place-content-space-evenly" => { "place-content": "space-evenly"; }
        "place-content-baseline" => { "place-content": "baseline"; }
        "place-content-stretch" => { "place-content": "stretch"; }

        // place_items
        "place-items-center" => { "place-items": "center"; }
        "place-items-start" => { "place-items": "start"; }
        "place-items-end" => { "place-items": "end"; }
        "place-items-baseline" => { "place-items": "baseline"; }
        "place-items-stretch" => { "place-items": "stretch"; }

        // align_content
        "content-normal" => { "align-content": "normal"; }
        "content-center" => { "align-content": "center"; }
        "content-start" => { "align-content": "start"; }
        "content-end" => { "align-content": "end"; }
        "content-between" => { "align-content": "space-between"; }
        "content-around" => { "align-content": "space-around"; }
        "content-evenly" => { "align-content": "space-evenly"; }
        "content-baseline" => { "align-content": "baseline"; }
        "content-stretch" => { "align-content": "stretch"; }

        // align_items
        "items-center" => { "align-items": "center"; }
        "items-start" => { "align-items": "start"; }
        "items-end" => { "align-items": "end"; }
        "items-baseline" => { "align-items": "baseline"; }
        "items-stretch" => { "align-items": "stretch"; }

        // justify_content
        "justify-normal" => { "justify-content": "normal"; }
        "justify-center" => { "justify-content": "center"; }
        "justify-start" => { "justify-content": "start"; }
        "justify-end" => { "justify-content": "end"; }
        "justify-between" => { "justify-content": "space-between"; }
        "justify-around" => { "justify-content": "space-around"; }
        "justify-evenly" => { "justify-content": "space-evenly"; }
        "justify-baseline" => { "justify-content": "baseline"; }
        "justify-stretch" => { "justify-content": "stretch"; }

        // justify_items
        "items-normal" => { "justify-items": "normal"; }
        "items-center" => { "justify-items": "center"; }
        "items-start" => { "justify-items": "start"; }
        "items-end" => { "justify-items": "end"; }
        "items-stretch" => { "justify-items": "stretch"; }

        // object_fit
        "object-contain" => { "object-fit": "contain"; }
        "object-cover" => { "object-fit": "cover"; }
        "object-fill" => { "object-fit": "fill"; }
        "object-none" => { "object-fit": "none"; }
        "object-scale-down" => { "object-fit": "scale-down"; }

        // object_position
        "object-bottom" => { "object-position": "bottom"; }
        "object-center" => { "object-position": "center"; }
        "object-left" => { "object-position": "left"; }
        "object-left-bottom" => { "object-position": "left bottom"; }
        "object-left-top" => { "object-position": "left top"; }
        "object-right" => { "object-position": "right"; }
        "object-right-bottom" => { "object-position": "right bottom"; }
        "object-right-top" => { "object-position": "right top"; }
        "object-top" => { "object-position": "top"; }

        // text_align
        "text-left" => { "text-align": "left"; }
        "text-center" => { "text-align": "center"; }
        "text-right" => { "text-align": "right"; }
        "text-justify" => { "text-align": "justify"; }
        "text-start" => { "text-align": "start"; }
        "text-end" => { "text-align": "end"; }

        // vertical_align
        "align-baseline" => { "vertical-align": "baseline"; }
        "align-top" => { "vertical-align": "top"; }
        "align-middle" => { "vertical-align": "middle"; }
        "align-bottom" => { "vertical-align": "bottom"; }
        "align-text-top" => { "vertical-align": "text-top"; }
        "align-text-bottom" => { "vertical-align": "text-bottom"; }
        "align-sub" => { "vertical-align": "sub"; }
        "align-super" => { "vertical-align": "super"; }

        // text_transform
        "uppercase" => { "text-transform": "uppercase"; }
        "lowercase" => { "text-transform": "lowercase"; }
        "capitalize" => { "text-transform": "capitalize"; }
        "normal-case" => { "text-transform": "none"; }

        // text_italic
        "italic" => { "font-style": "italic"; }
        "not-italic" => { "font-style": "normal"; }

        // font_stretch
        "font-stretch-normal" => { "font-stretch": "normal"; }
        "font-stretch-ultra-condensed" => { "font-stretch": "ultra-condensed"; }
        "font-stretch-extra-condensed" => { "font-stretch": "extra-condensed"; }
        "font-stretch-condensed" => { "font-stretch": "condensed"; }
        "font-stretch-semi-condensed" => { "font-stretch": "semi-condensed"; }
        "font-stretch-semi-expanded" => { "font-stretch": "semi-expanded"; }
        "font-stretch-expanded" => { "font-stretch": "expanded"; }
        "font-stretch-extra-expanded" => { "font-stretch": "extra-expanded"; }
        "font-stretch-ultra-expanded" => { "font-stretch": "ultra-expanded"; }

        // text_decoration
        "underline" => { "text-decoration": "underline"; }
        "overline" => { "text-decoration": "overline"; }
        "line-through" => { "text-decoration": "line-through"; }
        "no-underline" => { "text-decoration": "none"; }

        // text_decoration_style
        "decoration-solid" => { "text-decoration-style": "solid"; }
        "decoration-double" => { "text-decoration-style": "double"; }
        "decoration-dotted" => { "text-decoration-style": "dotted"; }
        "decoration-dashed" => { "text-decoration-style": "dashed"; }
        "decoration-wavy" => { "text-decoration-style": "wavy"; }

        // text_decoration_thickness
        "decoration-auto" => { "text-decoration-thickness": "auto"; }
        "decoration-from-font" => { "text-decoration-thickness": "from-font"; }

        // animate
        "animate-none" => { "animation": "none"; }

        // filter
        "filter" => { "filter": "var(--tw-blur,) var(--tw-brightness,) var(--tw-contrast,) var(--tw-grayscale,) var(--tw-hue-rotate,) var(--tw-invert,) var(--tw-saturate,) var(--tw-sepia,) var(--tw-drop-shadow,)"; }
        "filter-none" => { "filter": "none"; }

        // backdrop_filter
        "backdrop-filter" => {
            "-webkit-backdrop-filter": "var(--tw-backdrop-blur,) var(--tw-backdrop-brightness,) var(--tw-backdrop-contrast,) var(--tw-backdrop-grayscale,) var(--tw-backdrop-hue-rotate,) var(--tw-backdrop-invert,) var(--tw-backdrop-opacity,) var(--tw-backdrop-saturate,) var(--tw-backdrop-sepia,)";
            "backdrop-filter": "var(--tw-backdrop-blur,) var(--tw-backdrop-brightness,) var(--tw-backdrop-contrast,) var(--tw-backdrop-grayscale,) var(--tw-backdrop-hue-rotate,) var(--tw-backdrop-invert,) var(--tw-backdrop-opacity,) var(--tw-backdrop-saturate,) var(--tw-backdrop-sepia,)";
        }
        "backdrop-filter-none" => {
            "-webkit-backdrop-filter": "none";
            "backdrop-filter": "none";
        }

        // transform_origin
        "origin-center" => { "transform-origin": "center"; }
        "origin-top" => { "transform-origin": "top"; }
        "origin-top-right" => { "transform-origin": "top right"; }
        "origin-right" => { "transform-origin": "right"; }
        "origin-bottom-right" => { "transform-origin": "bottom right"; }
        "origin-bottom" => { "transform-origin": "bottom"; }
        "origin-bottom-left" => { "transform-origin": "bottom left"; }
        "origin-left" => { "transform-origin": "left"; }
        "origin-top-left" => { "transform-origin": "top left"; }

        // perspective_origin
        "perspective-origin-center" => { "perspective-origin": "center"; }
        "perspective-origin-top" => { "perspective-origin": "top"; }
        "perspective-origin-top-right" => { "perspective-origin": "top right"; }
        "perspective-origin-right" => { "perspective-origin": "right"; }
        "perspective-origin-bottom-right" => { "perspective-origin": "bottom right"; }
        "perspective-origin-bottom" => { "perspective-origin": "bottom"; }
        "perspective-origin-bottom-left" => { "perspective-origin": "bottom left"; }
        "perspective-origin-left" => { "perspective-origin": "left"; }
        "perspective-origin-top-left" => { "perspective-origin": "top left"; }

        // perspective
        "perspective-none" => { "perspective": "none"; }

        // translate
        "translate-3d" => {
            "translate": "var(--tw-translate-x) var(--tw-translate-y) var(--tw-translate-z)";
        }

        // scale
        "scale-3d" => {
            "scale": "var(--tw-scale-x) var(--tw-scale-y) var(--tw-scale-z)";
        }

        // cursor
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

        // touch_action
        "touch-auto" => { "touch-action": "auto"; }
        "touch-none" => { "touch-action": "none"; }
        "manipulation" => { "touch-action": "manipulation"; }

        // transform
        "transform-cpu" => { "transform": "var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)"; }
        "transform-gpu" => { "transform": "translateZ(0) var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)"; }
        "transform-none" => { "transform": "none"; }

        // transform_style
        "transform-flat" => { "transform-style": "flat"; }
        "transform-preserve-3d" => { "transform-style": "preserve-3d"; }

        // transform_box
        "transform-content" => { "transform-box": "content-box"; }
        "transform-border" => { "transform-box": "border-box"; }
        "transform-fill" => { "transform-box": "fill-box"; }
        "transform-stroke" => { "transform-box": "stroke-box"; }
        "transform-view" => { "transform-box": "view-box"; }

        // backface_visibility
        "backface-visible" => { "backface-visibility": "visible"; }
        "backface-hidden" => { "backface-visibility": "hidden"; }

        // scroll_snap
        "snap-none" => { "scroll-snap-type": "none"; }
        // TODO: snapProperties
        "snap-x" => { "scroll-snap-type": "x var(--tw-scroll-snap-strictness)"; }
        "snap-y" => { "scroll-snap-type": "y var(--tw-scroll-snap-strictness)"; }
        "snap-both" => { "scroll-snap-type": "both var(--tw-scroll-snap-strictness)"; }
        "snap-mandatory" => { "scroll-snap-type": "mandatory"; }
        "snap-proximity" => { "scroll-snap-type": "proximity"; }

        // touch_pan
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
    // TODO: :where(& > :not(:last-child)
        //  //
        "space-x-reverse" => {
            "--tw-space-x-reverse": "0";
        }
        "space-y-reverse" => {
            "--tw-space-y-reverse": "0";
        }

        // overflow
        "overflow-auto" => { "overflow": "auto"; }
        "overflow-hidden" => { "overflow": "hidden"; }
        "overflow-clip" => { "overflow": "clip"; }
        "overflow-visible" => { "overflow": "visible"; }
        "overflow-scroll" => { "overflow": "scroll"; }
        // overscroll_behavior
        "overscroll-auto" => { "overscroll-behavior": "auto"; }
        "overscroll-contain" => { "overscroll-behavior": "contain"; }
        "overscroll-none" => { "overscroll-behavior": "none"; }

        // scroll_behavior
        "scroll-auto" => { "scroll-behavior": "auto"; }
        "scroll-smooth" => { "scroll-behavior": "smooth"; }

        // truncate
        "truncate" => {
            "overflow": "hidden";
            "text-overflow": "ellipsis";
            "white-space": "nowrap";
        }

        // text_overflow
        "text-ellipsis" => { "text-overflow": "ellipsis"; }
        "text-clip" => { "text-overflow": "clip"; }

        // hyphens
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

        // white_space
        "whitespace-normal" => { "white-space": "normal"; }
        "whitespace-nowrap" => { "white-space": "nowrap"; }
        "whitespace-pre" => { "white-space": "pre"; }
        "whitespace-pre-line" => { "white-space": "pre-line"; }
        "whitespace-pre-wrap" => { "white-space": "pre-wrap"; }
        "whitespace-break-spaces" => { "white-space": "break-spaces"; }

        // text_wrap
        "text-wrap" => { "overflow-wrap": "break-word"; }
        "text-nowrap" => { "overflow-wrap": "normal"; }
        "text-balance" => { "overflow-wrap": "balance"; }
        "text-pretty" => { "overflow-wrap": "pretty"; }

        // word_break
        "break-normal" => {
            "overflow-wrap": "normal";
            "word-break": "normal";
        }
        "break-words" => { "overflow-wrap": "break-word"; }
        "break-all" => { "word-break": "break-all"; }
        "break-keep" => { "word-break": "break-keep"; }

        // border_style
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

        // background_attachment
        "bg-fixed" => { "background-attachment": "fixed"; }
        "bg-local" => { "background-attachment": "local"; }
        "bg-scroll" => { "background-attachment": "scroll"; }

        // background_repeat
        "bg-repeat" => { "background-repeat": "repeat"; }
        "bg-no-repeat" => { "background-repeat": "no-repeat"; }
        "bg-repeat-x" => { "background-repeat": "repeat-x"; }
        "bg-repeat-y" => { "background-repeat": "repeat-y"; }
        "bg-round" => { "background-repeat": "round"; }
        "bg-space" => { "background-repeat": "space"; }

        // background_image
        "bg-none" => { "background-image": "none"; }
    }
}

macro_rules! selector_static_utilities {
    (
        $($key:literal as $selector:literal => {
            $($name:literal: $value:literal;)+
        })+
    ) => {
        [
            $(
                (
                    smol_str::SmolStr::new_static($key),
                    StaticUtility::new(
                        smol_str::SmolStr::new_static($selector),
                        DeclList(vec![
                            $(
                                $crate::css::Decl {
                                    name: smol_str::SmolStr::new_static($name),
                                    value: smol_str::SmolStr::new_static($value),
                                },
                            )+
                        ]),
                    )
                ),
            )+
        ]
    };
}

fn static_utilities_selectored() -> [(SmolStr, StaticUtility); 5] {
    selector_static_utilities! {
        "divide-solid" as ":where(& > :not(:last-child))" => {
            "border-style": "solid";
        }
        "divide-dashed" as ":where(& > :not(:last-child))" => {
            "border-style": "dashed";
        }
        "divide-dotted" as ":where(& > :not(:last-child))" => {
            "border-style": "dotted";
        }
        "divide-double" as ":where(& > :not(:last-child))" => {
            "border-style": "double";
        }
        "divide-none" as ":where(& > :not(:last-child))" => {
            "border-style": "none";
        }
    }
}

pub fn load_static_utilities(ctx: &mut Context) {
    let iter = static_utilities().into_iter();
    let selector_iter = static_utilities_selectored().into_iter();
    let reserve = iter.size_hint().0 + selector_iter.size_hint().0;
    ctx.utilities.reserve(reserve);
    iter.for_each(|(key, value)| {
        ctx.add_static(key, value);
    });
    selector_iter.for_each(|(key, value)| {
        ctx.add_static(key, value);
    });
}
