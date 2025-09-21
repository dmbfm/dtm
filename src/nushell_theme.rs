use anyhow::Result;

use crate::config::Config;

#[derive(Clone)]
pub struct NushellTheme(pub String);

impl NushellTheme {
    pub fn apply(&self, config: &Config) -> Result<()> {
        let _ = std::fs::copy(
            config.nushell_themes_dir.join(self.0.clone() + ".nu"),
            config.nushell_themes_dir.join("current.nu"),
        )?;
        Ok(())
    }
}
