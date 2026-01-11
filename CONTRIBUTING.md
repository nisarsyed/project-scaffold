# Contributing to Project Scaffold

Thank you for your interest in contributing! This guide will help you get started.

## Requirements

- **Rust 1.85 or later** (check with `rustc --version`)

## Development Setup

```bash
# Clone the repository
git clone https://github.com/nisarsyed/project-scaffold.git
cd project-scaffold

# Build
cargo build

# Run tests (42 tests: 31 unit + 11 integration)
cargo test

# Run lints
cargo clippy -- -D warnings
cargo fmt --check
```

## Project Structure

```
src/
├── main.rs           # CLI entry point, command dispatch
├── commands/         # Command implementations
│   ├── mod.rs
│   ├── create.rs     # scaffold create
│   ├── add.rs        # scaffold add
│   ├── list.rs       # scaffold list
│   ├── info.rs       # scaffold info
│   ├── remove.rs     # scaffold remove
│   ├── validate.rs   # scaffold validate
│   └── config.rs     # scaffold config
├── template/         # Template loading and configuration
│   ├── mod.rs
│   ├── config.rs     # TemplateConfig, Variable, Conditional structs
│   └── loader.rs     # Template discovery and loading
├── copy.rs           # File copying with variable substitution
├── variables.rs      # Variable substitution and conditional evaluation
├── git.rs            # Git URL parsing and repository cloning
├── hooks.rs          # Post-create hook execution
└── global_config.rs  # Global configuration management

tests/
└── integration.rs    # End-to-end integration tests

example-templates/    # Example templates for testing
```

## Making Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run tests and lints
5. Commit your changes (see commit format below)
6. Push to your fork
7. Open a Pull Request

## Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/). Each commit message should be structured as:

```
<type>: <short description>

[optional body]

[optional footer]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Code style (formatting, no logic change) |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `chore` | Maintenance (deps, CI, build, etc.) |

### Examples

```bash
feat: add support for git subpaths in template URLs
fix: handle spaces in output directory paths
docs: add examples for team workflows
refactor: simplify variable substitution with regex
test: add integration tests for conditional files
chore: update dependencies
```

### Breaking Changes

For breaking changes, add `!` after the type:

```bash
feat!: change default template directory from .templates to templates
```

## Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality
- Use `.context()` or `.with_context()` from anyhow for error context
- Prefer `if cfg!(target_os = "windows")` for platform-specific code

## Testing

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

### Adding Tests

- **Unit tests**: Add `#[cfg(test)]` module in the relevant source file
- **Integration tests**: Add to `tests/integration.rs` for end-to-end scenarios

## Adding Templates

If you'd like to contribute a new template:

1. Create a directory in `example-templates/`
2. Add a `template.toml` with:
   ```toml
   name = "My Template"
   description = "What this template creates"

   [[variables]]
   name = "project_name"
   description = "Name of the project"
   default = "my-project"

   # Optional: choice variables
   [[variables]]
   name = "license"
   description = "Choose a license"
   type = "choice"
   choices = ["MIT", "Apache-2.0", "GPL-3.0"]

   # Optional: boolean variables
   [[variables]]
   name = "include_docker"
   description = "Add Docker support?"
   type = "bool"
   default = "false"

   # Optional: conditional files
   [[conditionals]]
   include = "Dockerfile"
   when = "include_docker == true"

   # Optional: post-create hooks
   [hooks]
   post_create = ["git init"]
   ```
3. Use `{{variable_name}}` placeholders in files and paths
4. Test with:
   ```bash
   scaffold validate example-templates/your-template
   scaffold add example-templates/your-template test
   scaffold create test -o /tmp/test --dry-run
   scaffold create test -o /tmp/test -y
   ```

## Reporting Issues

When reporting issues, please include:

- Version (`scaffold --version`)
- Operating system
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior

## Pull Request Guidelines

- Keep PRs focused on a single change
- Update documentation if needed
- Add tests for new features
- Ensure CI passes
- Update CHANGELOG.md for user-facing changes

## Architecture Notes

### Key Patterns

- **Error handling**: Use `anyhow::Result` with `.context()` for all fallible operations
- **CLI parsing**: Clap derive macros in `main.rs`
- **Configuration**: TOML parsing with serde in `template/config.rs`
- **Cross-platform**: Platform checks for hooks (`cfg!(target_os = "windows")`)

### Adding a New Command

1. Add the command variant to the `Commands` enum in `main.rs`
2. Create a new file in `src/commands/`
3. Implement the command function
4. Export from `src/commands/mod.rs`
5. Add the match arm in `main()`
6. Add tests

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
