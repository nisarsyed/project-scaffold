use anyhow::{Context, Result};
use console::style;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use super::TemplateConfig;
use super::config::{AvailableTemplate, TemplateSource};
use crate::bundled::ensure_bundled_templates;

/// Get all available templates from local .templates/ and bundled templates.
/// Local templates take precedence over bundled templates with the same name.
pub fn get_available_templates(local_dir: &Path) -> Result<Vec<AvailableTemplate>> {
    let mut templates = Vec::new();
    let mut seen_names = HashSet::new();

    // 1. First scan local .templates/ directory (highest priority)
    if local_dir.exists() {
        for template in scan_directory(local_dir, TemplateSource::Local)? {
            seen_names.insert(template.dir_name.clone());
            templates.push(template);
        }
    }

    // 2. Then scan bundled templates (skip if local exists with same name)
    if let Ok(bundled_dir) = ensure_bundled_templates() {
        for template in scan_directory(&bundled_dir, TemplateSource::Bundled)? {
            if !seen_names.contains(&template.dir_name) {
                templates.push(template);
            }
        }
    }

    Ok(templates)
}

/// Scan a directory for templates
fn scan_directory(dir: &Path, source: TemplateSource) -> Result<Vec<AvailableTemplate>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(dir).context("Failed to read templates directory")?;
    let mut templates = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden directories and version marker
            if dir_name.starts_with('.') {
                continue;
            }

            let config_path = path.join("template.toml");

            if !config_path.exists() {
                eprintln!(
                    "{}: Skipping '{}' (no template.toml found)",
                    style("Warning").yellow(),
                    dir_name
                );
                continue;
            }

            match load_template_config(&config_path) {
                Ok(config) => {
                    templates.push(AvailableTemplate {
                        dir_name,
                        config,
                        source,
                        path: path.clone(),
                    });
                }
                Err(e) => {
                    eprintln!(
                        "{}: Skipping '{}': {}",
                        style("Warning").yellow(),
                        dir_name,
                        e
                    );
                }
            }
        }
    }

    Ok(templates)
}

pub fn load_template_config(path: &Path) -> Result<TemplateConfig> {
    let content = fs::read_to_string(path).context("Failed to read template config")?;
    let config: TemplateConfig =
        toml::from_str(&content).context("Failed to parse template config")?;
    Ok(config)
}
