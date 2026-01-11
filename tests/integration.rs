use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn scaffold_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_scaffold"))
}

#[test]
fn test_list_command() {
    let output = scaffold_cmd().arg("list").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available templates") || stdout.contains("No templates found"));
}

#[test]
fn test_info_command_nonexistent() {
    let output = scaffold_cmd()
        .args(["info", "nonexistent-template"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found"));
}

#[test]
fn test_validate_command_nonexistent() {
    let output = scaffold_cmd()
        .args(["validate", "/nonexistent/path"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does not exist"));
}

#[test]
fn test_validate_command_valid_template() {
    // Create a temp directory with a valid template
    let temp = TempDir::new().unwrap();
    let template_dir = temp.path().join("test-template");
    fs::create_dir(&template_dir).unwrap();

    // Create template.toml
    fs::write(
        template_dir.join("template.toml"),
        r#"
name = "Test Template"
description = "A test template"

[[variables]]
name = "project_name"
description = "Name of the project"
default = "my-project"
"#,
    )
    .unwrap();

    // Create a file that uses the variable
    fs::write(
        template_dir.join("README.md"),
        "# {{project_name}}\n\nA new project.\n",
    )
    .unwrap();

    let output = scaffold_cmd()
        .args(["validate", template_dir.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("template.toml found"));
    assert!(stdout.contains("template.toml is valid TOML"));
}

#[test]
fn test_create_dry_run() {
    let temp = TempDir::new().unwrap();
    let template_dir = temp.path().join("test-template");
    fs::create_dir(&template_dir).unwrap();

    // Create template.toml
    fs::write(
        template_dir.join("template.toml"),
        r#"
name = "Test Template"
description = "A test template"

[[variables]]
name = "project_name"
description = "Name of the project"
default = "my-project"
"#,
    )
    .unwrap();

    // Create a file
    fs::create_dir(template_dir.join("src")).unwrap();
    fs::write(template_dir.join("src/main.rs"), "fn main() {}").unwrap();

    // Create .templates directory and add the template
    let templates_dir = temp.path().join(".templates");
    fs::create_dir(&templates_dir).unwrap();
    let dst_template = templates_dir.join("test");
    copy_dir_all(&template_dir, &dst_template).unwrap();

    let output_dir = temp.path().join("output");

    let output = scaffold_cmd()
        .current_dir(temp.path())
        .args([
            "create",
            "test",
            "-o",
            output_dir.to_str().unwrap(),
            "--dry-run",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dry run"));
    assert!(stdout.contains("Would create"));
    // Output directory should not exist
    assert!(!output_dir.exists());
}

#[test]
fn test_create_project() {
    let temp = TempDir::new().unwrap();
    let template_dir = temp.path().join("test-template");
    fs::create_dir(&template_dir).unwrap();

    // Create template.toml
    fs::write(
        template_dir.join("template.toml"),
        r#"
name = "Test Template"
description = "A test template"

[[variables]]
name = "project_name"
description = "Name of the project"
default = "my-project"
"#,
    )
    .unwrap();

    // Create files that use the variable
    fs::write(
        template_dir.join("README.md"),
        "# {{project_name}}\n\nA new project.\n",
    )
    .unwrap();
    fs::create_dir(template_dir.join("src")).unwrap();
    fs::write(
        template_dir.join("src/main.rs"),
        "// Project: {{project_name}}\nfn main() {}",
    )
    .unwrap();

    // Create .templates directory and add the template
    let templates_dir = temp.path().join(".templates");
    fs::create_dir(&templates_dir).unwrap();
    let dst_template = templates_dir.join("test");
    copy_dir_all(&template_dir, &dst_template).unwrap();

    let output_dir = temp.path().join("output");

    let output = scaffold_cmd()
        .current_dir(temp.path())
        .args([
            "create",
            "test",
            "-o",
            output_dir.to_str().unwrap(),
            "-v",
            "project_name=awesome-app",
            "-y",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());

    // Check output directory exists
    assert!(output_dir.exists());

    // Check README was created with substitution
    let readme = fs::read_to_string(output_dir.join("README.md")).unwrap();
    assert!(readme.contains("# awesome-app"));
    assert!(!readme.contains("{{project_name}}"));

    // Check src/main.rs was created with substitution
    let main_rs = fs::read_to_string(output_dir.join("src/main.rs")).unwrap();
    assert!(main_rs.contains("// Project: awesome-app"));
}

#[test]
fn test_add_and_remove_template() {
    let temp = TempDir::new().unwrap();

    // Create a source template
    let src_template = temp.path().join("src-template");
    fs::create_dir(&src_template).unwrap();
    fs::write(
        src_template.join("template.toml"),
        r#"
name = "Source Template"
description = "A source template"
"#,
    )
    .unwrap();

    // Create .templates directory
    let templates_dir = temp.path().join(".templates");
    fs::create_dir(&templates_dir).unwrap();

    // Add template
    let output = scaffold_cmd()
        .current_dir(temp.path())
        .args(["add", src_template.to_str().unwrap(), "new-template"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("added successfully"));

    // Template should exist
    assert!(templates_dir.join("new-template/template.toml").exists());

    // Remove template
    let output = scaffold_cmd()
        .current_dir(temp.path())
        .args(["remove", "new-template"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("removed"));

    // Template should not exist
    assert!(!templates_dir.join("new-template").exists());
}

#[test]
fn test_config_commands() {
    // Config commands write to system config, so we just test the output format
    let output = scaffold_cmd().args(["config", "list"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should either show defaults or say no defaults
    assert!(stdout.contains("defaults") || stdout.contains("No defaults"));
}

#[test]
fn test_conditional_file_exclusion() {
    let temp = TempDir::new().unwrap();
    let template_dir = temp.path().join("test-template");
    fs::create_dir(&template_dir).unwrap();

    // Create template.toml with conditional
    fs::write(
        template_dir.join("template.toml"),
        r#"
name = "Test Template"
description = "A test template"

[[variables]]
name = "project_name"
description = "Name of the project"
default = "my-project"

[[variables]]
name = "include_docker"
description = "Include Docker support"
type = "bool"
default = "false"

[[conditionals]]
include = "Dockerfile"
when = "include_docker == true"
"#,
    )
    .unwrap();

    // Create Dockerfile
    fs::write(template_dir.join("Dockerfile"), "FROM alpine:latest").unwrap();
    fs::write(template_dir.join("README.md"), "# {{project_name}}").unwrap();

    // Create .templates directory and add the template
    let templates_dir = temp.path().join(".templates");
    fs::create_dir(&templates_dir).unwrap();
    let dst_template = templates_dir.join("test");
    copy_dir_all(&template_dir, &dst_template).unwrap();

    let output_dir = temp.path().join("output");

    // Create with include_docker=false (default)
    let output = scaffold_cmd()
        .current_dir(temp.path())
        .args(["create", "test", "-o", output_dir.to_str().unwrap(), "-y"])
        .output()
        .unwrap();

    assert!(output.status.success());

    // Dockerfile should NOT exist (condition not met)
    assert!(!output_dir.join("Dockerfile").exists());
    // README should exist
    assert!(output_dir.join("README.md").exists());
}

#[test]
fn test_conditional_file_inclusion() {
    let temp = TempDir::new().unwrap();
    let template_dir = temp.path().join("test-template");
    fs::create_dir(&template_dir).unwrap();

    // Create template.toml with conditional
    fs::write(
        template_dir.join("template.toml"),
        r#"
name = "Test Template"
description = "A test template"

[[variables]]
name = "project_name"
description = "Name of the project"
default = "my-project"

[[variables]]
name = "include_docker"
description = "Include Docker support"
type = "bool"
default = "false"

[[conditionals]]
include = "Dockerfile"
when = "include_docker == true"
"#,
    )
    .unwrap();

    // Create Dockerfile
    fs::write(template_dir.join("Dockerfile"), "FROM alpine:latest").unwrap();
    fs::write(template_dir.join("README.md"), "# {{project_name}}").unwrap();

    // Create .templates directory and add the template
    let templates_dir = temp.path().join(".templates");
    fs::create_dir(&templates_dir).unwrap();
    let dst_template = templates_dir.join("test");
    copy_dir_all(&template_dir, &dst_template).unwrap();

    let output_dir = temp.path().join("output");

    // Create with include_docker=true
    let output = scaffold_cmd()
        .current_dir(temp.path())
        .args([
            "create",
            "test",
            "-o",
            output_dir.to_str().unwrap(),
            "-v",
            "include_docker=true",
            "-y",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());

    // Dockerfile SHOULD exist (condition met)
    assert!(output_dir.join("Dockerfile").exists());
    // README should exist
    assert!(output_dir.join("README.md").exists());
}

#[test]
fn test_variable_substitution_in_filename() {
    let temp = TempDir::new().unwrap();
    let template_dir = temp.path().join("test-template");
    fs::create_dir(&template_dir).unwrap();

    // Create template.toml
    fs::write(
        template_dir.join("template.toml"),
        r#"
name = "Test Template"
description = "A test template"

[[variables]]
name = "project_name"
description = "Name of the project"
default = "my-project"
"#,
    )
    .unwrap();

    // Create a directory with variable in name
    fs::create_dir(template_dir.join("{{project_name}}")).unwrap();
    fs::write(
        template_dir.join("{{project_name}}/main.py"),
        "# {{project_name}}",
    )
    .unwrap();

    // Create .templates directory and add the template
    let templates_dir = temp.path().join(".templates");
    fs::create_dir(&templates_dir).unwrap();
    let dst_template = templates_dir.join("test");
    copy_dir_all(&template_dir, &dst_template).unwrap();

    let output_dir = temp.path().join("output");

    let output = scaffold_cmd()
        .current_dir(temp.path())
        .args([
            "create",
            "test",
            "-o",
            output_dir.to_str().unwrap(),
            "-v",
            "project_name=my_app",
            "-y",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());

    // Directory with substituted name should exist
    assert!(output_dir.join("my_app").exists());
    assert!(output_dir.join("my_app/main.py").exists());

    // Check content substitution
    let content = fs::read_to_string(output_dir.join("my_app/main.py")).unwrap();
    assert!(content.contains("# my_app"));
}

/// Helper function to recursively copy a directory
fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
