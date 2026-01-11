use anyhow::{Context, Result};
use console::style;
use std::path::Path;
use std::process::Command;

/// Execute shell commands as hooks
///
/// On Unix systems, commands are executed via `sh -c`.
/// On Windows, commands are executed via `cmd /C`.
pub fn execute_hooks(commands: &[String], working_dir: &Path) -> Result<()> {
    for cmd in commands {
        println!("  Running: {}", style(cmd).dim());

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", cmd])
                .current_dir(working_dir)
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .current_dir(working_dir)
                .output()
        }
        .with_context(|| format!("Failed to execute hook: {}", cmd))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!(
                "  {}: Hook failed: {}",
                style("Warning").yellow(),
                stderr.trim()
            );
        } else {
            println!("  {}", style("ok").green());
        }
    }
    Ok(())
}
