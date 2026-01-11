use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::Path;

use crate::template::{TemplateSource, get_available_templates};

pub fn remove_template(templates_dir: &Path, template_name: &str) -> Result<()> {
    let templates = get_available_templates(templates_dir)?;

    let template = templates
        .iter()
        .find(|t| t.dir_name == template_name)
        .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", template_name))?;

    // Block removal of bundled templates
    if template.source == TemplateSource::Bundled {
        anyhow::bail!(
            "Cannot remove bundled template '{}'. \
             To override it, add a local template with the same name: scaffold add <path> {}",
            template_name,
            template_name
        );
    }

    // Only remove local templates
    let template_path = templates_dir.join(template_name);

    if !template_path.exists() {
        anyhow::bail!("Template '{}' not found in local templates", template_name);
    }

    fs::remove_dir_all(&template_path)
        .with_context(|| format!("Failed to remove template '{}'", template_name))?;

    println!("Template '{}' removed.", style(template_name).cyan().bold());

    Ok(())
}
