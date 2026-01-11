# Examples & Workflows

Real-world examples of how developers use Project Scaffold in their daily work.

## Table of Contents

- [Freelance Developer Workflow](#freelance-developer-workflow)
- [Team Standardization](#team-standardization)
- [Microservices Architecture](#microservices-architecture)
- [Building Your Template Library](#building-your-template-library)
- [CI/CD Integration](#cicd-integration)

---

## Team Standardization

Your team needs consistent project structure across all services.

### Create a Team Template Repository

```
your-org/project-templates/
├── backend-service/
│   ├── template.toml
│   ├── src/
│   ├── Dockerfile
│   ├── .github/workflows/
│   └── ...
├── frontend-app/
│   ├── template.toml
│   └── ...
└── data-pipeline/
    ├── template.toml
    └── ...
```

### Team Members Add Templates

```bash
# Each developer runs once:
scaffold add github:your-org/project-templates#backend-service backend
scaffold add github:your-org/project-templates#frontend-app frontend
scaffold add github:your-org/project-templates#data-pipeline pipeline
```

### Creating New Services

```bash
# Anyone on the team:
scaffold create backend -o user-service -y
# Gets: Your org's standard structure, CI/CD, Docker, linting, etc.
```

### Benefits

- **Consistency**: Every service follows the same patterns
- **Onboarding**: New devs are productive on day one
- **Best Practices**: Security, testing, and CI/CD built-in
- **Updates**: Update template repo, team gets improvements on next project

---

## Freelance Developer Workflow

You're a freelance developer who frequently builds REST APIs and web apps for clients.

### Initial Setup

Save your defaults so every project has your info:

```bash
scaffold config set author "Jane Smith"
scaffold config set author_email "jane@freelance.dev"
```

### Starting a New Client Project

```bash
# New client needs an API - ready in seconds
scaffold create fastapi -o acme-api -y

cd acme-api
uv run uvicorn acme_api.main:app --reload
# API running at http://localhost:8000
# Docs at http://localhost:8000/docs
```

### Quick Prototype for a Demo

```bash
# Demo in 30 minutes? No problem
scaffold create nextjs -o demo-app -y

cd demo-app
pnpm dev
# Running at http://localhost:3000
```

### The Time Savings

| Task | Without Scaffold | With Scaffold |
|------|-------------------|-----------------|
| Clone template | 30 sec | - |
| Remove .git history | 10 sec | - |
| Find/replace project name | 2-5 min | - |
| Update author in configs | 1-2 min | - |
| Install dependencies | 1-2 min | Automatic |
| **Total** | **5-10 min** | **10 sec** |

Over 50+ projects per year, that's **4-8 hours saved** on repetitive setup.

---

## Microservices Architecture

You're building a microservices system and need to spin up new services frequently.

### Service Template with Shared Config

Create a template with your infrastructure patterns:

```toml
# template.toml
name = "Microservice"
description = "Production-ready microservice"

[[variables]]
name = "service_name"
description = "Name of the service"

[[variables]]
name = "port"
description = "Service port"
default = "8080"

[[variables]]
name = "needs_database"
description = "Include PostgreSQL config?"
type = "bool"
default = "true"

[[variables]]
name = "message_queue"
description = "Message queue to use"
type = "choice"
choices = ["none", "rabbitmq", "kafka"]
default = "none"

[[conditionals]]
include = "docker-compose.db.yml"
when = "needs_database == true"

[[conditionals]]
include = "src/queue/rabbitmq.py"
when = "message_queue == 'rabbitmq'"

[[conditionals]]
include = "src/queue/kafka.py"
when = "message_queue == 'kafka'"

[hooks]
post_create = [
    "git init",
    "uv sync"
]
```

### Spinning Up Services

```bash
# Auth service with database
scaffold create microservice -o auth-service \
  -v service_name=auth \
  -v port=8001 \
  -v needs_database=true \
  -v message_queue=rabbitmq

# Notification service (no database, uses Kafka)
scaffold create microservice -o notification-service \
  -v service_name=notification \
  -v port=8002 \
  -v needs_database=false \
  -v message_queue=kafka

# Gateway service (minimal)
scaffold create microservice -o gateway \
  -v service_name=gateway \
  -v port=8000 \
  -v needs_database=false \
  -v message_queue=none
```

Each service gets exactly what it needs—no manual file deletion.

---

## Building Your Template Library

Start with bundled templates, then customize over time.

### Phase 1: Use Bundled Templates

```bash
scaffold list
# fastapi [bundled]
# nextjs [bundled]

scaffold create fastapi -o my-api -y
# Works great for basic projects
```

### Phase 2: Customize for Your Needs

After a few projects, you've added authentication, Docker, and CI:

```bash
# Save your improved version as a new template
scaffold add ./my-api fastapi-pro
```

### Phase 3: Specialized Templates

As you solve specific problems, capture them:

```bash
# After integrating Stripe
scaffold add ./stripe-project nextjs-stripe

# After setting up a CLI tool
scaffold add ./cli-project rust-cli

# After building a Chrome extension
scaffold add ./extension chrome-ext
```

### Your Growing Library

```bash
scaffold list
# fastapi [bundled] - Basic FastAPI
# fastapi-pro - FastAPI + Auth + Docker + CI
# nextjs [bundled] - Basic Next.js
# nextjs-stripe - Next.js + Stripe checkout
# rust-cli - Rust CLI with Clap
# chrome-ext - Chrome extension starter
# go-grpc - Go + gRPC + Kubernetes
```

Now starting any project type takes seconds, not hours.

---

## CI/CD Integration

Use scaffold in automated pipelines.

### GitHub Actions: Generate from Template

```yaml
# .github/workflows/create-service.yml
name: Create New Service

on:
  workflow_dispatch:
    inputs:
      service_name:
        description: 'Service name'
        required: true
      template:
        description: 'Template to use'
        required: true
        default: 'backend'

jobs:
  create:
    runs-on: ubuntu-latest
    steps:
      - name: Install scaffold
        run: cargo install project-scaffold

      - name: Add team template
        run: scaffold add github:${{ github.repository_owner }}/templates#${{ inputs.template }} template

      - name: Create service
        run: scaffold create template -o ${{ inputs.service_name }} -y

      - name: Create PR
        uses: peter-evans/create-pull-request@v5
        with:
          title: "Add new service: ${{ inputs.service_name }}"
          body: "Auto-generated from ${{ inputs.template }} template"
```

### Script: Batch Project Creation

```bash
#!/bin/bash
# create-services.sh - Create multiple services at once

services=("users" "orders" "payments" "notifications")

for service in "${services[@]}"; do
  scaffold create microservice -o "$service-service" \
    -v service_name="$service" \
    -y
  echo "Created $service-service"
done
```

### Makefile Integration

```makefile
# Makefile
.PHONY: new-api new-web

new-api:
	@read -p "Service name: " name; \
	scaffold create fastapi -o $$name -y

new-web:
	@read -p "App name: " name; \
	scaffold create nextjs -o $$name -y
```

Usage:
```bash
make new-api
# Service name: user-service
# Creates user-service/ with FastAPI template
```

---

## Quick Reference

### Save Time on Every Project

```bash
# Set defaults once
scaffold config set author "Your Name"
scaffold config set author_email "you@example.com"

# Create projects instantly
scaffold create <template> -o <dir> -y
```

### Build Your Template Library

```bash
# From local directory
scaffold add ./my-template my-template

# From GitHub
scaffold add github:org/repo#path template-name

# List your templates
scaffold list
```

### Automate with Flags

```bash
# Non-interactive mode (CI/CD)
scaffold create template -o output -y

# Pass variables directly
scaffold create template -o output -v key=value -v other=value -y

# Preview without creating
scaffold create template -o output --dry-run
```
