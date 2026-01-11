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
# Run the development server
uv run uvicorn {{project_name}}.main:app --reload

# Or with FastAPI CLI
uv run fastapi dev src/{{project_name}}/main.py

# API docs available at:
# - Swagger UI: http://localhost:8000/docs
# - ReDoc: http://localhost:8000/redoc
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/api/v1/items` | List items (with pagination) |
| `POST` | `/api/v1/items` | Create item |
| `GET` | `/api/v1/items/{id}` | Get item by ID |
| `PATCH` | `/api/v1/items/{id}` | Update item by ID |
| `DELETE` | `/api/v1/items/{id}` | Delete item by ID |

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
- [FastAPI](https://fastapi.tiangolo.com/) - Web framework
- [Pydantic](https://docs.pydantic.dev/) v2 - Data validation
- [Uvicorn](https://www.uvicorn.org/) - ASGI server
- [uv](https://docs.astral.sh/uv/) - Package manager
