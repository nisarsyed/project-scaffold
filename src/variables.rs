use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use crate::template::Conditional;

/// Regex for matching {{variable}} or {{ variable }} patterns
static VAR_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap());

/// Regex for condition evaluation: var == value, var == 'value', var == "value"
static CONDITION_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*(\w+)\s*==\s*(?:["']([^"']+)["']|(true|false|\w+))\s*$"#).unwrap()
});

/// Substitute {{variable}} patterns in a string
pub fn substitute_variables(template: &str, variables: &HashMap<String, String>) -> String {
    VAR_PATTERN
        .replace_all(template, |caps: &Captures| {
            variables
                .get(&caps[1])
                .cloned()
                .unwrap_or_else(|| caps[0].to_string())
        })
        .to_string()
}

/// Evaluate conditionals and return set of files/dirs to exclude
pub fn evaluate_conditionals(
    conditionals: &[Conditional],
    variables: &HashMap<String, String>,
) -> HashSet<String> {
    let mut excluded = HashSet::new();

    for cond in conditionals {
        let condition_met = evaluate_condition(&cond.when, variables);

        if let Some(ref include_path) = cond.include {
            // If condition is NOT met, exclude this file
            if !condition_met {
                excluded.insert(include_path.clone());
            }
        }

        if let Some(ref exclude_path) = cond.exclude {
            // If condition IS met, exclude this file
            if condition_met {
                excluded.insert(exclude_path.clone());
            }
        }
    }

    excluded
}

/// Evaluate a simple condition like "var == true" or "var == 'value'"
pub fn evaluate_condition(condition: &str, variables: &HashMap<String, String>) -> bool {
    if let Some(caps) = CONDITION_PATTERN.captures(condition) {
        let var_name = &caps[1];
        let expected_value = caps
            .get(2)
            .or(caps.get(3))
            .map(|m| m.as_str())
            .unwrap_or("");

        if let Some(actual_value) = variables.get(var_name) {
            return actual_value == expected_value;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitute_variables_basic() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "test".to_string());
        let result = substitute_variables("Hello {{name}}", &vars);
        assert_eq!(result, "Hello test");
    }

    #[test]
    fn test_substitute_variables_spaced() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "test".to_string());
        let result = substitute_variables("Hello {{ name }}", &vars);
        assert_eq!(result, "Hello test");
    }

    #[test]
    fn test_substitute_variables_multiple() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "myapp".to_string());
        vars.insert("author".to_string(), "John".to_string());
        let result = substitute_variables("{{name}} by {{author}}", &vars);
        assert_eq!(result, "myapp by John");
    }

    #[test]
    fn test_substitute_variables_no_match() {
        let vars = HashMap::new();
        let result = substitute_variables("Hello {{name}}", &vars);
        assert_eq!(result, "Hello {{name}}");
    }

    #[test]
    fn test_evaluate_condition_bool_true() {
        let mut vars = HashMap::new();
        vars.insert("include_docker".to_string(), "true".to_string());
        assert!(evaluate_condition("include_docker == true", &vars));
    }

    #[test]
    fn test_evaluate_condition_bool_false() {
        let mut vars = HashMap::new();
        vars.insert("include_docker".to_string(), "false".to_string());
        assert!(evaluate_condition("include_docker == false", &vars));
        assert!(!evaluate_condition("include_docker == true", &vars));
    }

    #[test]
    fn test_evaluate_condition_string_value() {
        let mut vars = HashMap::new();
        vars.insert("license".to_string(), "MIT".to_string());
        assert!(evaluate_condition("license == 'MIT'", &vars));
        assert!(evaluate_condition("license == \"MIT\"", &vars));
        assert!(!evaluate_condition("license == 'Apache'", &vars));
    }

    #[test]
    fn test_evaluate_condition_missing_var() {
        let vars = HashMap::new();
        assert!(!evaluate_condition("missing == true", &vars));
    }

    #[test]
    fn test_evaluate_conditionals_include() {
        let conditionals = vec![Conditional {
            include: Some("Dockerfile".to_string()),
            exclude: None,
            when: "include_docker == true".to_string(),
        }];

        let mut vars = HashMap::new();
        vars.insert("include_docker".to_string(), "true".to_string());
        let excluded = evaluate_conditionals(&conditionals, &vars);
        assert!(excluded.is_empty()); // Dockerfile included

        vars.insert("include_docker".to_string(), "false".to_string());
        let excluded = evaluate_conditionals(&conditionals, &vars);
        assert!(excluded.contains("Dockerfile")); // Dockerfile excluded
    }

    #[test]
    fn test_evaluate_conditionals_exclude() {
        let conditionals = vec![Conditional {
            include: None,
            exclude: Some("cli.rs".to_string()),
            when: "project_type == 'lib'".to_string(),
        }];

        let mut vars = HashMap::new();
        vars.insert("project_type".to_string(), "lib".to_string());
        let excluded = evaluate_conditionals(&conditionals, &vars);
        assert!(excluded.contains("cli.rs")); // cli.rs excluded

        vars.insert("project_type".to_string(), "bin".to_string());
        let excluded = evaluate_conditionals(&conditionals, &vars);
        assert!(excluded.is_empty()); // cli.rs included
    }
}
