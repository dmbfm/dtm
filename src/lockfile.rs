use std::fs::read_to_string;

use anyhow::{Context, Result};

use crate::config::Config;

pub struct Lockfile {
    pub theme_name: String,
}

impl Lockfile {
    pub fn read(config: &Config) -> Option<Self> {
        let mut name = read_to_string(config.lock_file.clone()).ok()?;
        name.trim();
        Some({ Self { theme_name: name } })
    }

    pub fn write(&self, config: &Config) -> Result<()> {
        std::fs::write(config.lock_file.clone(), &self.theme_name)
            .context("Failed to write lockfile")?;
        Ok(())
    }
}
