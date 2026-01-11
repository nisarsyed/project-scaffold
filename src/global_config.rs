use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Default values for common variables (author, author_email, etc.)
    #[serde(default)]
    pub defaults: HashMap<String, String>,
}

/// Get the path to the global config file
pub fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("scaffold")
        .join("config.toml")
}

/// Load global configuration from disk
pub fn load_global_config() -> GlobalConfig {
    let path = get_config_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        GlobalConfig::default()
    }
}

/// Save global configuration to disk
pub fn save_global_config(config: &GlobalConfig) -> Result<()> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create config directory at '{}'",
                parent.display()
            )
        })?;
    }
    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;
    fs::write(&path, content)
        .with_context(|| format!("Failed to write config to '{}'", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_serialization() {
        let mut config = GlobalConfig::default();
        config
            .defaults
            .insert("author".to_string(), "Test Author".to_string());
        config
            .defaults
            .insert("author_email".to_string(), "test@example.com".to_string());

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: GlobalConfig = toml::from_str(&serialized).unwrap();

        assert_eq!(deserialized.defaults.get("author").unwrap(), "Test Author");
        assert_eq!(
            deserialized.defaults.get("author_email").unwrap(),
            "test@example.com"
        );
    }

    #[test]
    fn test_global_config_empty() {
        let config = GlobalConfig::default();
        assert!(config.defaults.is_empty());
    }
}
