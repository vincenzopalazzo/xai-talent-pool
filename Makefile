.PHONY: dev build preview install clean check format lint server-dev server-build server-check server-clean grok-install grok-dev grok-format grok-lint grok-test grok-check

# Default target
all: install dev

# Install dependencies
install:
	cd ui && npm install
	cd server && cargo build
	cd grok-service && uv venv && uv pip install -e ".[dev]"

# Run the development server
ui-dev:
	cd ui && npm run dev

dev:
	@echo "Starting UI, server, and grok service in debug mode..."
	make -j3 server-dev ui-dev grok-dev

# Build for production
build:
	cd ui && npm run build

# Preview production build
preview:
	cd ui && npm run preview

# Clean build artifacts and node_modules
clean:
	rm -rf ui/node_modules ui/.svelte-kit ui/build
	rm -rf grok-service/.venv grok-service/*.egg-info grok-service/src/*.egg-info grok-service/.uv

# Run type checking
check:
	cd ui && npm run check
	cd grok-service && uv run black --check src tests && uv run ruff check src tests

# Format code
format:
	cd ui && npm run format
	cd grok-service && uv run black src tests

# Lint code
lint:
	cd ui && npm run lint
	cd grok-service && uv run ruff check src tests

# Server commands
server-dev:
	cd server && cargo run

server-build:
	cd server && cargo build --release

server-check:
	cd server && cargo check

server-prepare:
	cd server && cargo sqlx prepare

server-clean:
	cd server && cargo clean

dev-all:
	make -j3 server-dev ui-dev grok-dev

# Grok service commands
# uv manages virtual environments and dependencies automatically
grok-install:
	cd grok-service && uv venv && uv pip install -e ".[dev]"

grok-dev:
	cd grok-service && uv run uvicorn grok_service.main:app --reload --host 0.0.0.0 --port 8001

grok-format:
	cd grok-service && uv run black src tests

grok-lint:
	cd grok-service && uv run ruff check src tests

grok-test:
	cd grok-service && uv run pytest

grok-check:
	cd grok-service && uv run black --check src tests && uv run ruff check src tests
