use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::Config;
// use crate::ghostty_theme::GhosttyTheme;
use crate::helix_theme::HelixTheme;
use crate::lazygit_theme::LazygitTheme;
use crate::read_to_string;
use crate::wezterm::WeztermTheme;

pub struct Theme {
    name: String,
    // ghostty: GhosttyTheme,
    wezterm: WeztermTheme,
    helix: HelixTheme,
    lazygit: LazygitTheme,
}

pub static THEMES: LazyLock<HashMap<String, Theme>> = LazyLock::new(|| {
    HashMap::from([
        (
            "nord".to_owned(),
            Theme {
                name: "nord".to_owned(),
                // ghostty: GhosttyTheme::Builtin("nord".to_owned()),
                helix: HelixTheme::Builtin("nord-transparent".to_owned()),
                lazygit: LazygitTheme::named("nord").unwrap(),
                wezterm: WeztermTheme::Builtin("Nord (Gogh)".to_owned()),
            },
        ),
        (
            "nord-light".to_owned(),
            Theme {
                name: "nord-light".to_owned(),
                helix: HelixTheme::Builtin("nord_light".to_owned()),
                lazygit: LazygitTheme::named("nord-light").unwrap(),
                wezterm: WeztermTheme::Builtin("Nord Light (Gogh)".to_owned()),
            },
        ),
        (
            "rose-pine-dawn".to_owned(),
            Theme {
                name: "rose-pine-dawn".to_owned(),
                helix: HelixTheme::Builtin("rose-pine-dawn-transparent".to_owned()),
                lazygit: LazygitTheme::named("rose-pine-dawn").unwrap(),
                wezterm: WeztermTheme::Builtin("rose-pine-dawn".to_owned()),
            },
        ),
        (
            "everforest-dark".to_owned(),
            Theme {
                name: "everforest-dark".to_owned(),
                wezterm: WeztermTheme::Builtin("Everforest Dark (Gogh)".to_owned()),
                helix: HelixTheme::Builtin("everforest_dark".to_owned()),
                lazygit: LazygitTheme::named("everforest-dark").unwrap(),
            },
        ),
        (
            "everforest-light".to_owned(),
            Theme {
                name: "everforest-light".to_owned(),
                wezterm: WeztermTheme::Builtin("Everforest Light (Gogh)".to_owned()),
                helix: HelixTheme::Builtin("everforest_light".to_owned()),
                lazygit: LazygitTheme::named("everforest-light").unwrap(),
            },
        ),
    ])
});

impl Theme {
    pub fn apply(&self, config: &Config) -> Result<()> {
        self.helix.apply(config)?;
        self.lazygit.apply(config)?;
        self.wezterm.apply(config)?;
        Ok(())
    }
}
