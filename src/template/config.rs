use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Indicates where a template comes from
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateSource {
    /// User-added template in local .templates/ directory
    Local,
    /// Built-in template bundled with the binary
    Bundled,
}

/// Represents an available template with its metadata
pub struct AvailableTemplate {
    pub dir_name: String,
    pub config: TemplateConfig,
    pub source: TemplateSource,
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub variables: Vec<Variable>,
    #[serde(default)]
    pub conditionals: Vec<Conditional>,
    #[serde(default)]
    pub hooks: Option<HooksConfig>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HooksConfig {
    #[serde(default)]
    pub post_create: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub default: Option<String>,
    /// Variable type: "string" (default), "choice", or "bool"
    #[serde(rename = "type", default)]
    pub var_type: Option<String>,
    /// Choices for "choice" type variables
    #[serde(default)]
    pub choices: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conditional {
    /// File or directory to include (mutually exclusive with exclude)
    #[serde(default)]
    pub include: Option<String>,
    /// File or directory to exclude (mutually exclusive with include)
    #[serde(default)]
    pub exclude: Option<String>,
    /// Condition expression, e.g., "var_name == true" or "var_name == 'value'"
    pub when: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_template_with_hooks() {
        let content = r#"
name = "Test Template"
description = "Test description"

[[variables]]
name = "project_name"
description = "Project name"
default = "my-project"

[hooks]
post_create = ["npm install", "git init"]
"#;

        let config: TemplateConfig = toml::from_str(content).unwrap();
        assert_eq!(config.name, "Test Template");
        assert!(config.hooks.is_some());
        let hooks = config.hooks.unwrap();
        assert_eq!(hooks.post_create.len(), 2);
        assert_eq!(hooks.post_create[0], "npm install");
        assert_eq!(hooks.post_create[1], "git init");
    }

    #[test]
    fn test_parse_template_without_hooks() {
        let content = r#"
name = "Test Template"
description = "Test description"
"#;

        let config: TemplateConfig = toml::from_str(content).unwrap();
        assert!(config.hooks.is_none());
    }

    #[test]
    fn test_parse_nextjs_template() {
        let content = r#"name = "Next.js App"
description = "A Next.js 16 application with TypeScript and Tailwind CSS v4"

[[variables]]
name = "project_name"
description = "Name of the project (lowercase, hyphens allowed)"
default = "my-app"

[[variables]]
name = "description"
description = "Project description"
default = "A Next.js application"

[[variables]]
name = "author"
description = "Author name"
default = "Your Name"

[hooks]
post_create = [
    "pnpm install"
]
"#;

        let config: TemplateConfig = toml::from_str(content).unwrap();
        assert_eq!(config.name, "Next.js App");
        assert!(config.hooks.is_some(), "hooks should be Some");
        let hooks = config.hooks.unwrap();
        assert_eq!(hooks.post_create.len(), 1);
        assert_eq!(hooks.post_create[0], "pnpm install");
    }
}
