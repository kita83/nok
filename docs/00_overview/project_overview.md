# Project Overview: nok

## What is "nok"?

**nok** is a minimalist, retro-styled, terminal-based virtual office tool that allows distributed teams to feel present, reachable, and casually connected — all from the command line. Inspired by the simple act of knocking on a door, nok brings presence awareness and real-time communication to remote-first teams, one gentle "kon kon" at a time.

The name "nok" (knock) derives from the concept of gently knocking on someone's door, representing a non-intrusive way to initiate communication.

## ⚙️ Key Features

* **Terminal UI (TUI)**: Lightweight, retro feel that blends into developer workflows using ratatui
* **Knock to Notify**: Send a subtle "knock" (sound + ASCII animation) to others
* **Presence Indicators**: See who is available, away, or offline in real time
* **Quick Commands**: `nok @user` — fast and human
* **Tab Switching**: View switching with r (rooms), u (users), c (chat) keys
* **Voice Readout**: Optional text-to-speech feature that reads messages aloud — bridging the gap between chat and casual voice presence
* **Command Mode**: Enter command mode with `i` key to send messages, join rooms, etc.

## 🏛️ Architecture

nok is designed with the following module structure:

```
nok/
├── src/
│   ├── app/           # Application core logic
│   │   ├── mod.rs     # Module definition and App struct
│   │   ├── state.rs   # Application state management
│   │   ├── user.rs    # User-related structs and functions
│   │   ├── room.rs    # Room-related structs and functions
│   │   └── message.rs # Message-related structs and functions
│   ├── ui/            # User interface
│   │   └── mod.rs     # UI rendering logic
│   ├── audio/         # Audio functionality
│   │   ├── mod.rs     # Sound playback logic
│   │   └── knock.mp3  # Knock sound file
│   ├── util/          # Utility functions
│   │   └── mod.rs     # Helper functions for file operations, etc.
│   ├── main.rs        # Application entry point
│   └── lib.rs         # Library definition
└── Cargo.toml         # Dependencies and configuration
```

### Key Components

#### App

The `App` struct manages the application state and processes user input. Main features:

- User management
- Room management
- Message processing
- Command processing
- Knock functionality

#### UI

The `ui` module uses ratatui to render the terminal UI. Main features:

- Single-line display
- Tab switching
- User list display
- Room list display
- Chat display
- Input field

#### Audio

The `audio` module uses the rodio library to play sound effects. Main features:

- Knock sound playback
- Optional text-to-speech (TTS) readouts for incoming messages

## 🔧 Tech Stack

* **Language**: Rust
* **UI Framework**: ratatui
* **Audio**: rodio
* **Serialization**: serde (for configuration files)

## 📆 Use Cases

* Remote teams who want low-friction check-ins
* Coworking-style ambient presence
* Async workspaces that want to simulate "being around"
* Paired work / gentle pings without jumping on a call

## ✨ Philosophy

* **Presence, not pressure**: Create a sense of connection without demanding interaction
* **CLI-native**: Designed for terminal-native workers
* **Simple is scalable**: Build small, work well

## 🚀 Usage

### Installation

```bash
# Clone the repository
git clone https://github.com/username/nok.git
cd nok

# Build and run
cargo build
cargo run
```

### Keyboard Shortcuts

- `r`: Switch to rooms view
- `u`: Switch to users view
- `c`: Switch to chat view
- `i`: Switch to input mode
- `Tab`: Select next user
- `n`: Knock on the selected user
- `q`: Exit the application

### Commands

In input mode (`i` key), you can use the following commands:

- `nok @username`: Knock on the specified user

## 🔮 Future Development Plans

- **User Authentication**: Login functionality using Google authentication, etc.
- **Real-time Communication**: WebSocket-based real-time sync between users
- **Persistent Storage**: Database integration for message history and user settings
- **Configuration Files**: Customization of user settings
- **Plugin Functionality**: Plugin system for feature extensions
- **Multilingual Support**: Internationalization support

## 📜 License

This project is released under the MIT License. See the LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Bug reports, feature requests, pull requests, or any form of contribution are greatly appreciated.

## 🙏 Acknowledgements

- [ratatui](https://github.com/ratatui-org/ratatui) - Modern terminal UI framework
- [rodio](https://github.com/RustAudio/rodio) - Audio playback library
- [lazygit](https://github.com/jesseduffield/lazygit) - CUI application that inspired this project
