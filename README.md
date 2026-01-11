# Project Scaffold

[![CI](https://github.com/nisarsyed/project-scaffold/actions/workflows/ci.yml/badge.svg)](https://github.com/nisarsyed/project-scaffold/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/project-scaffold.svg)](https://crates.io/crates/project-scaffold)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast CLI tool for scaffolding new projects from customizable templates. Written in Rust.

## Features

- **Batteries Included** - Bundled `fastapi` and `nextjs` templates work immediately after install
- **Interactive & Scriptable** - Fuzzy search template selection or use flags for CI/CD
- **Variable Substitution** - `{{variable}}` placeholders in file contents and paths
- **Multiple Variable Types** - String, choice (selection menu), and boolean (yes/no)
- **Conditional Files** - Include/exclude files based on variable values
- **Post-Create Hooks** - Run setup commands after project creation
- **Git Integration** - Add templates directly from any git repository
- **Global Defaults** - Save common values (author, email) for reuse
- **Cross-Platform** - Works on macOS, Linux, and Windows
- **Fast** - Written in Rust with minimal dependencies

## Installation

```bash
# Quick install (macOS/Linux)
curl -fsSL https://raw.githubusercontent.com/nisarsyed/project-scaffold/main/install.sh | sh

# From crates.io
cargo install project-scaffold

# From source
git clone https://github.com/nisarsyed/project-scaffold.git && cd project-scaffold && cargo install --path .
```

<details>
<summary>Manual download</summary>

Download from [Releases](https://github.com/nisarsyed/project-scaffold/releases):

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `scaffold-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `scaffold-x86_64-apple-darwin.tar.gz` |
| Linux (x86_64) | `scaffold-x86_64-unknown-linux-gnu.tar.gz` |
| Windows | `scaffold-x86_64-pc-windows-msvc.zip` |

</details>

## Quick Start

```bash
# Install
cargo install project-scaffold

# Create a project (interactive mode) - works immediately!
scaffold create
```

Bundled templates (`fastapi`, `nextjs`) are available right after install - no setup needed:

```
Select a template:

  fastapi [bundled] - A Python REST API with FastAPI and Pydantic
  nextjs [bundled] - A Next.js 16 application with TypeScript and Tailwind CSS v4

Creating project from: Python FastAPI

? Output directory: ./my-api
? Project name (project_name): my-api
? Description (description): My awesome API
? Author name (author): John Doe

Running post-create hooks...
  Running: uv sync

Project created at: ./my-api

Next steps:

  cd ./my-api
```

## Commands

| Command | Description |
|---------|-------------|
| `scaffold list` | List all available templates |
| `scaffold create` | Create project (interactive) |
| `scaffold create <template> -o <dir> -y` | Create with defaults (scripting) |
| `scaffold create <template> -o <dir> --dry-run` | Preview without creating files |
| `scaffold info <template>` | Show template details and variables |
| `scaffold add <path> <name>` | Add template from local path |
| `scaffold add <git-url> <name>` | Add template from git repository |
| `scaffold remove <template>` | Remove a template |
| `scaffold validate <path>` | Validate template structure and variables |
| `scaffold config set <key> <value>` | Save a default value |
| `scaffold config get <key>` | Get a saved default |
| `scaffold config list` | List all saved defaults |
| `scaffold config unset <key>` | Remove a saved default |
| `scaffold config reset` | Reset all config to defaults |

### Adding Templates from Git

```bash
# Full git URL
scaffold add https://github.com/org/repo.git api

# With subdirectory (use # to specify path within repo)
scaffold add https://github.com/org/repo.git#templates/api api

# GitHub shorthand
scaffold add github:org/repo#templates/api api
```

### Global Defaults

Save values you use frequently to skip repetitive prompts:

```bash
# Set defaults for author info
scaffold config set author "John Doe"
scaffold config set author_email "john@example.com"

# List saved defaults
scaffold config list

# These are now used automatically when creating projects
scaffold create fastapi -o ./my-api
# Author will default to "John Doe" instead of prompting
```

Config is stored in platform-specific directories:
- **macOS**: `~/Library/Application Support/scaffold/config.toml`
- **Linux**: `~/.config/scaffold/config.toml`
- **Windows**: `%APPDATA%\scaffold\config.toml`

### Scripting / CI Usage

Use `-y` flag to skip prompts and use defaults:

```bash
scaffold create fastapi -o ./my-api -v project_name=my-api -y
```

## Templates

### Bundled Templates

These templates are embedded in the binary and available immediately after install:

| Template | Stack | Features |
|----------|-------|----------|
| `fastapi` | Python 3.12, FastAPI, Pydantic v2, uv | REST API, CRUD, async, OpenAPI docs |
| `nextjs` | Next.js 16, React 19, Tailwind v4, pnpm | App Router, TypeScript, modern CSS |

```bash
# Works right after install - no setup needed
scaffold list
# Output:
#   fastapi [bundled] - A Python REST API with FastAPI and Pydantic
#   nextjs [bundled] - A Next.js 16 application with TypeScript and Tailwind CSS v4

scaffold create fastapi -o my-api -y
```

Local templates (in `.templates/`) take precedence over bundled templates with the same name.

### Adding More Templates

Add templates from any git repository:

```bash
# From GitHub (shorthand)
scaffold add github:your-org/templates#python-api api

# Full git URL with subpath
scaffold add https://github.com/org/repo.git#templates/react react

# From local directory
scaffold add ./my-template my-template
```

### Example Templates

Additional templates available in the repo's `example-templates/` directory:

| Template | Stack |
|----------|-------|
| `python-cli` | Python, Typer, Rich, uv |
| `express-api` | Express.js 5, TypeScript, pnpm |
| `rust-cli` | Rust, Clap |
| `rust-lib` | Rust library crate |
| `go-cli` | Go, Cobra |
| `nodejs-basic` | Node.js, TypeScript, pnpm |

```bash
# Add directly from GitHub
scaffold add github:nisarsyed/project-scaffold#example-templates/python-cli cli
```

> Python templates use [**uv**](https://docs.astral.sh/uv/). JS/TS templates use **pnpm**.

## Creating Custom Templates

1. Create a directory with your project files
2. Add a `template.toml` configuration:

```toml
name = "My Template"
description = "What this template creates"

[[variables]]
name = "project_name"
description = "Name of the project"
default = "my-project"

[[variables]]
name = "author"
description = "Author name"

# Choice variable - presents a selection menu
[[variables]]
name = "license"
description = "Choose a license"
type = "choice"
choices = ["MIT", "Apache-2.0", "GPL-3.0"]
default = "MIT"

# Boolean variable - yes/no prompt
[[variables]]
name = "include_docker"
description = "Add Docker support?"
type = "bool"
default = "false"

# Conditional file inclusion
[[conditionals]]
include = "Dockerfile"
when = "include_docker == true"

[[conditionals]]
exclude = "src/cli.rs"
when = "project_type == 'lib'"

# Post-create hooks
[hooks]
post_create = [
    "git init",
    "echo 'Project ready!'"
]
```

3. Use `{{variable_name}}` placeholders in any file (contents and filenames)
4. Validate your template: `scaffold validate ./my-template`
5. Register it: `scaffold add ./my-template my-template`

### Variable Types

| Type | Description | Example |
|------|-------------|---------|
| `string` | Free text input (default) | `name = "project_name"` |
| `choice` | Selection from predefined options | `type = "choice"` with `choices = [...]` |
| `bool` | Yes/no confirmation | `type = "bool"` |

### Conditional Files

Include or exclude files based on variable values:

```toml
# Include Dockerfile only if include_docker is true
[[conditionals]]
include = "Dockerfile"
when = "include_docker == true"

# Exclude CLI module for library projects
[[conditionals]]
exclude = "src/cli.rs"
when = "project_type == 'lib'"
```

### Hooks (Post-Create Scripts)

Run commands after project creation:

```toml
[hooks]
post_create = [
    "uv sync",
    "git init",
    "echo 'Project ready!'"
]
```

Hooks run in the output directory after all files are copied. On Unix, commands run via `sh -c`; on Windows, via `cmd /C`. Failed hooks show a warning but don't stop the process.

### Sharing Templates (Teams / Organizations)

Host your templates in a git repository and team members can add them directly:

```bash
# Team members run:
scaffold add github:your-org/templates#python-api api
scaffold add github:your-org/templates#react-app frontend
```

No need to clone the entire repo - just reference the template path.

### Template Storage

Templates are stored in `.templates/` in your current directory:

```
.templates/
└── my-template/
    ├── template.toml    # Required
    ├── src/
    └── ...
```

## Variable Substitution

Variables use `{{name}}` or `{{ name }}` syntax and work in both file contents and paths.

```toml
# Input
[package]
name = "{{project_name}}"

# With: -v project_name=myapp
# Output
[package]
name = "myapp"
```

Binary files are automatically detected and copied without substitution.

## Examples & Workflows

See [docs/examples.md](docs/examples.md) for detailed usage scenarios:

- **Freelance Developer Workflow** - Save hours on repetitive project setup
- **Team Standardization** - Consistent project structure across your organization
- **Microservices Architecture** - Spin up new services with conditional configs
- **Building Your Template Library** - Grow from bundled templates to a personalized collection
- **CI/CD Integration** - Automate project creation in pipelines

## Development

**Requirements**: Rust 1.85 or later

```bash
# Clone and build
git clone https://github.com/nisarsyed/project-scaffold.git
cd project-scaffold
cargo build

# Run tests (42 tests: 31 unit + 11 integration)
cargo test

# Run lints
cargo clippy -- -D warnings
cargo fmt --check

# Build release
cargo build --release
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## License

MIT - see [LICENSE](LICENSE)
