use anyhow::Result;
use console::style;
use std::path::Path;

use crate::template::{TemplateSource, get_available_templates};

pub fn show_template_info(templates_dir: &Path, template_name: &str) -> Result<()> {
    let templates = get_available_templates(templates_dir)?;

    let template = templates
        .iter()
        .find(|t| t.dir_name == template_name)
        .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", template_name))?;

    let config = &template.config;

    let source_tag = match template.source {
        TemplateSource::Bundled => format!(" {}", style("[bundled]").dim()),
        TemplateSource::Local => String::new(),
    };

    println!("{}{}", style(&config.name).cyan().bold(), source_tag);
    println!("{}\n", config.description);

    if config.variables.is_empty() {
        println!("No variables defined.");
    } else {
        println!("Variables:\n");
        for var in &config.variables {
            let var_type = var.var_type.as_deref().unwrap_or("string");
            let type_str = match var_type {
                "choice" => format!(" [{}]", var.choices.join("|")),
                "bool" => " [yes/no]".to_string(),
                _ => String::new(),
            };
            let default_str = var
                .default
                .as_ref()
                .map(|d| format!(" (default: {})", style(d).dim()))
                .unwrap_or_default();
            println!(
                "  {}{} - {}{}",
                style(&var.name).green().bold(),
                style(&type_str).dim(),
                var.description,
                default_str
            );
        }
    }

    if !config.conditionals.is_empty() {
        println!("\nConditional files:\n");
        for cond in &config.conditionals {
            let action = if cond.include.is_some() {
                format!("include {}", cond.include.as_ref().unwrap())
            } else {
                format!("exclude {}", cond.exclude.as_ref().unwrap())
            };
            println!(
                "  {} when {}",
                style(&action).cyan(),
                style(&cond.when).dim()
            );
        }
    }

    if let Some(ref hooks) = config.hooks {
        if !hooks.post_create.is_empty() {
            println!("\nPost-create hooks:\n");
            for cmd in &hooks.post_create {
                println!("  {}", style(cmd).dim());
            }
        }
    }

    Ok(())
}
