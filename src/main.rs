#![allow(dead_code, unused)]
// DTM: Daniel`s theme manager
//
// TODO:
// - Change desktop background to match theme (i.e., add a desktop theme target)

mod config;
// mod ghostty_theme;
mod helix_theme;
mod lazygit_theme;
mod lockfile;
mod palette;
mod theme;
mod wezterm;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use config::Config;
use lockfile::Lockfile;
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

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    SetTheme {
        name: String,
    },
    SetVariation {
        #[arg(value_enum)]
        variation: Variation,
    },
    Toggle,
    Refresh,
    List,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Variation {
    Dark,
    Light,
}

impl Cli {
    fn set_theme(config: &Config, name: &str) -> Result<()> {
        let theme = THEMES
            .get(name)
            .context(format!("Theme '{name}' not found!"))?;

        theme.apply(config)?;

        Lockfile {
            theme_name: name.to_owned(),
        }
        .write(config)?;

        Ok(())
    }

    fn set_variation(config: &Config, variation: &Variation) -> Result<()> {
        let name = match variation {
            Variation::Dark => &config.dark_theme,
            Variation::Light => &config.light_theme,
        };

        Cli::set_theme(config, name)?;

        Ok(())
    }

    fn toggle(config: &Config) -> Result<()> {
        match Lockfile::read(&config) {
            Some(lockfile) if lockfile.theme_name == config.light_theme => {
                Self::set_theme(config, &config.dark_theme)?
            }
            Some(lockfile) if lockfile.theme_name == config.dark_theme => {
                Self::set_theme(config, &config.light_theme)?
            }
            Some(lockfile) => {
                println!(
                    "The current theme, '{}', is not listed as preferred dark or light theme; setting theme to preferred dark theme.",
                    lockfile.theme_name
                );
                Self::set_theme(config, &config.dark_theme)?
            }
            None => {
                println!("Lockfile not present; setting theme to preferred dark theme.",);
                Self::set_theme(config, &config.dark_theme)?
            }
        };

        Ok(())
    }

    fn refresh(config: &Config) -> Result<()> {
        println!("refresh");

        match Lockfile::read(&config) {
            Some(lockfile) => Self::set_theme(config, &lockfile.theme_name)?,
            None => {
                println!("Lockfile not present; setting theme to preferred dark theme.",);
                Self::set_theme(config, &config.dark_theme)?
            }
        }

        Ok(())
    }

    fn list() {
        for (name, theme) in THEMES.iter() {
            println!("{}", name);
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config: Config = Config::default();

    match &cli.subcommand {
        Commands::SetTheme { name } => Cli::set_theme(&config, &name)?,
        Commands::SetVariation { variation } => Cli::set_variation(&config, variation)?,
        Commands::Toggle => Cli::toggle(&config)?,
        Commands::Refresh => Cli::refresh(&config)?,
        Commands::List => Cli::list(),
    };

    return Ok(());
}
