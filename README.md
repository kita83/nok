# nok

A terminal-based chat application built on the Matrix protocol, allowing team members to see each other's presence and send quick "knocks" to get attention through Matrix homeserver infrastructure.

🚧 **Status**: Matrix migration completed! This is the Matrix-compliant version of nok.

## Features

- **Matrix Protocol Compliance**: Full integration with Matrix Client-Server API v1.12
- **Real-time presence awareness** through Matrix presence events
- **Quick knock functionality** via custom Matrix events (`com.nok.knock`)
- **Terminal-based UI** using ratatui with retro aesthetic
- **Sound notifications** using rodio
- **Matrix rooms support** for team communication
- **Conduit homeserver** for lightweight Matrix server deployment
- **End-to-end encryption** support (via matrix-sdk)
- **Data migration** from legacy nok database to Matrix format
- **Cross-platform compatibility** (Linux, macOS, Windows)

## Architecture

### Matrix-Based Architecture (Current)
```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  nok Client     │    │ Conduit          │    │ Other Matrix    │
│  (Rust/TUI)     │◄──►│ Homeserver       │◄──►│ Clients         │
│                 │    │ (Room v10)       │    │ (Element, etc.) │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │
         ▼                       ▼
┌─────────────────┐    ┌──────────────────┐
│ Matrix SDK      │    │ RocksDB          │
│ State Store     │    │ (Conduit)        │
└─────────────────┘    └──────────────────┘
```

### Key Components
- **Conduit Homeserver**: Lightweight Matrix homeserver (Room Version v10)
- **nok Client**: Rust client using matrix-sdk with terminal UI
- **Matrix State Store**: SQLite-based state storage for Matrix SDK
- **Migration Tools**: Legacy data conversion to Matrix format

## Quick Start

### 1. Start Conduit Homeserver
```bash
cd backend/conduit
./start_conduit.sh
```
The script will automatically download the Conduit binary if needed, then start the server on `http://localhost:6167` with domain `nok.local`.

### 2. Create User Account
```bash
cargo build --bin register_test_user
./target/debug/register_test_user
```
Follow the prompts to create a Matrix account on your local homeserver.

### 3. Run nok Client
```bash
cargo run
```
Enter your username and password when prompted.

### 4. Create/Join Rooms
```bash
# Create a test room
cargo build --bin create_test_room
./target/debug/create_test_room

# Or join existing room via client UI
```

## Development & Testing

### Integration Tests
```bash
# Run full Matrix integration tests
./tests/matrix_integration_test.sh

# Manual testing with test client
cargo build --bin test_matrix
./target/debug/test_matrix
```

### Available Test Tools
- `register_test_user`: Create Matrix user accounts
- `create_test_room`: Create Matrix rooms
- `test_matrix`: Interactive Matrix client testing

### Configuration

Conduit configuration is in `backend/conduit/conduit.toml`:
```toml
server_name = "nok.local"
port = 6167
default_room_version = "10"
allow_federation = false
registration_token = "nokdev_registration_token"
```

## Usage

### Terminal Client
- Use **arrow keys** to navigate between users/rooms
- Press **'k'** to send knock to selected user
- Press **'j'** to join/create rooms
- Press **'m'** to send messages
- Press **'s'** for settings
- Press **'q'** to quit
- **Presence updates automatically** via Matrix sync

### Matrix Features
- **Knock Events**: Custom `com.nok.knock` events for attention requests
- **Presence Sync**: Real-time user status via Matrix presence
- **Room Messaging**: Standard Matrix room messaging support
- **User Discovery**: Matrix user directory integration
- **Cross-Client Compatibility**: Works with Element, FluffyChat, etc.

### Command Mode
Press `i` to enter command mode:
```
nok @username         # Send knock to user
/join #room:nok.local # Join Matrix room
/status away          # Set presence status
/help                 # Show help
```

## Matrix Protocol Compliance

nok implements the following Matrix specifications:
- **Client-Server API v1.12**
- **Room Version v10**
- **Custom event types** for nok-specific features:
  - `com.nok.knock` - Knock events for attention requests
- **Standard Matrix authentication** and device management
- **End-to-end encryption** support via matrix-sdk
- **Matrix presence** and typing indicators
- **Matrix sync** for real-time updates

### Custom Events

#### Knock Event (`com.nok.knock`)
```json
{
  "type": "com.nok.knock",
  "content": {
    "target_user": "@user:nok.local",
    "timestamp": 1704067200000
  }
}
```

## Dependencies

### Core Matrix & UI
- `matrix-sdk` (0.11.0) - Matrix Client-Server API implementation
- `ratatui` (0.26.0) - Terminal UI framework
- `tokio` (1.0) - Async runtime
- `serde` (1.0) - Serialization

### Audio & Utils
- `rodio` (0.17.3) - Audio playback
- `chrono` (0.4) - Date/time handling
- `uuid` (1.0) - Unique identifiers
- `dirs` (5.0) - Directory utilities
- `rusqlite` (0.33) - SQLite database (for migration)

## Project Status

✅ **Phase 1**: Matrix Client-Server API Implementation (Complete)  
✅ **Phase 2**: Conduit Homeserver Setup (Complete)  
✅ **Phase 3**: Data Migration Implementation (Complete)  
✅ **Phase 4**: Integration & Testing (Complete)

**Total Progress**: 45/45 tasks complete (100%)

The project has successfully migrated from a custom WebSocket-based architecture to a fully Matrix-compliant implementation that interoperates with the broader Matrix ecosystem.

## Troubleshooting

### Common Issues

1. **Login fails with crypto store error**:
   ```bash
   # Clean state stores
   rm -rf /tmp/nok_test_store_*
   ```

2. **Conduit won't start (LOCK error)**:
   ```bash
   pkill conduit
   cd backend/conduit && ./start_conduit.sh
   ```

3. **Registration fails**:
   - Ensure Conduit is running
   - Check registration token in `conduit.toml`

### Logs and Debugging

- Conduit logs: Check terminal output where Conduit is running
- Matrix SDK logs: Set `RUST_LOG=matrix_sdk=debug`
- Client logs: Embedded in TUI interface

## Contributing

Contributions are welcome! This project has completed its Matrix migration but can benefit from:

- UI/UX improvements for the terminal interface
- Additional Matrix features (threads, reactions, etc.)
- Performance optimizations
- Cross-platform testing
- Integration with other Matrix clients

## License

MIT License

## Authors and Version
- **Authors**: kita83
- **Version**: 0.2.0 (Matrix Edition)
- **Matrix Compliance**: Client-Server API v1.12
- **Room Version**: v10

---

*Built with 🦀 Rust and the Matrix protocol for distributed team collaboration.* 