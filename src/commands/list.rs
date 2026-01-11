use anyhow::Result;
use console::style;
use std::path::Path;

use crate::template::{TemplateSource, get_available_templates};

pub fn list_templates(templates_dir: &Path) -> Result<()> {
    let templates = get_available_templates(templates_dir)?;

    if templates.is_empty() {
        println!(
            "No templates available. Run '{}' to add a template first.",
            style("scaffold add <path> <name>").cyan()
        );
        return Ok(());
    }

    println!("Available templates:\n");
    for template in templates {
        let source_tag = match template.source {
            TemplateSource::Bundled => format!(" {}", style("[bundled]").dim()),
            TemplateSource::Local => String::new(),
        };
        println!(
            "  {}{} - {}",
            style(&template.dir_name).cyan().bold(),
            source_tag,
            template.config.description
        );
    }

    Ok(())
}
