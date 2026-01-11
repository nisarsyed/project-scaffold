use anyhow::Result;
use console::style;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::template::load_template_config;

pub fn validate_template(template_path: &str) -> Result<()> {
    let path = PathBuf::from(template_path);

    println!("Validating template at: {}\n", style(template_path).cyan());

    // Check directory exists
    if !path.exists() {
        println!("{} Directory does not exist", style("x").red());
        anyhow::bail!("Template directory '{}' does not exist", template_path);
    }

    if !path.is_dir() {
        println!("{} Path is not a directory", style("x").red());
        anyhow::bail!("'{}' is not a directory", template_path);
    }

    // Check template.toml exists
    let config_path = path.join("template.toml");
    if !config_path.exists() {
        println!("{} template.toml not found", style("x").red());
        anyhow::bail!("template.toml not found in '{}'", template_path);
    }
    println!("{} template.toml found", style("ok").green());

    // Load and validate config
    let config = match load_template_config(&config_path) {
        Ok(c) => {
            println!("{} template.toml is valid TOML", style("ok").green());
            c
        }
        Err(e) => {
            println!("{} template.toml parse error: {}", style("x").red(), e);
            anyhow::bail!("Failed to parse template.toml: {}", e);
        }
    };

    // Check all variables have descriptions
    let mut all_have_descriptions = true;
    for var in &config.variables {
        if var.description.is_empty() {
            println!(
                "{} Variable '{}' has no description",
                style("!").yellow(),
                var.name
            );
            all_have_descriptions = false;
        }
    }
    if all_have_descriptions && !config.variables.is_empty() {
        println!("{} All variables have descriptions", style("ok").green());
    }

    // Check choice variables have choices defined
    for var in &config.variables {
        if var.var_type.as_deref() == Some("choice") && var.choices.is_empty() {
            println!(
                "{} Variable '{}' is type 'choice' but has no choices",
                style("x").red(),
                var.name
            );
        }
    }

    // Collect defined variable names
    let defined_vars: HashSet<String> = config.variables.iter().map(|v| v.name.clone()).collect();

    // Find variables used in template files
    let used_vars = find_variables_in_files(&path)?;

    // Check for undefined variables (used but not defined)
    let undefined: Vec<_> = used_vars.difference(&defined_vars).collect();
    if !undefined.is_empty() {
        for var in &undefined {
            println!(
                "{} Variable '{{{{{}}}}}' used in files but not defined",
                style("!").yellow(),
                var
            );
        }
    }

    // Check for unused variables (defined but not used)
    let unused: Vec<_> = defined_vars.difference(&used_vars).collect();
    if !unused.is_empty() {
        for var in &unused {
            println!(
                "{} Variable '{}' defined but never used",
                style("!").yellow(),
                var
            );
        }
    }

    if undefined.is_empty() && unused.is_empty() && !defined_vars.is_empty() {
        println!("{} All variables are defined and used", style("ok").green());
    }

    println!("\nTemplate: {}", style(&config.name).cyan().bold());
    println!("Description: {}", config.description);
    println!("Variables: {}", config.variables.len());
    println!("Conditionals: {}", config.conditionals.len());

    Ok(())
}

/// Find all {{variable}} patterns in template files
fn find_variables_in_files(dir: &Path) -> Result<HashSet<String>> {
    let mut vars = HashSet::new();
    let re = Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap();

    find_variables_recursive(dir, &re, &mut vars)?;

    Ok(vars)
}

fn find_variables_recursive(dir: &Path, re: &Regex, vars: &mut HashSet<String>) -> Result<()> {
    if dir.is_file() {
        // Skip template.toml itself
        if dir.file_name() == Some(std::ffi::OsStr::new("template.toml")) {
            return Ok(());
        }

        // Check filename for variables
        if let Some(name) = dir.file_name() {
            let name_str = name.to_string_lossy();
            for cap in re.captures_iter(&name_str) {
                vars.insert(cap[1].to_string());
            }
        }

        // Check file contents (only text files)
        if let Ok(content) = fs::read_to_string(dir) {
            for cap in re.captures_iter(&content) {
                vars.insert(cap[1].to_string());
            }
        }
    } else if dir.is_dir() {
        // Check directory name for variables
        if let Some(name) = dir.file_name() {
            let name_str = name.to_string_lossy();
            for cap in re.captures_iter(&name_str) {
                vars.insert(cap[1].to_string());
            }
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            find_variables_recursive(&entry.path(), re, vars)?;
        }
    }

    Ok(())
}
