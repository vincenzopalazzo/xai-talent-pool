.PHONY: dev build preview install clean check format lint server-dev server-build server-check server-clean grok-install grok-dev grok-format grok-lint grok-test grok-check

# Default target
all: install dev

# Install dependencies
install:
	cd ui && npm install
	cd server && cargo build
	cd grok-service && pip install -e ".[dev]"

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
	rm -rf grok-service/.venv grok-service/*.egg-info grok-service/src/*.egg-info

# Run type checking
check:
	cd ui && npm run check
	cd grok-service && python3 -m black --check src tests && python3 -m ruff check src tests

# Format code
format:
	cd ui && npm run format
	cd grok-service && python3 -m black src tests

# Lint code
lint:
	cd ui && npm run lint
	cd grok-service && python3 -m ruff check src tests

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
GROK_VENV = grok-service/.venv
GROK_PYTHON = $(GROK_VENV)/bin/python

$(GROK_VENV):
	python3 -m venv $(GROK_VENV)

grok-install: $(GROK_VENV)
	$(GROK_PYTHON) -m pip install -e "grok-service[dev]"

grok-dev: $(GROK_VENV)
	cd grok-service && GRPC_DNS_RESOLVER=native .venv/bin/python -m uvicorn grok_service.main:app --reload --host 0.0.0.0 --port 8001

grok-format: $(GROK_VENV)
	$(GROK_PYTHON) -m black grok-service/src grok-service/tests

grok-lint: $(GROK_VENV)
	$(GROK_PYTHON) -m ruff check grok-service/src grok-service/tests

grok-test: $(GROK_VENV)
	cd grok-service && GRPC_DNS_RESOLVER=native .venv/bin/python -m pytest

grok-check: $(GROK_VENV)
	$(GROK_PYTHON) -m black --check grok-service/src grok-service/tests && $(GROK_PYTHON) -m ruff check grok-service/src grok-service/tests
