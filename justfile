# Just recipe file for time-to-rust project

# Display help
@help:
    echo "***Commands***"
    echo ""
    echo "Usage:"
    echo "  just <recipe>"
    echo ""
    just --list

# Run clippy linter
clippy:
    cargo clippy

# Format code
format:
    cargo fmt

# Kill and remove containers
kill:
    docker-compose down

# Start all services locally with rebuild
local:
    docker-compose up --build

# Build and start only the database
dev-db:
    docker-compose -f docker-compose-db-only.yaml up
