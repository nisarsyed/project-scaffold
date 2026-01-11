mod bundled;
mod commands;
mod copy;
mod git;
mod global_config;
mod hooks;
mod template;
mod variables;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use commands::{
    ConfigAction, add_template, create_project_interactive, handle_config_command, list_templates,
    remove_template, show_template_info, validate_template,
};

#[derive(Parser)]
#[command(name = "scaffold")]
#[command(version, about = "A simple project template scaffolder", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available templates
    List,
    /// Create a new project from a template
    Create {
        /// Name of the template to use (interactive if not provided)
        template: Option<String>,
        /// Directory where the project will be created
        #[arg(short, long)]
        output: Option<String>,
        /// Template variable in key=value format (e.g., -v name=myapp -v author="John Doe")
        #[arg(short, long, value_parser = parse_key_val)]
        vars: Vec<(String, String)>,
        /// Skip prompts and use defaults for missing values
        #[arg(short, long)]
        yes: bool,
        /// Preview what would be created without creating files
        #[arg(long)]
        dry_run: bool,
    },
    /// Add a new template from local path or git URL
    Add {
        /// Local path or git URL (use #path for subdirectory, e.g., https://github.com/org/repo.git#templates/api)
        path: String,
        /// Name to register the template under
        name: String,
    },
    /// Show detailed information about a template
    Info {
        /// Name of the template
        template: String,
    },
    /// Remove a template
    Remove {
        /// Name of the template to remove
        template: String,
    },
    /// Validate a template's structure and variables
    Validate {
        /// Path to the template directory
        path: String,
    },
    /// Manage global configuration (saved defaults for variables)
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

fn parse_key_val(s: &str) -> Result<(String, String)> {
    let (key, value) = s.split_once('=').ok_or_else(|| {
        anyhow::anyhow!(
            "Invalid key=value pair: '{}'. Expected format: key=value",
            s
        )
    })?;
    Ok((key.to_string(), value.to_string()))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let templates_dir = PathBuf::from(".templates");

    match cli.command {
        Commands::List => list_templates(&templates_dir),
        Commands::Create {
            template,
            output,
            vars,
            yes,
            dry_run,
        } => create_project_interactive(&templates_dir, template, output, vars, yes, dry_run),
        Commands::Add { path, name } => add_template(&templates_dir, &path, &name),
        Commands::Info { template } => show_template_info(&templates_dir, &template),
        Commands::Remove { template } => remove_template(&templates_dir, &template),
        Commands::Validate { path } => validate_template(&path),
        Commands::Config { action } => handle_config_command(action),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_val_valid() {
        let result = parse_key_val("key=value").unwrap();
        assert_eq!(result, ("key".to_string(), "value".to_string()));
    }

    #[test]
    fn test_parse_key_val_with_equals_in_value() {
        let result = parse_key_val("key=val=ue").unwrap();
        assert_eq!(result, ("key".to_string(), "val=ue".to_string()));
    }

    #[test]
    fn test_parse_key_val_empty_value() {
        let result = parse_key_val("key=").unwrap();
        assert_eq!(result, ("key".to_string(), "".to_string()));
    }

    #[test]
    fn test_parse_key_val_invalid() {
        assert!(parse_key_val("invalid").is_err());
    }
}
