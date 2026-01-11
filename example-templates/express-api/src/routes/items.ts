import { Router } from "express";

export const itemsRouter = Router();

interface Item {
  id: number;
  name: string;
  description?: string;
  price: number;
  createdAt: string;
}

// In-memory storage for demo
const items: Map<number, Item> = new Map();
let counter = 0;

// List all items
itemsRouter.get("/", (_req, res) => {
  res.json(Array.from(items.values()));
});

// Create item
itemsRouter.post("/", (req, res) => {
  const { name, description, price } = req.body;

  if (!name || price === undefined) {
    res.status(400).json({ error: "Name and price are required" });
    return;
  }

  counter++;
  const item: Item = {
    id: counter,
    name,
    description,
    price,
    createdAt: new Date().toISOString(),
  };
  items.set(counter, item);

  res.status(201).json(item);
});

// Get item by ID
itemsRouter.get("/:id", (req, res) => {
  const id = parseInt(req.params.id);
  const item = items.get(id);

  if (!item) {
    res.status(404).json({ error: "Item not found" });
    return;
  }

  res.json(item);
});

// Update item
itemsRouter.put("/:id", (req, res) => {
  const id = parseInt(req.params.id);
  const item = items.get(id);

  if (!item) {
    res.status(404).json({ error: "Item not found" });
    return;
  }

  const { name, description, price } = req.body;
  const updatedItem: Item = {
    ...item,
    name: name ?? item.name,
    description: description ?? item.description,
    price: price ?? item.price,
  };
  items.set(id, updatedItem);

  res.json(updatedItem);
});

// Delete item
itemsRouter.delete("/:id", (req, res) => {
  const id = parseInt(req.params.id);

  if (!items.has(id)) {
    res.status(404).json({ error: "Item not found" });
    return;
  }

  items.delete(id);
  res.status(204).send();
});
