use anyhow::{Result, ensure};
use console::style;
use dialoguer::{Confirm, FuzzySelect, Input, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::copy::{copy_template_recursive, count_files, preview_template};
use crate::global_config::load_global_config;
use crate::hooks::execute_hooks;
use crate::template::get_available_templates;
use crate::variables::evaluate_conditionals;

pub fn create_project_interactive(
    templates_dir: &Path,
    template: Option<String>,
    output: Option<String>,
    cli_vars: Vec<(String, String)>,
    use_defaults: bool,
    dry_run: bool,
) -> Result<()> {
    let theme = ColorfulTheme::default();
    let templates = get_available_templates(templates_dir)?;

    if templates.is_empty() {
        anyhow::bail!(
            "No templates available. Run 'scaffold add <path> <name>' to add a template first."
        );
    }

    // Select template (interactive if not provided)
    let selected_template = match template {
        Some(name) => templates
            .iter()
            .find(|t| t.dir_name == name)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Template '{}' not found. Run 'scaffold list' to see available templates.",
                    name
                )
            })?,
        None => {
            println!("Select a template:");
            let items: Vec<String> = templates
                .iter()
                .map(|t| format!("{} - {}", t.dir_name, t.config.description))
                .collect();

            let selection = FuzzySelect::with_theme(&theme)
                .items(&items)
                .default(0)
                .interact()?;

            &templates[selection]
        }
    };

    let template_path = &selected_template.path;
    let config = &selected_template.config;

    println!(
        "\nCreating project from: {}",
        style(&config.name).cyan().bold()
    );

    // Get output directory (interactive if not provided)
    let output_str = match output {
        Some(o) => o,
        None => {
            if use_defaults || dry_run {
                anyhow::bail!("Output directory is required when using --yes or --dry-run flag");
            }
            Input::with_theme(&theme)
                .with_prompt("Output directory")
                .interact_text()?
        }
    };

    let output_path = PathBuf::from(&output_str);

    // Skip validation in dry run mode
    if !dry_run {
        validate_output_path(&output_path)?;
    }

    // Build variables map from CLI args
    let mut variables: HashMap<String, String> = cli_vars.into_iter().collect();

    // Auto-derive project_name from output directory when using --yes flag
    if (use_defaults || dry_run) && !variables.contains_key("project_name") {
        if let Some(dir_name) = output_path.file_name().and_then(|n| n.to_str()) {
            // Convert hyphens to underscores for valid Python/package names
            let derived_name = dir_name.replace('-', "_");
            variables.insert("project_name".to_string(), derived_name);
        }
    }

    // Load global config for defaults
    let global_config = load_global_config();

    // Prompt for missing variables
    for var in &config.variables {
        if variables.contains_key(&var.name) {
            continue;
        }

        let var_type = var.var_type.as_deref().unwrap_or("string");

        // Check for default: template default, then global config default
        let effective_default = var
            .default
            .clone()
            .or_else(|| global_config.defaults.get(&var.name).cloned());

        if use_defaults || dry_run {
            // Use default or appropriate fallback
            let value = match var_type {
                "bool" => effective_default.unwrap_or_else(|| "false".to_string()),
                "choice" => effective_default
                    .or_else(|| var.choices.first().cloned())
                    .unwrap_or_default(),
                _ => effective_default.unwrap_or_default(),
            };
            variables.insert(var.name.clone(), value);
        } else {
            // Interactive prompt based on variable type
            let value = match var_type {
                "bool" => {
                    let default_bool = effective_default
                        .as_ref()
                        .is_some_and(|d| d == "true" || d == "yes");
                    Confirm::with_theme(&theme)
                        .with_prompt(&var.description)
                        .default(default_bool)
                        .interact()?
                        .to_string()
                }
                "choice" => {
                    if var.choices.is_empty() {
                        anyhow::bail!(
                            "Variable '{}' is type 'choice' but has no choices defined",
                            var.name
                        );
                    }
                    let default_idx = effective_default
                        .as_ref()
                        .and_then(|d| var.choices.iter().position(|c| c == d))
                        .unwrap_or(0);
                    let selection = Select::with_theme(&theme)
                        .with_prompt(&var.description)
                        .items(&var.choices)
                        .default(default_idx)
                        .interact()?;
                    var.choices[selection].clone()
                }
                _ => {
                    // String type (default)
                    let prompt_text = if var.description.is_empty() {
                        var.name.clone()
                    } else {
                        format!("{} ({})", var.description, &var.name)
                    };

                    match &effective_default {
                        Some(default) => Input::with_theme(&theme)
                            .with_prompt(&prompt_text)
                            .default(default.clone())
                            .interact_text()?,
                        None => Input::with_theme(&theme)
                            .with_prompt(&prompt_text)
                            .allow_empty(true)
                            .interact_text()?,
                    }
                }
            };

            variables.insert(var.name.clone(), value);
        }
    }

    // Build exclusion set from conditionals
    let excluded_files = evaluate_conditionals(&config.conditionals, &variables);

    if dry_run {
        // Preview mode - show what would be created
        println!("\n{}", style("Dry run - no files will be created").yellow());
        println!("\nWould create:\n");
        preview_template(template_path, &output_path, &variables, &excluded_files, 0)?;
        return Ok(());
    }

    // Count files for progress bar
    let file_count = count_files(template_path, &excluded_files)?;
    let progress = ProgressBar::new(file_count as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files")
            .expect("valid progress template")
            .progress_chars("=>-"),
    );

    // Copy and render template
    copy_template_recursive(
        template_path,
        &output_path,
        &variables,
        &excluded_files,
        true,
        Some(&progress),
    )?;

    progress.finish_and_clear();

    // Execute post-create hooks
    if let Some(ref hooks) = config.hooks {
        if !hooks.post_create.is_empty() {
            println!("\nRunning post-create hooks...");
            execute_hooks(&hooks.post_create, &output_path)?;
        }
    }

    println!(
        "\nProject created at: {}\n",
        style(&output_str).green().bold()
    );
    println!("Next steps:\n");
    println!("  cd {}", output_str);

    Ok(())
}

fn validate_output_path(path: &Path) -> Result<()> {
    ensure!(
        !path.exists(),
        "Output directory '{}' already exists",
        path.display()
    );
    Ok(())
}
