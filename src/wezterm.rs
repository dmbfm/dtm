use anyhow::{Context, Result};
use std::{fs::read_to_string, io::Write, process::Command};

use crate::Config;

#[derive(Clone)]
pub enum WeztermTheme {
    Builtin(String),
}

impl WeztermTheme {
    pub fn apply(&self, config: &Config) -> Result<Self> {
        let mut contents = read_to_string(config.wezterm_config.clone())
            .context("Failed to read ghostty config file!")?;
        std::fs::write(config.wezterm_config.clone(), self.transform(&contents))
            .context("Failed to write ghostty config!")?;
        Ok(self.clone())
    }

    pub fn transform(&self, contents: &str) -> String {
        match self {
            WeztermTheme::Builtin(str) => {
                let mut done = false;
                let mut result: String = contents
                    .lines()
                    .map(|line| {
                        if line.starts_with("config.color_scheme =") {
                            done = true;
                            format!("config.color_scheme = '{}'", str)
                        } else {
                            line.to_owned()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                if !done {
                    result.push_str(&format!("config.color_scheme = '{}'\n", str));
                }

                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wezterm::WeztermTheme;

    #[test]
    fn test_wezterm_transform() {
        let contents = [
            "config.front_end = \"WebGpu\"",
            "config.window_decorations = \"RESIZE\"",
            "config.use_fancy_tab_bar = true",
            "config.tab_bar_at_bottom = true",
            "config.hide_tab_bar_if_only_one_tab = true",
            "-- config.window_background_opacity = 1.0",
            "-- config.macos_window_background_blur = 50",
            "config.default_cursor_style = 'SteadyBar'",
            "config.cursor_thickness = 2",
            "config.show_new_tab_button_in_tab_bar = false",
            "config.color_scheme = 'Everforest Dark (Gogh)'",
        ]
        .join("\n");

        let result = WeztermTheme::Builtin("nord".to_owned()).transform(&contents);
        let expected = [
            "config.front_end = \"WebGpu\"",
            "config.window_decorations = \"RESIZE\"",
            "config.use_fancy_tab_bar = true",
            "config.tab_bar_at_bottom = true",
            "config.hide_tab_bar_if_only_one_tab = true",
            "-- config.window_background_opacity = 1.0",
            "-- config.macos_window_background_blur = 50",
            "config.default_cursor_style = 'SteadyBar'",
            "config.cursor_thickness = 2",
            "config.show_new_tab_button_in_tab_bar = false",
            "config.color_scheme = 'nord'",
        ]
        .join("\n");

        assert_eq!(result, expected);
    }
}
