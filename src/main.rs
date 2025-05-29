#![allow(dead_code, unused)]
// DTM: Daniel`s theme manager
//

mod ghostty_theme;
mod helix_theme;
mod lazygit_theme;
mod palette;
mod theme;

use theme::THEMES;

use crate::theme::Theme;
use std::{
    collections::HashMap,
    env,
    ffi::FromBytesUntilNulError,
    fs::read_to_string,
    io::Write,
    os::unix::fs::FileTypeExt,
    path::{Path, PathBuf},
    process::{self, Command},
};

#[derive(Debug)]
struct Config {
    ghostty_config: PathBuf,
    helix_config: PathBuf,
    lazygit_config: PathBuf,
    lock_file: PathBuf,
}

impl Config {
    fn new(
        ghostty_config: PathBuf,
        helix_config: PathBuf,
        lazygit_config: PathBuf,
        lock_file: PathBuf,
    ) -> Self {
        Self {
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
            ghostty_config: format!("{home}/.config/ghostty/config").into(),
            helix_config: format!("{home}/.config/helix/config.toml").into(),
            lazygit_config: format!("{home}/Library/Application Support/lazygit/config.yml").into(),
            lock_file: format!("{home}/Library/Application Support/dtm/lockfile").into(),
        }
    }
}

fn main() {
    let config = Config::default();

    let first_arg = std::env::args_os()
        .skip(1)
        .rev()
        .last()
        .map(|x| x.into_string().unwrap());

    let dark_theme_name = "nord";
    let light_theme_name = "rose-pine-dawn";

    let dark_theme = THEMES
        .get(dark_theme_name)
        .expect("Theme 'nord' not found.");
    let light_theme = THEMES
        .get(light_theme_name)
        .expect("Theme 'rose-pine-dawn' not found.");

    let mut current_theme_name = read_to_string(config.lock_file.clone())
        .ok()
        .unwrap_or("dark".to_owned())
        .trim()
        .to_owned();

    if let Some(arg) = first_arg {
        if arg == "light" {
            current_theme_name = "dark".to_owned();
        } else if arg == "dark" {
            current_theme_name = "light".to_owned();
        } else {
            eprintln!("Invalid argument: {}", arg);
            return;
        }
    }

    let new_theme_name = if current_theme_name.trim() == "dark" {
        light_theme.apply(&config);
        "light"
    } else {
        dark_theme.apply(&config);
        "dark"
    };

    std::fs::create_dir_all(config.lock_file.parent().unwrap()).unwrap();
    std::fs::write(config.lock_file.clone(), new_theme_name).expect("Failed to write lockfile");
}
