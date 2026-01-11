use anyhow::{Context, Result};
use clap::Subcommand;
use console::style;
use std::fs;

use crate::global_config::{get_config_path, load_global_config, save_global_config};

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set a default value for a variable (e.g., scaffold config set author "John Doe")
    Set {
        /// Variable name (e.g., author, author_email)
        key: String,
        /// Value to set
        value: String,
    },
    /// Get the current value of a variable
    Get {
        /// Variable name
        key: String,
    },
    /// List all saved defaults
    List,
    /// Remove a saved default
    Unset {
        /// Variable name to remove
        key: String,
    },
    /// Reset all config to defaults
    Reset,
}

/// Handle the config subcommand
pub fn handle_config_command(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Set { key, value } => {
            let mut config = load_global_config();
            config.defaults.insert(key.clone(), value.clone());
            save_global_config(&config)?;
            println!(
                "Set {} = {}",
                style(&key).green().bold(),
                style(&value).cyan()
            );
        }
        ConfigAction::Get { key } => {
            let config = load_global_config();
            match config.defaults.get(&key) {
                Some(value) => println!("{}", value),
                None => println!("{}: not set", style(&key).dim()),
            }
        }
        ConfigAction::List => {
            let config = load_global_config();
            if config.defaults.is_empty() {
                println!("No defaults configured.");
                println!(
                    "\nSet defaults with: {}",
                    style("scaffold config set <key> <value>").dim()
                );
            } else {
                println!("Global defaults:\n");
                let mut keys: Vec<_> = config.defaults.keys().collect();
                keys.sort();
                for key in keys {
                    let value = &config.defaults[key];
                    println!("  {} = {}", style(key).green().bold(), style(value).cyan());
                }
                println!(
                    "\nConfig file: {}",
                    style(get_config_path().display()).dim()
                );
            }
        }
        ConfigAction::Unset { key } => {
            let mut config = load_global_config();
            if config.defaults.remove(&key).is_some() {
                save_global_config(&config)?;
                println!("Removed {}", style(&key).green().bold());
            } else {
                println!("{}: not set", style(&key).dim());
            }
        }
        ConfigAction::Reset => {
            let path = get_config_path();
            if path.exists() {
                fs::remove_file(&path).with_context(|| {
                    format!("Failed to remove config file '{}'", path.display())
                })?;
                println!("Config reset to defaults.");
            } else {
                println!("No config file to reset.");
            }
        }
    }
    Ok(())
}
