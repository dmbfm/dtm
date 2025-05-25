#![allow(dead_code, unused)]
use std::{
    env,
    ffi::FromBytesUntilNulError,
    fs::read_to_string,
    io::Write,
    os::unix::fs::FileTypeExt,
    path::{Path, PathBuf},
    process::{self, Command},
};

const GHOST_OSASCRIPT: &'static str = r#"
tell application "System Events"
    tell process "Ghostty"
        click menu item "Reload Configuration" of menu "Ghostty" of menu bar item "Ghostty" of menu bar 1
    end tell
end tell
"#;

fn ghostty_reload_config() {
    let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
    tmp_file.write_all(GHOST_OSASCRIPT.as_bytes()).unwrap();

    let status = Command::new("osascript")
        .arg(format!("{}", tmp_file.path().to_str().unwrap()))
        .status()
        .unwrap();

    if !status.success() {
        panic!("fail!");
    }
}

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

struct Theme {
    name: String,
    ghostty: Ghostty,
    helix: Helix,
    lazygit: Lazygit,
}

impl Theme {
    fn builtin(name: &str, ghostty_name: &str, helix_name: &str, lazygit_name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ghostty: Ghostty::Builtin(ghostty_name.to_owned()),
            helix: Helix::Builtin(helix_name.to_owned()),
            lazygit: Lazygit {
                name: lazygit_name.to_owned(),
            },
        }
    }

    fn nord() -> Self {
        Self {
            name: "nord".to_owned(),
            ghostty: Ghostty::Builtin("nord".to_owned()),
            helix: Helix::Builtin("nord".to_owned()),
            lazygit: Lazygit {
                name: "nord".to_owned(),
            },
        }
    }

    fn apply(&self, config: &Config) {
        self.ghostty.apply(config);
        self.helix.apply(config);
        self.lazygit.apply(config);
    }
}

#[derive(Clone)]
enum Ghostty {
    Builtin(String),
}

impl Ghostty {
    fn apply(&self, config: &Config) -> Self {
        let mut contents = read_to_string(config.ghostty_config.clone()).unwrap();
        std::fs::write(config.ghostty_config.clone(), self.transform(&contents)).unwrap();
        ghostty_reload_config();
        self.clone()
    }

    fn reload(self) -> Self {
        ghostty_reload_config();
        self
    }

    fn transform(&self, contents: &str) -> String {
        match self {
            Ghostty::Builtin(str) => {
                let mut done = false;
                let mut result: String = contents
                    .lines()
                    .map(|line| {
                        if line.starts_with("theme =") {
                            done = true;
                            format!("theme = {}", str)
                        } else {
                            line.to_owned()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                if !done {
                    result.push_str(&format!("theme = {}\n", str));
                }

                result
            }
        }
    }
}

#[derive(Clone)]
enum Helix {
    Builtin(String),
}

impl Helix {
    fn apply(&self, config: &Config) {
        let contents = read_to_string(config.helix_config.clone()).unwrap();
        std::fs::write(config.helix_config.clone(), self.transform(&contents)).unwrap();
    }

    fn transform(&self, contents: &str) -> String {
        match self {
            Helix::Builtin(name) => {
                let mut doc = contents.parse::<toml_edit::DocumentMut>().unwrap();
                doc["theme"] = toml_edit::value(name);
                doc.to_string()
            }
        }
    }
}

struct Lazygit {
    name: String,
}

impl Lazygit {
    fn apply(&self, config: &Config) {
        let theme = if self.name == "nord" {
            LazygitTheme::nord()
        } else if self.name == "nord-light" {
            LazygitTheme::nord_light()
        } else {
            panic!("Invalid lazygit theme!");
        };

        let contents = theme.to_string();
        std::fs::write(config.lazygit_config.clone(), contents).unwrap();
    }
}

struct LazygitTheme {
    active_border_color: String,
    inactive_border_color: String,
    options_text_color: String,
    selected_line_bg_color: String,
    cherry_picked_commit_bg_color: String,
    cherry_picked_commit_fg_color: String,
    unstaged_changes_color: String,
    default_fg_color: String,
    searching_active_border_color: String,
    author_colors: String,
}

struct NordPalette {
    colors: Vec<String>,
}

impl NordPalette {
    fn nord(&self, index: usize) -> String {
        self.colors[index].clone()
    }

    fn new() -> Self {
        let colors = vec![
            "#2e3440".to_owned(),
            "#3b4252".to_owned(),
            "#434c5e".to_owned(),
            "#4c566a".to_owned(),
            "#d8dee9".to_owned(),
            "#e5e9f0".to_owned(),
            "#eceff4".to_owned(),
            "#8fbcbb".to_owned(),
            "#88c0d0".to_owned(),
            "#81a1c1".to_owned(),
            "#5e81ac".to_owned(),
            "#bf616a".to_owned(),
            "#d08770".to_owned(),
            "#ebcb8b".to_owned(),
            "#a3be8c".to_owned(),
            "#b48ead".to_owned(),
        ];
        Self { colors }
    }
}

impl LazygitTheme {
    fn nord() -> Self {
        let pal = NordPalette::new();
        Self {
            active_border_color: pal.nord(14),
            inactive_border_color: pal.nord(3),
            options_text_color: pal.nord(2),
            selected_line_bg_color: pal.nord(3),
            cherry_picked_commit_bg_color: pal.nord(3),
            cherry_picked_commit_fg_color: pal.nord(11),
            unstaged_changes_color: pal.nord(12),
            default_fg_color: pal.nord(4),
            searching_active_border_color: pal.nord(15),
            author_colors: pal.nord(14),
        }
    }

    fn nord_light() -> Self {
        Self::nord()
    }

    fn to_string(&self) -> String {
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
}

fn main() {
    let config = Config::default();

    let first_arg = std::env::args_os()
        .skip(1)
        .rev()
        .last()
        .map(|x| x.into_string().unwrap());

    let dark_theme = Theme::builtin("nord", "nord", "nord", "nord");
    let light_theme = Theme::builtin("nord-light", "nord-light", "nord_light", "nord-light");
    // let light_theme = Theme::builtin("rose-pine-dawn", "rose-pine-dawn", "rose_pine_dawn");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghostty_transform() {
        let contents = [
            "font-size = 14",
            "theme = gruvbox",
            "background-opacity = 1.0",
        ]
        .join("\n");

        let result = Ghostty::Builtin("nord".to_owned()).transform(&contents);
        let expected = ["font-size = 14", "theme = nord", "background-opacity = 1.0"].join("\n");

        assert_eq!(result, expected);
    }
}
