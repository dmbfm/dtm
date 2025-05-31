use anyhow::{Context, Result};
use std::{fs::read_to_string, io::Write, process::Command};

use crate::Config;

#[derive(Clone)]
pub enum GhosttyTheme {
    Builtin(String),
}

impl GhosttyTheme {
    pub fn apply(&self, config: &Config) -> Result<Self> {
        let mut contents = read_to_string(config.ghostty_config.clone())
            .context("Failed to read ghostty config file!")?;
        std::fs::write(config.ghostty_config.clone(), self.transform(&contents))
            .context("Failed to write ghostty config!")?;
        ghostty_reload_config();
        Ok(self.clone())
    }

    pub fn reload(self) -> Self {
        ghostty_reload_config();
        self
    }

    pub fn transform(&self, contents: &str) -> String {
        match self {
            GhosttyTheme::Builtin(str) => {
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

#[cfg(test)]
mod tests {
    use crate::ghostty_theme::GhosttyTheme;

    #[test]
    fn test_ghostty_transform() {
        let contents = [
            "font-size = 14",
            "theme = gruvbox",
            "background-opacity = 1.0",
        ]
        .join("\n");

        let result = GhosttyTheme::Builtin("nord".to_owned()).transform(&contents);
        let expected = ["font-size = 14", "theme = nord", "background-opacity = 1.0"].join("\n");

        assert_eq!(result, expected);
    }
}
