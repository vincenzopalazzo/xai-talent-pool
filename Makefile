.PHONY: dev build preview install clean check format lint server-dev server-build server-check server-clean

# Default target
all: install dev

# Install dependencies
install:
	cd ui && npm install
	cd server && cargo build

# Run the development server
ui-dev:
	cd ui && npm run dev

dev:
	@echo "Starting both UI and server in debug mode..."
	make -j2 server-dev ui-dev

# Build for production
build:
	cd ui && npm run build

# Preview production build
preview:
	cd ui && npm run preview

# Clean build artifacts and node_modules
clean:
	rm -rf ui/node_modules ui/.svelte-kit ui/build

# Run type checking
check:
	cd ui && npm run check

# Format code
format:
	cd ui && npm run format

# Lint code
lint:
	cd ui && npm run lint

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
	make -j2 server-dev dev
