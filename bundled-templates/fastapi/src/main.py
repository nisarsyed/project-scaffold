"""FastAPI application."""

from collections.abc import AsyncIterator
from contextlib import asynccontextmanager

from fastapi import FastAPI

from .routes import health, items


@asynccontextmanager
async def lifespan(app: FastAPI) -> AsyncIterator[None]:
    """Application lifespan context manager.

    Handles startup and shutdown events.
    """
    # Startup: Initialize resources (database connections, caches, etc.)
    yield
    # Shutdown: Clean up resources


app = FastAPI(
    title="{{project_name}}",
    description="{{description}}",
    version="0.1.0",
    lifespan=lifespan,
)

app.include_router(health.router)
app.include_router(items.router, prefix="/api/v1")
