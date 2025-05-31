use std::path::PathBuf;

use crate::theme::THEMES;

#[derive(Debug)]
pub struct Config {
    pub light_theme: String,
    pub dark_theme: String,
    pub ghostty_config: PathBuf,
    pub helix_config: PathBuf,
    pub lazygit_config: PathBuf,
    pub lock_file: PathBuf,
}

impl Config {
    fn new(
        light_theme: String,
        dark_theme: String,
        ghostty_config: PathBuf,
        helix_config: PathBuf,
        lazygit_config: PathBuf,
        lock_file: PathBuf,
    ) -> Self {
        Self {
            light_theme,
            dark_theme,
            ghostty_config,
            helix_config,
            lazygit_config,
            lock_file,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap();

        Self {
            light_theme: "rose-pine-dawn".to_owned(),
            dark_theme: "nord".to_owned(),
            ghostty_config: format!("{home}/.config/ghostty/config").into(),
            helix_config: format!("{home}/.config/helix/config.toml").into(),
            lazygit_config: format!("{home}/Library/Application Support/lazygit/config.yml").into(),
            lock_file: format!("{home}/Library/Application Support/dtm/lockfile").into(),
        }
    }
}
