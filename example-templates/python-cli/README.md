# {{project_name}}

{{description}}

## Requirements

- Python 3.12+
- [uv](https://docs.astral.sh/uv/) (recommended) or pip

## Installation

```bash
# Using uv (recommended)
uv sync

# Or using pip
pip install -e .
```

## Usage

```bash
# Show help
uv run {{project_name}} --help

# Greet someone
uv run {{project_name}} hello
uv run {{project_name}} hello "Your Name"
uv run {{project_name}} hello "Your Name" --count 3
uv run {{project_name}} hello "Your Name" --loud

# Show system info
uv run {{project_name}} info

# Show version
uv run {{project_name}} version
```

## Development

```bash
# Install dependencies (includes dev)
uv sync

# Run tests
uv run pytest

# Lint code
uv run ruff check .
uv run ruff format .

# Type checking
uv run mypy src
```

## Tech Stack

- [Python](https://www.python.org/) 3.12+
- [Typer](https://typer.tiangolo.com/) - CLI framework
- [Rich](https://rich.readthedocs.io/) - Terminal formatting
- [uv](https://docs.astral.sh/uv/) - Package manager
