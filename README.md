# nok

🚧 This project is under active development and not ready for production use.

nok is a minimalist, terminal-based virtual office tool that allows distributed teams to feel present, reachable, and casually connected — all from the command line. Inspired by the simple act of knocking on a door, nok brings presence awareness and real-time communication to remote-first teams, one gentle "kon kon" at a time.

## ⚙️ Key Features

* **Terminal UI (TUI)**: Lightweight, retro feel that blends into developer workflows using ratatui
* **Knock to Notify**: Send a subtle "knock" (sound + ASCII animation) to others
* **Presence Indicators**: See who is available, away, or offline in real time
* **Quick Commands**: `nok @user` — fast and human
* **Single-line Display**: Optimized to work in height-constrained terminals
* **Tab Switching**: View switching with r (rooms), u (users), c (chat) keys
* **Voice Readout**: Optional text-to-speech feature that reads messages aloud — bridging the gap between chat and casual voice presence
* **Command Mode**: Enter command mode with `i` key to send messages, join rooms, etc.

## Usage

```
# Run the application
cargo run

# Basic commands
nok @username    # Knock on a user's door
/join room_name  # Join a different room
/status away     # Change your status
/help            # Show help
```

## Development

```
# Build the project
cargo build

# Run tests
cargo test
```

## License

MIT
