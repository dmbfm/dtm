use std::path::PathBuf;

use crate::theme::THEMES;

#[derive(Debug)]
pub struct Config {
    pub light_theme: String,
    pub dark_theme: String,
    pub wezterm_config: PathBuf,
    pub helix_config: PathBuf,
    pub lazygit_config: PathBuf,
    pub nushell_themes_dir: PathBuf,
    pub wallpapers_dir: PathBuf,
    pub lock_file: PathBuf,
}

impl Config {
    fn new(
        light_theme: String,
        dark_theme: String,
        wezterm_config: PathBuf,
        helix_config: PathBuf,
        lazygit_config: PathBuf,
        nushell_themes_dir: PathBuf,
        wallpapers_dir: PathBuf,
        lock_file: PathBuf,
    ) -> Self {
        Self {
            light_theme,
            dark_theme,
            wezterm_config,
            helix_config,
            lazygit_config,
            wallpapers_dir,
            lock_file,
            nushell_themes_dir,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap();

        Self {
            light_theme: "everforest-light".to_owned(),
            dark_theme: "everforest-dark".to_owned(),
            wezterm_config: format!("{home}/.config/wezterm/wezterm.lua").into(),
            helix_config: format!("{home}/.config/helix/config.toml").into(),
            lazygit_config: format!("{home}/Library/Application Support/lazygit/config.yml").into(),
            wallpapers_dir: format!("{home}/Documents/Wallpapers").into(),
            lock_file: format!("{home}/Library/Application Support/dtm/lockfile").into(),
            nushell_themes_dir: format!("{home}/Library/Application Support/nushell/themes").into(),
        }
    }
}
