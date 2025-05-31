use anyhow::{Context, Result};
use std::fs::read_to_string;

use crate::Config;

#[derive(Clone)]
pub enum HelixTheme {
    Builtin(String),
}

impl HelixTheme {
    pub fn apply(&self, config: &Config) -> Result<()> {
        let contents = read_to_string(config.helix_config.clone())
            .context("Failed to read helix config file!")?;
        std::fs::write(config.helix_config.clone(), self.transform(&contents))
            .context("Failed to write helix config file!")?;
        Ok(())
    }

    pub fn transform(&self, contents: &str) -> String {
        match self {
            HelixTheme::Builtin(name) => {
                let mut doc = contents.parse::<toml_edit::DocumentMut>().unwrap();
                doc["theme"] = toml_edit::value(name);
                doc.to_string()
            }
        }
    }
}
