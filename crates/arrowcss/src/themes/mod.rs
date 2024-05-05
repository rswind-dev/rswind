use phf::Map;

use crate::{
    theme::Theme,
    themes::{colors::COLORS, spacing::SPACING},
};

mod colors;
mod spacing;

macro_rules! create_theme {
    ($($key:expr => $value:expr),*) => {
        {
            let mut m = fxhash::FxHashMap::default();
            $(
                m.insert($key.into(), $value.into());
            )*
            Theme(m)
        }
    };
}

pub fn theme() -> Theme {
    create_theme! {
        "colors" => &COLORS,
        "spacing" => &SPACING,
        "translate" => &TRANSLATE,
        "blur" => &BLUR,
        "ringWidth" => &RING_WIDTH,
        "ringOffsetWidth" => &RING_OFFSET_WIDTH,
        "backgroundPosition" => &BACKGROUND_POSITION,
        "backgroundSize" => &BACKGROUND_SIZE,
        "backgroundImage" => &BACKGROUND_IMAGE,
        "opacity" => &OPACITY,
        "lineClamp" => &LINE_CLAMP,
        "borderWidth" => &BORDER_WIDTH,
        "breakpoints" => &BREAKPOINTS,
        "lineHeight" => &LINE_HEIGHT,
        "animate" => &ANIMATE,
        "fontSize" => &FONT_SIZE,
        "fontSize:lineHeight" => &FONT_SIZE_LINE_HEIGHT,
        "fontWeight" => &FONT_WEIGHT,
        "gradientColorStopPositions" => &GRADIENT_COLOR_STOP_POSITIONS,
        "stokeWidth" => &STROKE_WIDTH,
        "brightness" => &BRIGHTNESS,
        "contrast" => &CONTRAST,
        "grayscale" => &GRAYSCALE,
        "hueRotate" => &HUE_ROTATE,
        "invert" => &INVERT,
        "saturate" => &SATURATE,
        "sepia" => &SEPIA,
        "flex" => &FLEX,
        "flexShrink" => &FLEX_SHRINK,
        "flexGrow" => &FLEX_GROW,
        "borderRadius" => &BORDER_RADIUS,
        "scale" => &SCALE,
        "rotate" => &ROTATE,
        "skew" => &SKEW,
        "textDecorationThickness" => &TEXT_DECORATION_THICKNESS,
        "gridAutoColumns" => &GRID_AUTO_COLUMNS,
        "gridAutoRows" => &GRID_AUTO_ROWS
    }
}

macro_rules! define_theme {
    ($($name:ident => { $($key:expr => $value:expr),* }),*) => {
        $(
            static $name: Map<&'static str, &'static str> = phf::phf_map!(
                $(
                    $key => $value,
                )*
            );
        )*
    };
}

define_theme!(
    BLUR => {
        "0" => "0",
        "none" => "0",
        "sm" => "4px",
        "DEFAULT" => "8px",
        "md" => "12px",
        "lg" => "16px",
        "xl" => "24px",
        "2xl" => "40px",
        "3xl" => "64px"
    },
    TRANSLATE => {
        "0" => "0",
        "px" => "1px",
        "1/2" => "50%",
        "1/3" => "33.333333%",
        "2/3" => "66.666667%",
        "1/4" => "25%",
        "2/4" => "50%",
        "3/4" => "75%",
        "full" => "100%"
    },
    RING_WIDTH => {
        "DEFAULT" => "3px",
        "0" => "0px",
        "1" => "1px",
        "2" => "2px",
        "4" => "4px",
        "8" => "8px"
    },
    RING_OFFSET_WIDTH => {
        "0" => "0px",
        "1" => "1px",
        "2" => "2px",
        "4" => "4px",
        "8" => "8px"
    },
    BACKGROUND_POSITION => {
        "bottom" => "bottom",
        "center" => "center",
        "left" => "left",
        "left-bottom" => "left bottom",
        "left-top" => "left top",
        "right" => "right",
        "right-bottom" => "right bottom",
        "right-top" => "right top",
        "top" => "top"
    },
    BACKGROUND_SIZE => {
        "auto" => "auto",
        "cover" => "cover",
        "contain" => "contain"
    },
    BACKGROUND_IMAGE => {
        "none" => "none",
        "gradient-to-t" => "linear-gradient(to top, var(--tw-gradient-stops,))",
        "gradient-to-tr" => "linear-gradient(to top right, var(--tw-gradient-stops,))",
        "gradient-to-r" => "linear-gradient(to right, var(--tw-gradient-stops,))",
        "gradient-to-br" => "linear-gradient(to bottom right, var(--tw-gradient-stops,))",
        "gradient-to-b" => "linear-gradient(to bottom, var(--tw-gradient-stops,))",
        "gradient-to-bl" => "linear-gradient(to bottom left, var(--tw-gradient-stops,))",
        "gradient-to-l" => "linear-gradient(to left, var(--tw-gradient-stops,))",
        "gradient-to-tl" => "linear-gradient(to top left, var(--tw-gradient-stops,))"
    },
    OPACITY => {
        "0" => "0",
        "5" => "0.05",
        "10" => "0.1",
        "20" => "0.2",
        "25" => "0.25",
        "30" => "0.3",
        "40" => "0.4",
        "50" => "0.5",
        "60" => "0.6",
        "70" => "0.7",
        "75" => "0.75",
        "80" => "0.8",
        "90" => "0.9",
        "95" => "0.95",
        "100" => "1"
    },
    LINE_CLAMP => {
        "1" => "1",
        "2" => "2",
        "3" => "3",
        "4" => "4",
        "5" => "5",
        "6" => "6"
    },
    BORDER_WIDTH => {
        "DEFAULT" => "1px",
        "0" => "0",
        "2" => "2px",
        "4" => "4px",
        "8" => "8px"
    },
    BREAKPOINTS => {
        "sm" => "640px",
        "md" => "768px",
        "lg" => "1024px",
        "xl" => "1280px",
        "2xl" => "1536px"
    },
    LINE_HEIGHT => {
        "3" => "0.75rem",
        "4" => "1rem",
        "5" => "1.25rem",
        "6" => "1.5rem",
        "7" => "1.75rem",
        "8" => "2rem",
        "9" => "2.25rem",
        "10" => "2.5rem",
        "none" => "1",
        "tight" => "1.25",
        "snug" => "1.375",
        "normal" => "1.5",
        "relaxed" => "1.625",
        "loose" => "2"
    },
    ANIMATE => {
        "none" => "none",
        "spin" => "spin 1s linear infinite",
        "ping" => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite",
        "pulse" => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "bounce" => "bounce 1s infinite"
    },
    FONT_SIZE => {
        "xs" => "0.75rem",
        "sm" => "0.875rem",
        "base" => "1rem",
        "lg" => "1.125rem",
        "xl" => "1.25rem",
        "2xl" => "1.5rem",
        "3xl" => "1.875rem",
        "4xl" => "2.25rem",
        "5xl" => "3rem",
        "6xl" => "4rem"
    },
    FONT_SIZE_LINE_HEIGHT => {
        "xs" => "1.5",
        "sm" => "1.5",
        "base" => "1.5",
        "lg" => "1.5",
        "xl" => "1.5",
        "2xl" => "1.5",
        "3xl" => "1.25",
        "4xl" => "1.25",
        "5xl" => "1.25",
        "6xl" => "1.125"
    },
    FONT_WEIGHT => {
        "thin" => "100",
        "extralight" => "200",
        "light" => "300",
        "normal" => "400",
        "medium" => "500",
        "semibold" => "600",
        "bold" => "700",
        "extrabold" => "800",
        "black" => "900"
    },
    GRADIENT_COLOR_STOP_POSITIONS => {
        "0%" => "0%",
        "5%" => "5%",
        "10%" => "10%",
        "15%" => "15%",
        "20%" => "20%",
        "25%" => "25%",
        "30%" => "30%",
        "35%" => "35%",
        "40%" => "40%",
        "45%" => "45%",
        "50%" => "50%",
        "55%" => "55%",
        "60%" => "60%",
        "65%" => "65%",
        "70%" => "70%",
        "75%" => "75%",
        "80%" => "80%",
        "85%" => "85%",
        "90%" => "90%",
        "95%" => "95%",
        "100%" => "100%"
    },
    STROKE_WIDTH => {
        "0" => "0",
        "1" => "1",
        "2" => "2",
        "3" => "3"
    },
    BRIGHTNESS => {
        "0" => "0",
        "50" => ".5",
        "75" => ".75",
        "90" => ".9",
        "95" => ".95",
        "100" => "1.0",
        "105" => "1.05",
        "110" => "1.1",
        "125" => "1.25",
        "150" => "1.5",
        "200" => "2"
    },
    CONTRAST => {
        "0" => "0",
        "50" => ".5",
        "75" => ".75",
        "100" => "1.0",
        "125" => "1.25",
        "150" => "1.5",
        "200" => "2"
    },
    GRAYSCALE => {
        "0" => "0",
        "DEFAULT" => "100%"
    },
    HUE_ROTATE => {
        "0" => "0deg",
        "15" => "15deg",
        "30" => "30deg",
        "60" => "60deg",
        "90" => "90deg",
        "180" => "180deg"
    },
    INVERT => {
        "0" => "0",
        "DEFAULT" => "100%"
    },
    SATURATE => {
        "0" => "0",
        "50" => ".5",
        "100" => "1",
        "150" => "1.5",
        "200" => "2"
    },
    SEPIA => {
        "0" => "0",
        "DEFAULT" => "100%"
    },
    FLEX => {
        "1" => "1 1 0%",
        "auto" => "1 1 auto",
        "initial" => "0 1 auto",
        "none" => "none"
    },
    FLEX_SHRINK => {
        "0" => "0",
        "DEFAULT" => "1"
    },
    FLEX_GROW => {
        "0" => "0",
        "DEFAULT" => "1"
    },
    // FLEX_BASIS => {
    BORDER_RADIUS => {
        "none" => "0",
        "sm" => "0.125rem",
        "DEFAULT" => "0.25rem",
        "md" => "0.375rem",
        "lg" => "0.5rem",
        "xl" => "0.75rem",
        "2xl" => "1rem",
        "3xl" => "1.5rem",
        "full" => "9999px"
    },
    // scale: {
    //     0: '0',
    //     50: '.5',
    //     75: '.75',
    //     90: '.9',
    //     95: '.95',
    //     100: '1',
    //     105: '1.05',
    //     110: '1.1',
    //     125: '1.25',
    //     150: '1.5',
    //   },
    SCALE => {
        "0" => "0",
        "50" => ".5",
        "75" => ".75",
        "90" => ".9",
        "95" => ".95",
        "100" => "1",
        "105" => "1.05",
        "110" => "1.1",
        "125" => "1.25",
        "150" => "1.5"
    },
    // 0: '0deg',
    // 1: '1deg',
    // 2: '2deg',
    // 3: '3deg',
    // 6: '6deg',
    // 12: '12deg',
    // 45: '45deg',
    // 90: '90deg',
    // 180: '180deg',
    ROTATE => {
        "0" => "0deg",
        "1" => "1deg",
        "2" => "2deg",
        "3" => "3deg",
        "6" => "6deg",
        "12" => "12deg",
        "45" => "45deg",
        "90" => "90deg",
        "180" => "180deg"
    },
    // 0: '0deg',
    // 1: '1deg',
    // 2: '2deg',
    // 3: '3deg',
    // 6: '6deg',
    // 12: '12deg',
    SKEW => {
        "0" => "0deg",
        "1" => "1deg",
        "2" => "2deg",
        "3" => "3deg",
        "6" => "6deg",
        "12" => "12deg"
    },
    TEXT_DECORATION_THICKNESS => {
        "auto" => "auto",
        "from-font" => "from-font",
        "0" => "0px",
        "1" => "1px",
        "2" => "2px",
        "4" => "4px",
        "8" => "8px"
    },
    GRID_AUTO_COLUMNS => {
        "auto" => "auto",
        "min" => "min-content",
        "max" => "max-content",
        "fr" => "minmax(0, 1fr)"
    },
    GRID_AUTO_ROWS => {
        "auto" => "auto",
        "min" => "min-content",
        "max" => "max-content",
        "fr" => "minmax(0, 1fr)"
    }
);
