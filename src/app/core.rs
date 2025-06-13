use super::state::AppState;
use super::user::{User, UserStatus};
use super::room::Room;
use super::message::Message;
use super::config::Config;
use crate::ui::TabView;
use crate::util::{ValidationError, NokError, NokResult};
use chrono;

/// Core application state - minimal essential data
#[derive(Debug)]
pub struct AppCore {
    pub state: AppState,
    pub view: TabView,
    pub focused_pane: PaneIdentifier,
    pub should_quit: bool,
    pub notification: Option<String>,
    pub error: Option<String>,
    pub config: Config,
}

/// UI-specific state management
#[derive(Debug)]
pub struct UiState {
    pub input: String,
    pub selected_user: Option<usize>,
    pub selected_room_idx: usize,
    pub selected_message_idx: Option<usize>,
    pub my_aa_position: (u16, u16), // ASCII Art pane position
    pub username_edit_buffer: String,
    pub status_selection_index: usize,
}

/// Data collections and content management
#[derive(Debug)]
pub struct DataState {
    pub users: Vec<User>,
    pub rooms: Vec<Room>,
    pub messages: Vec<Message>,
    pub current_user: User,
    pub current_room: usize,
}

/// Logging and debugging information
#[derive(Debug)]
pub struct LogState {
    pub debug_logs: Vec<String>,
    pub settings_logs: Vec<String>,
    pub max_debug_logs: usize,
}

/// Connection and network state
#[derive(Debug)]
pub struct NetworkState {
    pub connection_status: ConnectionStatus,
    pub should_reconnect: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PaneIdentifier {
    Rooms,
    Users,
    Messages,
    AsciiArt,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

impl AppCore {
    pub fn new(config: Config) -> Self {
        Self {
            state: AppState::Normal,
            view: TabView::Rooms,
            focused_pane: PaneIdentifier::Rooms,
            should_quit: false,
            notification: None,
            error: None,
            config,
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn set_notification(&mut self, msg: String) {
        self.notification = Some(msg);
    }

    pub fn clear_notification(&mut self) {
        self.notification = None;
    }
}

impl UiState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            selected_user: None,
            selected_room_idx: 0,
            selected_message_idx: None,
            my_aa_position: (0, 0),
            username_edit_buffer: String::new(),
            status_selection_index: 0,
        }
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    pub fn reset_selections(&mut self) {
        self.selected_user = None;
        self.selected_room_idx = 0;
        self.selected_message_idx = None;
    }
}

impl DataState {
    pub fn new(current_user: User) -> Self {
        Self {
            users: Vec::new(),
            rooms: Vec::new(),
            messages: Vec::new(),
            current_user,
            current_room: 0,
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn get_current_room(&self) -> Option<&Room> {
        self.rooms.get(self.current_room)
    }

    pub fn get_current_room_mut(&mut self) -> Option<&mut Room> {
        self.rooms.get_mut(self.current_room)
    }

    pub fn get_selected_user(&self, selected_idx: Option<usize>) -> Option<&User> {
        selected_idx.and_then(|idx| self.users.get(idx))
    }

    pub fn set_current_room_idx(&mut self, idx: usize) {
        if idx < self.rooms.len() {
            self.current_room = idx;
        }
    }
}

impl LogState {
    pub fn new() -> Self {
        Self {
            debug_logs: Vec::new(),
            settings_logs: Vec::new(),
            max_debug_logs: 100,
        }
    }

    pub fn add_debug_log(&mut self, message: String) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        let log_entry = format!("[{}] {}", timestamp, message);
        
        self.debug_logs.push(log_entry);
        
        // Limit debug logs to prevent memory issues
        if self.debug_logs.len() > self.max_debug_logs {
            self.debug_logs.remove(0);
        }
    }

    pub fn add_settings_log(&mut self, message: String) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        let log_entry = format!("[{}] {}", timestamp, message);
        self.settings_logs.push(log_entry);
    }

    pub fn clear_debug_logs(&mut self) {
        self.debug_logs.clear();
    }

    pub fn clear_settings_logs(&mut self) {
        self.settings_logs.clear();
    }
}

impl NetworkState {
    pub fn new() -> Self {
        Self {
            connection_status: ConnectionStatus::Disconnected,
            should_reconnect: false,
        }
    }

    pub fn set_connected(&mut self) {
        self.connection_status = ConnectionStatus::Connected;
        self.should_reconnect = false;
    }

    pub fn set_connecting(&mut self) {
        self.connection_status = ConnectionStatus::Connecting;
    }

    pub fn set_disconnected(&mut self) {
        self.connection_status = ConnectionStatus::Disconnected;
    }

    pub fn set_error(&mut self, error: String) {
        self.connection_status = ConnectionStatus::Error(error);
    }

    pub fn request_reconnect(&mut self) {
        self.should_reconnect = true;
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connected)
    }

    pub fn needs_reconnect(&self) -> bool {
        self.should_reconnect
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LogState {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for NetworkState {
    fn default() -> Self {
        Self::new()
    }
}