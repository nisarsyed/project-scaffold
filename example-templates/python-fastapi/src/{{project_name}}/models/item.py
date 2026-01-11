"""Item models."""

from pydantic import BaseModel, ConfigDict, Field


class ItemCreate(BaseModel):
    """Item creation schema."""

    model_config = ConfigDict(str_strip_whitespace=True)

    name: str = Field(min_length=1, max_length=100, description="Name of the item")
    description: str | None = Field(default=None, max_length=500, description="Item description")
    price: float = Field(gt=0, description="Price of the item (must be positive)")


class ItemUpdate(BaseModel):
    """Item update schema (all fields optional)."""

    model_config = ConfigDict(str_strip_whitespace=True)

    name: str | None = Field(default=None, min_length=1, max_length=100)
    description: str | None = Field(default=None, max_length=500)
    price: float | None = Field(default=None, gt=0)


class Item(ItemCreate):
    """Item schema with ID."""

    id: int = Field(description="Unique identifier for the item")
