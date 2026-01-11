use anyhow::{Context, Result};
use console::style;
use indicatif::ProgressBar;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::variables::substitute_variables;

/// Count total files in template (excluding template.toml and conditionally excluded files)
pub fn count_files(path: &Path, excluded: &HashSet<String>) -> Result<usize> {
    let mut count = 0;

    if path.is_file() {
        if path.file_name() != Some(std::ffi::OsStr::new("template.toml")) {
            count += 1;
        }
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if excluded.contains(&file_name) {
                continue;
            }

            count += count_files(&entry_path, excluded)?;
        }
    }

    Ok(count)
}

/// Preview what files would be created (for --dry-run)
pub fn preview_template(
    src: &Path,
    dst: &Path,
    variables: &HashMap<String, String>,
    excluded: &HashSet<String>,
    depth: usize,
) -> Result<()> {
    let file_name = src.file_name().map(|n| n.to_string_lossy().to_string());

    // Skip template.toml and excluded files
    if file_name.as_deref() == Some("template.toml") {
        return Ok(());
    }
    if let Some(ref name) = file_name {
        if excluded.contains(name) {
            return Ok(());
        }
    }

    let indent = "  ".repeat(depth);
    let dst_name = dst
        .file_name()
        .map(|n| substitute_variables(&n.to_string_lossy(), variables))
        .unwrap_or_else(|| dst.to_string_lossy().to_string());

    if src.is_dir() {
        if depth == 0 {
            println!("  {}/", style(&dst_name).cyan().bold());
        } else {
            println!("  {}{}/", indent, style(&dst_name).cyan());
        }

        let mut entries: Vec<_> = fs::read_dir(src)?.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let src_path = entry.path();
            let entry_name = src_path.file_name().unwrap().to_string_lossy();
            let dst_file_name = substitute_variables(&entry_name, variables);
            let dst_path = dst.with_file_name(&dst_name).join(&dst_file_name);

            preview_template(&src_path, &dst_path, variables, excluded, depth + 1)?;
        }
    } else {
        println!("  {}{}", indent, dst_name);
    }

    Ok(())
}

pub fn copy_template_recursive(
    src: &Path,
    dst: &Path,
    variables: &HashMap<String, String>,
    excluded: &HashSet<String>,
    skip_template_toml: bool,
    progress: Option<&ProgressBar>,
) -> Result<()> {
    let file_name = src.file_name().map(|n| n.to_string_lossy().to_string());

    // Skip template.toml and excluded files
    if skip_template_toml && file_name.as_deref() == Some("template.toml") {
        return Ok(());
    }
    if let Some(ref name) = file_name {
        if excluded.contains(name) {
            return Ok(());
        }
    }

    if src.is_dir() {
        let dst_name = dst
            .file_name()
            .map(|n| substitute_variables(&n.to_string_lossy(), variables))
            .unwrap_or_else(|| dst.to_string_lossy().to_string());
        let dst_with_substitution = dst
            .parent()
            .map_or_else(|| PathBuf::from(&dst_name), |parent| parent.join(&dst_name));

        fs::create_dir_all(&dst_with_substitution).with_context(|| {
            format!(
                "Failed to create directory '{}'. Check that you have write permissions.",
                dst_with_substitution.display()
            )
        })?;

        for entry in fs::read_dir(src).with_context(|| {
            format!(
                "Failed to read template directory '{}'. Check that it exists and is readable.",
                src.display()
            )
        })? {
            let entry = entry?;
            let src_path = entry.path();
            let entry_file_name = src_path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid file path: {}", src_path.display()))?
                .to_string_lossy();
            let dst_file_name = substitute_variables(&entry_file_name, variables);
            let dst_path = dst_with_substitution.join(&dst_file_name);

            copy_template_recursive(
                &src_path,
                &dst_path,
                variables,
                excluded,
                skip_template_toml,
                progress,
            )?;
        }
    } else {
        // Check if it's a binary file by trying to read as text
        let content = fs::read(src).with_context(|| {
            format!(
                "Failed to read template file '{}'. Check that it exists and is readable.",
                src.display()
            )
        })?;

        // Try to interpret as UTF-8, if it fails, copy as binary
        match String::from_utf8(content) {
            Ok(text_content) => {
                let rendered_content = substitute_variables(&text_content, variables);
                fs::write(dst, &rendered_content).with_context(|| {
                    format!(
                        "Failed to write file '{}'. Check that you have write permissions.",
                        dst.display()
                    )
                })?;
            }
            Err(e) => {
                // Binary file, copy as-is (get bytes back from error)
                fs::write(dst, e.into_bytes()).with_context(|| {
                    format!(
                        "Failed to write file '{}'. Check that you have write permissions.",
                        dst.display()
                    )
                })?;
            }
        }

        if let Some(pb) = progress {
            pb.inc(1);
        }
    }

    Ok(())
}
