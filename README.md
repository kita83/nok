# nok (ノック)

A terminal-based virtual office application inspired by gather.town and metalife.co.jp, but designed for the terminal with a minimal CUI interface similar to lazygit/lazydocker.

## Features

- Terminal-based virtual office with rooms and user presence
- Gentle "knock" notifications with sound effects and ASCII animations
- Minimal and lightweight interface
- Written in Rust using ratatui

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
