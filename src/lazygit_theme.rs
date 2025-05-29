use std::{collections::HashMap, sync::LazyLock};

use crate::{
    Config,
    palette::{PALETTES, Palette},
};

pub struct LazygitTheme {
    pub active_border_color: String,
    pub inactive_border_color: String,
    pub options_text_color: String,
    pub selected_line_bg_color: String,
    pub cherry_picked_commit_bg_color: String,
    pub cherry_picked_commit_fg_color: String,
    pub unstaged_changes_color: String,
    pub default_fg_color: String,
    pub searching_active_border_color: String,
    pub author_colors: String,
}

impl LazygitTheme {
    pub fn named(name: &str) -> Option<Self> {
        PALETTES.get(name).map(|pal| Self::from_palette(pal))
    }

    pub fn from_palette(palette: &Palette) -> Self {
        Self {
            active_border_color: palette.green.clone(),
            inactive_border_color: palette.fg2.clone(),
            options_text_color: palette.fg0.clone(),
            selected_line_bg_color: palette.sel0.clone(),
            cherry_picked_commit_bg_color: palette.sel0.clone(),
            cherry_picked_commit_fg_color: palette.fg1.clone(),
            unstaged_changes_color: palette.red.clone(),
            default_fg_color: palette.fg0.clone(),
            searching_active_border_color: palette.orange.clone(),
            author_colors: palette.purple.clone(),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        let mut result = String::new();

        result += &format!("gui:\n");
        result += &format!("  theme:\n");
        result += &format!("    activeBorderColor:\n");
        result += &format!("    - '{}'\n", self.active_border_color);
        result += &format!("    - bold\n");
        result += &format!("    inactiveBorderColor:\n");
        result += &format!("    - '{}'\n", self.inactive_border_color);
        result += &format!("    selectedLineBgColor:\n");
        result += &format!("    - '{}'\n", self.selected_line_bg_color);
        result += &format!("    optionsTextColor:\n");
        result += &format!("    - '{}'\n", self.options_text_color);
        result += &format!("    cherryPickedCommitBgColor:\n");
        result += &format!("    - '{}'\n", self.cherry_picked_commit_bg_color);
        result += &format!("    cherryPickedCommitFgColor:\n");
        result += &format!("    - '{}'\n", self.cherry_picked_commit_fg_color);
        result += &format!("    unstagedChangesColor:\n");
        result += &format!("    - '{}'\n", self.unstaged_changes_color);
        result += &format!("    defaultFgColor:\n");
        result += &format!("    - '{}'\n", self.default_fg_color);
        result += &format!("    searchingActiveBorderColor:\n");
        result += &format!("    - '{}'\n", self.searching_active_border_color);
        result += &format!("  authorColors:\n");
        result += &format!("    '*': '{}'\n", self.author_colors);

        result
    }

    pub fn apply(&self, config: &Config) {
        let contents = self.to_string();
        std::fs::write(config.lazygit_config.clone(), contents).unwrap();
    }
}
