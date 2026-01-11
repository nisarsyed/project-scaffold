use anyhow::{Context, Result};
use rust_embed::RustEmbed;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(RustEmbed)]
#[folder = "bundled-templates/"]
struct BundledTemplates;

/// Get the path where bundled templates are extracted
pub fn get_bundled_templates_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("scaffold")
        .join("bundled-templates")
}

/// Extract bundled templates if not already present or if version changed
/// Returns path to bundled templates directory
pub fn ensure_bundled_templates() -> Result<PathBuf> {
    let bundled_dir = get_bundled_templates_dir();
    let marker = bundled_dir.join(".version");
    let current_version = env!("CARGO_PKG_VERSION");

    let needs_extraction = match fs::read_to_string(&marker) {
        Ok(v) => v.trim() != current_version,
        Err(_) => true,
    };

    if needs_extraction {
        extract_bundled_templates(&bundled_dir)?;
        fs::write(&marker, current_version).context("Failed to write version marker")?;
    }

    Ok(bundled_dir)
}

fn extract_bundled_templates(dest: &Path) -> Result<()> {
    // Clear old bundled templates
    if dest.exists() {
        fs::remove_dir_all(dest).context("Failed to remove old bundled templates")?;
    }
    fs::create_dir_all(dest).context("Failed to create bundled templates directory")?;

    for file_path in BundledTemplates::iter() {
        let file = BundledTemplates::get(&file_path).context("Failed to read embedded file")?;
        let dest_path = dest.join(file_path.as_ref());

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
        fs::write(&dest_path, file.data.as_ref())
            .with_context(|| format!("Failed to write file: {}", dest_path.display()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundled_templates_exist() {
        // Verify that bundled templates are embedded
        let files: Vec<_> = BundledTemplates::iter().collect();
        assert!(!files.is_empty(), "No bundled templates found");

        // Check for expected templates
        let has_fastapi = files.iter().any(|f| f.starts_with("fastapi/"));
        let has_nextjs = files.iter().any(|f| f.starts_with("nextjs/"));
        assert!(has_fastapi, "fastapi template not found");
        assert!(has_nextjs, "nextjs template not found");
    }

    #[test]
    fn test_bundled_templates_have_config() {
        // Verify each template has template.toml
        assert!(BundledTemplates::get("fastapi/template.toml").is_some());
        assert!(BundledTemplates::get("nextjs/template.toml").is_some());
    }
}
