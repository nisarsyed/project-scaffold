use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Parsed remote URL with optional subpath
pub struct RemoteUrl {
    pub repo_url: String,
    pub subpath: Option<String>,
}

/// Check if a path looks like a git URL
pub fn is_git_url(path: &str) -> bool {
    path.starts_with("https://")
        || path.starts_with("http://")
        || path.starts_with("git@")
        || path.starts_with("git://")
        || path.starts_with("github:")
}

/// Parse a git URL, extracting the repo URL and optional subpath
/// Supports: https://github.com/org/repo.git#path/to/template
///           github:org/repo#path/to/template
pub fn parse_git_url(url: &str) -> RemoteUrl {
    // Handle github: shorthand
    let (url, is_github_shorthand) = if let Some(rest) = url.strip_prefix("github:") {
        (rest.to_string(), true)
    } else {
        (url.to_string(), false)
    };

    // Split on # to get subpath
    let (repo_part, subpath) = if let Some((repo, path)) = url.split_once('#') {
        (repo.to_string(), Some(path.to_string()))
    } else {
        (url, None)
    };

    // Convert github shorthand to full URL
    let repo_url = if is_github_shorthand {
        format!("https://github.com/{}.git", repo_part)
    } else {
        repo_part
    };

    RemoteUrl { repo_url, subpath }
}

/// Clone a git repository to a temporary directory
pub fn clone_repo(url: &str) -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir().join(format!("scaffold-{}", std::process::id()));

    // Clean up if exists from previous failed run
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).context("Failed to clean up temp directory")?;
    }

    println!("Cloning {}...", style(url).dim());

    let temp_path = temp_dir
        .to_str()
        .context("Temp directory path contains invalid UTF-8")?;

    let output = Command::new("git")
        .args(["clone", "--depth", "1", url, temp_path])
        .output()
        .context("Failed to execute git clone. Is git installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git clone failed: {}", stderr.trim());
    }

    Ok(temp_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_url() {
        assert!(is_git_url("https://github.com/org/repo.git"));
        assert!(is_git_url("http://github.com/org/repo.git"));
        assert!(is_git_url("git@github.com:org/repo.git"));
        assert!(is_git_url("git://github.com/org/repo.git"));
        assert!(is_git_url("github:org/repo"));
        assert!(!is_git_url("./local/path"));
        assert!(!is_git_url("/absolute/path"));
        assert!(!is_git_url("relative/path"));
    }

    #[test]
    fn test_parse_git_url_https() {
        let parsed = parse_git_url("https://github.com/org/repo.git");
        assert_eq!(parsed.repo_url, "https://github.com/org/repo.git");
        assert!(parsed.subpath.is_none());
    }

    #[test]
    fn test_parse_git_url_with_subpath() {
        let parsed = parse_git_url("https://github.com/org/repo.git#templates/api");
        assert_eq!(parsed.repo_url, "https://github.com/org/repo.git");
        assert_eq!(parsed.subpath, Some("templates/api".to_string()));
    }

    #[test]
    fn test_parse_git_url_github_shorthand() {
        let parsed = parse_git_url("github:org/repo");
        assert_eq!(parsed.repo_url, "https://github.com/org/repo.git");
        assert!(parsed.subpath.is_none());
    }

    #[test]
    fn test_parse_git_url_github_shorthand_with_subpath() {
        let parsed = parse_git_url("github:org/repo#templates/api");
        assert_eq!(parsed.repo_url, "https://github.com/org/repo.git");
        assert_eq!(parsed.subpath, Some("templates/api".to_string()));
    }
}
