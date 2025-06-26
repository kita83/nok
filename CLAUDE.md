# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

nok is a terminal-based virtual office application built on the Matrix protocol. It allows team members to see each other's presence and send quick "knocks" to get attention through Matrix homeserver infrastructure. The project has been successfully migrated from a custom WebSocket-based architecture to a fully Matrix-compliant implementation.

## Core Architecture

- **Rust Client**: Main terminal UI application using matrix-sdk and ratatui
- **React Ink Frontend**: Modern terminal UI alternative using React and TypeScript
- **Conduit Homeserver**: Lightweight Matrix homeserver (Room Version v10) 
- **Matrix Protocol**: Full compliance with Client-Server API v1.12
- **Custom Events**: `com.nok.knock` events for attention requests
- **Python Backend**: Legacy FastAPI backend (for reference/migration purposes)

## Essential Commands

### Development Setup
```bash
# Configure secure credentials (first time setup)
cd backend/conduit
cp .env.example .env
# Edit .env with secure values for NOK_REGISTRATION_TOKEN and NOK_EMERGENCY_PASSWORD

# Start Conduit homeserver (downloads binary automatically if needed)
./start_conduit.sh

# Build all binaries (main client + test tools)
cargo build

# Run main nok client (Rust/ratatui)
cargo run

# Run single test
cargo test <test_name>

# Run all tests
cargo test
./tests/matrix_integration_test.sh

# Lint and format Rust code
cargo fmt
cargo clippy
```

### React Ink Frontend
```bash
# Install dependencies (first time setup)
cd frontend-ink
npm install

# Run Ink frontend in development mode
npm run dev

# Build Ink frontend
npm run build

# Run built Ink frontend
npm start

# Lint and format TypeScript code
npm run lint
npm run type-check
```

### Test Tools
```bash
# Create Matrix user accounts 
cargo build --bin register_test_user
./target/debug/register_test_user

# Create Matrix rooms
cargo build --bin create_test_room
./target/debug/create_test_room

# Interactive Matrix client testing
cargo build --bin test_matrix
./target/debug/test_matrix
```

### Python Backend (Legacy)
```bash
cd backend
# Install dependencies
pip install -r requirements.txt
# or use uv: uv sync

# Setup database and start server
python setup_data.py
python main.py

# Run linting and formatting
ruff check
ruff format

# Run Python tests
cd backend && python -m pytest
```

## Key Implementation Details

### Matrix Integration
- Uses `matrix-sdk` 0.11.0 with E2E encryption support
- State storage via SQLite (`matrix_state.db`)
- Custom event type `com.nok.knock` for knock functionality
- Homeserver: `nok.local:6167` (configurable via environment variables)
- Registration token: Set via `NOK_REGISTRATION_TOKEN` environment variable
- Emergency password: Set via `NOK_EMERGENCY_PASSWORD` environment variable

### Project Structure
- `src/matrix/`: Matrix client implementation and custom events (Rust)
- `src/ui/`: Terminal UI components using ratatui (Rust)
- `src/app/`: Core application state and logic (Rust)
- `src/migration/`: Legacy data migration tools (Rust)
- `frontend-ink/`: React Ink frontend implementation (TypeScript)
  - `src/components/`: React Ink UI components
  - `src/hooks/`: Custom React hooks for Matrix integration
  - `src/store/`: State management using Zustand
  - `src/utils/`: Matrix client wrapper and utilities
- `backend/conduit/`: Conduit homeserver setup and configuration
- `backend/app/`: FastAPI backend (legacy, for reference)

### Configuration Files
- `Cargo.toml`: Main Rust dependencies and binary definitions
- `frontend-ink/package.json`: Node.js dependencies and scripts for Ink frontend
- `frontend-ink/tsconfig.json`: TypeScript configuration for Ink frontend
- `backend/conduit/conduit.toml`: Conduit homeserver configuration
- `backend/pyproject.toml`: Python backend dependencies with ruff configuration

### Testing Strategy
- Integration tests via `tests/matrix_integration_test.sh`
- Manual testing with dedicated test binaries (`test_matrix`, `register_test_user`, `create_test_room`)
- Matrix protocol compliance testing with test users (test1, test2)
- Unit tests via `cargo test` for Rust components
- Python backend tests via `pytest` (if applicable)

## Development Workflow

1. Ensure Conduit homeserver is running before client development
2. Use test binaries for creating users/rooms during development
3. Matrix state stores are temporary (`/tmp/nok_test_store_*`) for testing
4. Clean state stores when troubleshooting authentication issues
5. Check Conduit logs for Matrix protocol debugging
6. Run `cargo fmt` and `cargo clippy` before committing Rust changes
7. Run `ruff check` and `ruff format` for Python code formatting

## Important Notes

- Project is Matrix-compliant and interoperates with other Matrix clients
- Two frontend implementations available:
  - **Rust/ratatui**: Original implementation (stable)
  - **React Ink**: Modern TypeScript implementation (experimental)
- Both frontends share the same Matrix homeserver and are fully compatible
- Legacy Python backend is kept for reference but main development is Rust/TypeScript
- Custom knock events require Matrix clients that understand the `com.nok.knock` event type
- State persistence uses SQLite for both Matrix SDK and legacy backend
- Audio notifications use rodio (Rust) or node audio libraries (Ink)

## Frontend Comparison

| Feature | Rust/ratatui | React Ink |
|---------|--------------|-----------|
| Language | Rust | TypeScript |
| UI Framework | ratatui | React Ink |
| State Management | Native Rust structs | Zustand |
| Matrix Integration | matrix-sdk (Rust) | matrix-js-sdk |
| Development Speed | Moderate | Fast |
| Type Safety | ✅ Excellent | ✅ Excellent |
| Memory Usage | ✅ Low | Moderate |
| Hot Reload | ❌ No | ✅ Yes |
| Ecosystem | Rust crates | NPM packages |