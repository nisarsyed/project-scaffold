# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-11

### Added
- `scaffold list` command to list available templates
- `scaffold create` command with interactive mode
- `scaffold create --dry-run` flag to preview what would be created
- `scaffold add` command to add templates from local paths or git URLs
- `scaffold info <template>` command to show template details and variables
- `scaffold remove <template>` command to remove templates
- `scaffold validate <path>` command to validate template structure
- `scaffold config` command for managing global defaults
  - `scaffold config set <key> <value>` - save default values for variables
  - `scaffold config get <key>` - retrieve a saved default
  - `scaffold config list` - list all saved defaults
  - `scaffold config unset <key>` - remove a saved default
  - `scaffold config reset` - reset all configuration
- Remote template support with git URLs
- GitHub shorthand syntax (`github:org/repo#path`)
- Subpath support for git URLs (`repo.git#templates/api`)
- Variable substitution with `{{variable}}` syntax
- Support for `template.toml` configuration
- Choice variables (`type = "choice"`) for selection menus
- Boolean variables (`type = "bool"`) for yes/no prompts
- Conditional file inclusion/exclusion based on variable values
- Post-create hooks (`[hooks] post_create = [...]`) for running scripts after project creation
- Progress bar during template creation
- Bundled `fastapi` and `nextjs` templates (available immediately after install, no setup needed)
- Python templates use [uv](https://docs.astral.sh/uv/) for package management
- Colored CLI output with fuzzy template selection
- `-y` flag for non-interactive mode
- `-v` flag for passing variables via CLI
- Warning messages for skipped templates with invalid configuration
- Improved error messages with actionable context
- Global config file stored in platform-specific config directory
- Cross-platform support (macOS, Linux, Windows)
- Binary file detection (copied without variable substitution)

### Technical

- Modular codebase architecture with separated concerns
- Minimum Supported Rust Version (MSRV): 1.85
- 42 tests (31 unit tests + 11 integration tests)
- CI/CD with GitHub Actions (build, test, clippy, fmt)
- Release binaries for macOS (Intel/Apple Silicon), Linux, and Windows

[0.1.0]: https://github.com/nisarsyed/project-scaffold/releases/tag/v0.1.0
