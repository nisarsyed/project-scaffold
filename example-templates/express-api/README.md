# {{project_name}}

{{description}}

## Installation

```bash
pnpm install
```

## Usage

```bash
# Development (with hot reload)
pnpm dev

# Production
pnpm build
pnpm start
```

Server runs on http://localhost:3000

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/api/v1/items` | List all items |
| POST | `/api/v1/items` | Create item |
| GET | `/api/v1/items/:id` | Get item by ID |
| PUT | `/api/v1/items/:id` | Update item |
| DELETE | `/api/v1/items/:id` | Delete item |

## Scripts

| Command | Description |
|---------|-------------|
| `pnpm dev` | Start dev server with hot reload |
| `pnpm build` | Build TypeScript to JavaScript |
| `pnpm start` | Run production server |
| `pnpm lint` | Run ESLint |

## Tech Stack

- [Express.js 5](https://expressjs.com/) - Web framework
- [TypeScript](https://www.typescriptlang.org/) - Type safety
- [tsx](https://tsx.is/) - TypeScript execution
