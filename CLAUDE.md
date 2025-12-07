# CLAUDE.md

This file provides guidance to Claude Code when working on this project.

## Project Overview

This is a talent pool application for X, built as a monorepo with a Svelte frontend, Rust backend using actix-web, and a Python Grok microservice for AI operations.

## Project Structure

```
xai-talent-pool/
├── Makefile          # Build commands for the monorepo
├── CLAUDE.md         # This file
├── server/           # Rust backend API server
├── grok-service/     # Python Grok microservice
└── ui/               # Svelte frontend application
```

## Development Commands

```bash
make install       # Install dependencies for UI and build server
make dev           # Run both UI and server development servers concurrently (debug mode)
make server-dev    # Run backend server only (debug mode)
make build         # Build UI for production
make server-build  # Build backend
make check         # Type checking for UI
make server-check  # Check backend
make dev-all       # Run both UI dev and server dev concurrently

# Grok service commands
make grok-install  # Install grok-service with dev dependencies
make grok-dev      # Run grok service in development mode (port 8001)
make grok-format   # Format Python code with black
make grok-lint     # Lint Python code with ruff
make grok-test     # Run pytest tests
make grok-check    # Run black and ruff checks
```

## UI Development Rules

### Component Library

**Always use shadcn-svelte for UI components**: https://www.shadcn-svelte.com

- Use existing shadcn-svelte components whenever possible
- Add new components via: `npx shadcn-svelte@latest add <component-name>`
- Components are located in `ui/src/lib/components/ui/`
- Follow shadcn-svelte patterns for component composition

### UI Logic Constraints

**The UI must remain a thin presentation layer**:

1. **No complex business logic in components** - UI components should only handle:
   - Rendering data passed via props
   - Local UI state (toggles, form inputs, modals)
   - Simple data transformations for display (formatting dates, truncating text)

2. **No API calls directly in components** - All data fetching should be handled by:
   - SvelteKit load functions (`+page.ts`, `+layout.ts`)
   - Dedicated service modules in `$lib/services/`

3. **No complex computations** - Avoid:
   - Heavy data processing in components
   - Complex algorithms or calculations
   - Data aggregation or analytics logic

4. **Keep components focused** - Each component should:
   - Do one thing well
   - Accept data via props
   - Emit events for user actions
   - Delegate complex operations to parent components or services

### State Management

- Use Svelte 5 runes (`$state`, `$derived`, `$effect`) for local component state
- For shared state, use Svelte stores in `$lib/stores/`
- Keep state as close to where it's used as possible

### Styling

- Use Tailwind CSS utility classes
- Follow the shadcn-svelte theming system (CSS variables in `app.css`)
- Support both light and dark modes

### TypeScript

- All components must be written in TypeScript
- Define interfaces for component props
- Use strict type checking

## Backend Development Rules

### Server Framework
- Use actix-web for the HTTP server
- Use paperclip (v0.8 with actix4 feature) for OpenAPI v3 documentation generation
- Serve OpenAPI spec at `/api/v1` and Swagger UI at `/`

### API Design
- RESTful endpoints under `/api/v1/`
- Use paperclip macros: `#[paperclip::actix::get("/path")]` etc. for routes and schema
- Models derive `paperclip::actix::Apiv2Schema` and serde traits
- App setup: `.wrap_api().with_json_spec_at("/api/v1")`
- In-memory or sqlx sqlite for dev; production postgres

### Data Access
- No direct db calls in handlers; use repositories/services
- Current: in-memory Vec<Talent> in AppState for basic CRUD

### State Management
- AppState with Arc<Mutex<AppState>> shared via web::Data
- Clone Arcs for fields like repositories

## Grok Service Development Rules

### Framework
- Use FastAPI for the HTTP server
- Use xai-sdk for Grok API integration
- OpenAPI docs at `/docs` (Swagger UI) and `/redoc`

### Code Quality
- Use black for code formatting (line length: 88)
- Use ruff for linting
- Use pytest for testing

### Project Structure
```
grok-service/
├── pyproject.toml           # Project configuration and dependencies
├── src/grok_service/        # Main package
│   ├── __init__.py
│   ├── main.py              # FastAPI application
│   └── config.py            # Settings configuration
└── tests/                   # Test files
```

### Configuration
- Environment variables prefixed with `GROK_`
- `GROK_XAI_API_KEY`: xAI API key for Grok
- `GROK_HOST`: Server host (default: 0.0.0.0)
- `GROK_PORT`: Server port (default: 8001)

### API Design
- RESTful endpoints under `/api/v1/`
- Health check at `/health`
- Use Pydantic models for request/response schemas

## File Naming Conventions

- Components: `kebab-case.svelte` (e.g., `talent-card.svelte`)
- TypeScript modules: `kebab-case.ts`
- Rust modules: snake_case.rs
- Python modules: snake_case.py
- Routes follow SvelteKit conventions (`+page.svelte`, `+layout.svelte`)
- Backend routes: /api/v1/resources/{id}
