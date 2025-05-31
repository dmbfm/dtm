use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::Config;
use crate::ghostty_theme::GhosttyTheme;
use crate::helix_theme::HelixTheme;
use crate::lazygit_theme::LazygitTheme;
use crate::read_to_string;

pub struct Theme {
    name: String,
    ghostty: GhosttyTheme,
    helix: HelixTheme,
    lazygit: LazygitTheme,
}

pub static THEMES: LazyLock<HashMap<String, Theme>> = LazyLock::new(|| {
    HashMap::from([
        (
            "nord".to_owned(),
            Theme {
                name: "nord".to_owned(),
                ghostty: GhosttyTheme::Builtin("nord".to_owned()),
                helix: HelixTheme::Builtin("nord-transparent".to_owned()),
                lazygit: LazygitTheme::named("nord").unwrap(),
            },
        ),
        (
            "nord-light".to_owned(),
            Theme {
                name: "nord-light".to_owned(),
                ghostty: GhosttyTheme::Builtin("nord-light".to_owned()),
                helix: HelixTheme::Builtin("nord_light".to_owned()),
                lazygit: LazygitTheme::named("nord-light").unwrap(),
            },
        ),
        (
            "rose-pine-dawn".to_owned(),
            Theme {
                name: "rose-pine-dawn".to_owned(),
                ghostty: GhosttyTheme::Builtin("rose-pine-dawn".to_owned()),
                helix: HelixTheme::Builtin("rose-pine-dawn-transparent".to_owned()),
                lazygit: LazygitTheme::named("rose-pine-dawn").unwrap(),
            },
        ),
    ])
});

impl Theme {
    pub fn apply(&self, config: &Config) -> Result<()> {
        self.ghostty.apply(config)?;
        self.helix.apply(config)?;
        self.lazygit.apply(config)?;
        Ok(())
    }
}
