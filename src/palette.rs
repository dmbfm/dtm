use std::{collections::HashMap, sync::LazyLock};

pub struct Palette {
    /// Base background
    pub bg0: String,
    /// Secondary background
    pub bg1: String,
    /// Tertiary background (popovers)
    pub bg2: String,
    /// Text
    pub fg0: String,
    /// Secondary text, comments, etc.
    pub fg1: String,
    /// Tertiary text, disabled elements, etc.
    pub fg2: String,
    /// Cursorline
    pub sel0: String,
    /// Selection background
    pub sel1: String,
    /// Borders and dividers
    pub sel2: String,
    pub red: String,
    pub yellow: String,
    pub blue: String,
    pub green: String,
    pub orange: String,
    pub purple: String,
}

pub static PALETTES: LazyLock<HashMap<String, Palette>> = LazyLock::new(|| {
    HashMap::from([
        (
            "rose-pine-dawn".to_owned(),
            Palette {
                bg0: "#faf4ed".to_owned(),
                bg1: "#fffaf3".to_owned(),
                bg2: "#f2e9e1".to_owned(),
                fg0: "#575279".to_owned(),
                fg1: "#797593".to_owned(),
                fg2: "#9893a5".to_owned(),
                sel0: "#f4ede8".to_owned(),
                sel1: "#dfdad9".to_owned(),
                sel2: "#cecacd".to_owned(),
                red: "#b4637a".to_owned(),
                yellow: "#ea9d34".to_owned(),
                blue: "#286983".to_owned(),
                green: "#56949f".to_owned(),
                orange: "#d7827e".to_owned(),
                purple: "#907aa9".to_owned(),
            },
        ),
        (
            "nord".to_owned(),
            Palette {
                bg0: "#2E3440".to_owned(),
                bg1: "#3B4252".to_owned(),
                bg2: "#4C566A".to_owned(),
                fg0: "#ECEFF4".to_owned(),
                fg1: "#E5E9F0".to_owned(),
                fg2: "#D8DEE9".to_owned(),
                sel0: "#434C5E".to_owned(),
                sel1: "#434C5E".to_owned(),
                sel2: "#4C566A".to_owned(),
                red: "#BF616A".to_owned(),
                yellow: "#EBCB8B".to_owned(),
                blue: "#81A1C1".to_owned(),
                green: "#8FBCBB".to_owned(),
                orange: "#D08770".to_owned(),
                purple: "#B48EAD".to_owned(),
            },
        ),
        (
            "nord-light".to_owned(),
            Palette {
                bg0: "#ECEFF4".to_owned(),
                bg1: "#E5E9F0".to_owned(),
                bg2: "#D8DEE9".to_owned(),
                fg0: "#2E3440".to_owned(),
                fg1: "#3B4252".to_owned(),
                fg2: "#4C566A".to_owned(),
                sel0: "#E5E9F0".to_owned(),
                sel1: "#E5E9F0".to_owned(),
                sel2: "#ECEFF4".to_owned(),
                red: "#BF616A".to_owned(),
                yellow: "#EBCB8B".to_owned(),
                blue: "#81A1C1".to_owned(),
                green: "#8FBCBB".to_owned(),
                orange: "#D08770".to_owned(),
                purple: "#B48EAD".to_owned(),
            },
        ),
    ])
});
