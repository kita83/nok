# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

nok is a terminal-based virtual office application built on the Matrix protocol. It allows team members to see each other's presence and send quick "knocks" to get attention through Matrix homeserver infrastructure. The project has been successfully migrated from a custom WebSocket-based architecture to a fully Matrix-compliant implementation.

## Core Architecture

- **Rust Client**: Main terminal UI application using matrix-sdk and ratatui
- **Conduit Homeserver**: Lightweight Matrix homeserver (Room Version v10) 
- **Matrix Protocol**: Full compliance with Client-Server API v1.12
- **Custom Events**: `com.nok.knock` events for attention requests
- **Python Backend**: Legacy FastAPI backend (for reference/migration purposes)

## Essential Commands

### Development Setup
```bash
# Start Conduit homeserver
cd backend/conduit && ./start_conduit.sh

# Build all binaries (main client + test tools)
cargo build

# Run main nok client
cargo run

# Run tests
cargo test
./tests/matrix_integration_test.sh
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

# Run linting
ruff check
ruff format
```

## Key Implementation Details

### Matrix Integration
- Uses `matrix-sdk` 0.11.0 with E2E encryption support
- State storage via SQLite (`matrix_state.db`)
- Custom event type `com.nok.knock` for knock functionality
- Homeserver: `nok.local:6167` (configurable in `backend/conduit/conduit.toml`)
- Registration token: `nokdev_registration_token`

### Project Structure
- `src/matrix/`: Matrix client implementation and custom events
- `src/ui/`: Terminal UI components using ratatui
- `src/app/`: Core application state and logic
- `src/migration/`: Legacy data migration tools
- `backend/conduit/`: Conduit homeserver setup and configuration
- `backend/app/`: FastAPI backend (legacy, for reference)

### Configuration Files
- `Cargo.toml`: Main Rust dependencies and binary definitions
- `backend/conduit/conduit.toml`: Conduit homeserver configuration
- `backend/pyproject.toml`: Python backend dependencies with ruff configuration

### Testing Strategy
- Integration tests via `tests/matrix_integration_test.sh`
- Manual testing with dedicated test binaries
- Matrix protocol compliance testing with test users (test1, test2)

## Development Workflow

1. Ensure Conduit homeserver is running before client development
2. Use test binaries for creating users/rooms during development
3. Matrix state stores are temporary (`/tmp/nok_test_store_*`) for testing
4. Clean state stores when troubleshooting authentication issues
5. Check Conduit logs for Matrix protocol debugging

## Important Notes

- Project is Matrix-compliant and interoperates with other Matrix clients
- Legacy Python backend is kept for reference but main development is Rust
- Custom knock events require Matrix clients that understand the `com.nok.knock` event type
- State persistence uses SQLite for both Matrix SDK and legacy backend
- Audio notifications use rodio for cross-platform compatibility