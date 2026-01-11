"""Item routes."""

from typing import Annotated

from fastapi import APIRouter, HTTPException, Path, Query

from ..models.item import Item, ItemCreate, ItemUpdate

router = APIRouter(tags=["items"])

# In-memory storage for demo
items_db: dict[int, Item] = {}
counter = 0


@router.get("/items", response_model=list[Item])
async def list_items(
    skip: Annotated[int, Query(ge=0, description="Number of items to skip")] = 0,
    limit: Annotated[int, Query(ge=1, le=100, description="Max items to return")] = 10,
) -> list[Item]:
    """List all items with pagination."""
    items = list(items_db.values())
    return items[skip : skip + limit]


@router.post("/items", response_model=Item, status_code=201)
async def create_item(item: ItemCreate) -> Item:
    """Create a new item."""
    global counter
    counter += 1
    new_item = Item(id=counter, **item.model_dump())
    items_db[counter] = new_item
    return new_item


@router.get("/items/{item_id}", response_model=Item)
async def get_item(
    item_id: Annotated[int, Path(gt=0, description="The ID of the item to get")],
) -> Item:
    """Get an item by ID."""
    if item_id not in items_db:
        raise HTTPException(status_code=404, detail="Item not found")
    return items_db[item_id]


@router.patch("/items/{item_id}", response_model=Item)
async def update_item(
    item_id: Annotated[int, Path(gt=0, description="The ID of the item to update")],
    item: ItemUpdate,
) -> Item:
    """Update an item by ID."""
    if item_id not in items_db:
        raise HTTPException(status_code=404, detail="Item not found")
    existing = items_db[item_id]
    update_data = item.model_dump(exclude_unset=True)
    updated = existing.model_copy(update=update_data)
    items_db[item_id] = updated
    return updated


@router.delete("/items/{item_id}", status_code=204)
async def delete_item(
    item_id: Annotated[int, Path(gt=0, description="The ID of the item to delete")],
) -> None:
    """Delete an item by ID."""
    if item_id not in items_db:
        raise HTTPException(status_code=404, detail="Item not found")
    del items_db[item_id]
