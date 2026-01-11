use anyhow::{Context, Result, ensure};
use console::style;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::copy::copy_template_recursive;
use crate::git::{clone_repo, is_git_url, parse_git_url};

pub fn add_template(templates_dir: &Path, template_path: &str, template_name: &str) -> Result<()> {
    validate_template_name(template_name)?;

    let dst = templates_dir.join(template_name);

    fs::create_dir_all(templates_dir).context("Failed to create templates directory")?;

    if dst.exists() {
        anyhow::bail!("Template '{}' already exists", template_name);
    }

    // Determine source path (local or remote)
    let (src, temp_dir) = if is_git_url(template_path) {
        let parsed = parse_git_url(template_path);
        let cloned_dir = clone_repo(&parsed.repo_url)?;

        let src = match &parsed.subpath {
            Some(subpath) => cloned_dir.join(subpath),
            None => cloned_dir.clone(),
        };

        (src, Some(cloned_dir))
    } else {
        (PathBuf::from(template_path), None)
    };

    // Validate source exists
    if !src.exists() {
        if let Some(temp) = &temp_dir {
            let _ = fs::remove_dir_all(temp);
        }
        anyhow::bail!("Template path '{}' does not exist", src.display());
    }

    // Validate template.toml exists
    let config_path = src.join("template.toml");
    if !config_path.exists() {
        if let Some(temp) = &temp_dir {
            let _ = fs::remove_dir_all(temp);
        }
        anyhow::bail!("template.toml not found in template directory");
    }

    // Copy template (no exclusions, no progress bar for add)
    copy_template_recursive(&src, &dst, &HashMap::new(), &HashSet::new(), false, None)?;

    // Clean up temp directory
    if let Some(temp) = temp_dir {
        let _ = fs::remove_dir_all(temp);
    }

    println!(
        "Template '{}' added successfully.",
        style(template_name).cyan().bold()
    );
    Ok(())
}

fn validate_template_name(name: &str) -> Result<()> {
    ensure!(!name.is_empty(), "Template name cannot be empty");
    ensure!(
        !name.contains('/') && !name.contains('\\'),
        "Template name cannot contain path separators"
    );
    ensure!(
        !name.starts_with('.'),
        "Template name cannot start with '.'"
    );
    ensure!(
        name.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
        "Template name can only contain alphanumeric characters, hyphens, and underscores"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_template_name_valid() {
        assert!(validate_template_name("my-template").is_ok());
        assert!(validate_template_name("template_123").is_ok());
        assert!(validate_template_name("Template").is_ok());
    }

    #[test]
    fn test_validate_template_name_empty() {
        assert!(validate_template_name("").is_err());
    }

    #[test]
    fn test_validate_template_name_with_slash() {
        assert!(validate_template_name("my/template").is_err());
        assert!(validate_template_name("my\\template").is_err());
    }

    #[test]
    fn test_validate_template_name_starts_with_dot() {
        assert!(validate_template_name(".hidden").is_err());
    }

    #[test]
    fn test_validate_template_name_special_chars() {
        assert!(validate_template_name("my template").is_err());
        assert!(validate_template_name("my@template").is_err());
    }
}
