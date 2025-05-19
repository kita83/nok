mod state;
pub mod user;
mod room;
mod message;

use crossterm::event::KeyEvent;
use crossterm::event::KeyCode;
pub use state::AppState;
pub use user::User;
pub use room::Room;
pub use message::Message;
use crate::ui::TabView;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PaneIdentifier {
    Rooms,
    Users,
    Messages,
    AsciiArt,
}

const AA_PANE_WIDTH: u16 = 60; // Approximate width, adjust as needed
const AA_PANE_HEIGHT: u16 = 20; // Approximate height, adjust as needed

pub struct App {
    pub state: AppState,
    pub users: Vec<User>,
    pub rooms: Vec<Room>,
    pub messages: Vec<Message>,
    pub current_user: User,
    pub current_room: usize,
    pub input: String,
    pub notification: Option<String>,
    pub selected_user: Option<usize>,
    pub selected_room_idx: usize,
    pub selected_message_idx: Option<usize>,
    pub error: Option<String>,
    pub view: TabView,
    pub focused_pane: PaneIdentifier,
    pub my_aa_position: (u16, u16), // (x, y) for player in ASCII Art pane
}

impl App {
    pub fn new() -> Self {
        // Create a default user
        let current_user = User::new("You".to_string());

        // Create some example rooms
        let rooms = vec![
            Room::new("Main Room".to_string()),
            Room::new("Meeting Room".to_string()),
            Room::new("Break Room".to_string()),
        ];

        // Create some example users
        let users = vec![
            User::new("Alice".to_string()),
            User::new("Bob".to_string()),
            User::new("Charlie".to_string()),
        ];

        Self {
            state: AppState::Normal,
            users,
            rooms,
            messages: Vec::new(),
            current_user,
            current_room: 0,
            input: String::new(),
            notification: None,
            selected_user: None,
            selected_room_idx: 0,
            selected_message_idx: None,
            error: None,
            view: TabView::Rooms,
            focused_pane: PaneIdentifier::Rooms,
            my_aa_position: (5, 2), // Initial position for player's AA
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Normal => {
                // First, check for pane-specific keybindings if a certain pane is focused
                if self.focused_pane == PaneIdentifier::AsciiArt {
                    match key.code {
                        KeyCode::Char('i') => { // Move Up
                            self.my_aa_position.1 = self.my_aa_position.1.saturating_sub(1).max(1); // Min Y boundary
                            return; // Consume the key event
                        }
                        KeyCode::Char('k') => { // Move Down
                            self.my_aa_position.1 = (self.my_aa_position.1 + 1).min(AA_PANE_HEIGHT - 2); // Max Y boundary
                            return; // Consume the key event
                        }
                        KeyCode::Char('j') => { // Move Left
                            self.my_aa_position.0 = self.my_aa_position.0.saturating_sub(1).max(1); // Min X boundary
                            return;
                        }
                        KeyCode::Char('l') => { // Move Right
                            self.my_aa_position.0 = (self.my_aa_position.0 + 1).min(AA_PANE_WIDTH - 10); // Max X boundary (leave space for icon+name)
                            return;
                        }
                        _ => {} // Other keys will fall through to global keybindings
                    }
                }

                // Global keybindings (if not consumed by pane-specific logic above)
                match key.code {
                    KeyCode::Char('r') => self.focused_pane = PaneIdentifier::Rooms,
                    KeyCode::Char('u') => self.focused_pane = PaneIdentifier::Users,
                    KeyCode::Char('c') => self.focused_pane = PaneIdentifier::Messages,
                    KeyCode::Char('p') => self.focused_pane = PaneIdentifier::AsciiArt,
                    KeyCode::Char('1') => self.focused_pane = PaneIdentifier::Rooms,
                    KeyCode::Char('2') => self.focused_pane = PaneIdentifier::Users,
                    KeyCode::Char('3') => self.focused_pane = PaneIdentifier::Messages,
                    KeyCode::Char('4') => self.focused_pane = PaneIdentifier::AsciiArt,
                    KeyCode::Tab => {
                        self.cycle_focus(false);
                    },
                    KeyCode::BackTab => {
                        self.cycle_focus(true);
                    },
                    KeyCode::Down | KeyCode::Char('j') => self.handle_down_key(),
                    KeyCode::Up | KeyCode::Char('k') => self.handle_up_key(),
                    KeyCode::Enter => self.handle_enter_key(),
                    KeyCode::Char('i') => {
                        if self.focused_pane == PaneIdentifier::Messages ||
                           self.focused_pane == PaneIdentifier::Rooms ||
                           self.focused_pane == PaneIdentifier::Users {
                            self.state = AppState::Input;
                            self.input.clear();
                            self.error = None;
                            self.notification = None;
                        }
                    },
                    _ => {}
                }
            },
            AppState::Input => {
                match key.code {
                    KeyCode::Enter => {
                        let input_clone = self.input.trim().to_string();
                        if !input_clone.is_empty() {
                            if self.focused_pane == PaneIdentifier::Messages ||
                               self.focused_pane == PaneIdentifier::Rooms ||
                               self.focused_pane == PaneIdentifier::Users {

                                if input_clone.starts_with("nok @") {
                                    self.handle_command(&input_clone);
                                } else {
                                    let sender_name = self.current_user.name.clone();
                                    let room_name = self.rooms.get(self.current_room)
                                        .map_or_else(|| "Unknown Room".to_string(), |r| r.name.clone());
                                    let new_message = Message::new(sender_name, input_clone, room_name);
                                    self.messages.push(new_message);
                                }
                            }
                        }
                        self.input.clear();
                        self.state = AppState::Normal;
                    },
                    KeyCode::Esc => {
                        self.input.clear();
                        self.state = AppState::Normal;
                        self.error = None;
                        self.notification = None;
                    },
                    KeyCode::Char(c) => self.input.push(c),
                    KeyCode::Backspace => { self.input.pop(); },
                    _ => {}
                }
            },
        }
    }

    fn cycle_focus(&mut self, backward: bool) {
        let panes = [
            PaneIdentifier::Rooms,
            PaneIdentifier::Users,
            PaneIdentifier::Messages,
            PaneIdentifier::AsciiArt,
        ];
        let current_idx = panes.iter().position(|&p| p == self.focused_pane).unwrap_or(0);
        let next_idx = if backward {
            if current_idx == 0 { panes.len() - 1 } else { current_idx - 1 }
        } else {
            (current_idx + 1) % panes.len()
        };
        self.focused_pane = panes[next_idx];
    }

    fn handle_down_key(&mut self) {
        match self.focused_pane {
            PaneIdentifier::Rooms => {
                if !self.rooms.is_empty() {
                    self.selected_room_idx = (self.selected_room_idx + 1) % self.rooms.len();
                }
            }
            PaneIdentifier::Users => {
                if !self.users.is_empty() {
                    let current_selected = self.selected_user.unwrap_or(0);
                    self.selected_user = Some((current_selected + 1) % self.users.len());
                }
            }
            PaneIdentifier::Messages => {
                // TODO: Implement message scrolling/selection down
            }
            _ => {}
        }
    }

    fn handle_up_key(&mut self) {
        match self.focused_pane {
            PaneIdentifier::Rooms => {
                if !self.rooms.is_empty() {
                    if self.selected_room_idx == 0 {
                        self.selected_room_idx = self.rooms.len() - 1;
                    } else {
                        self.selected_room_idx -= 1;
                    }
                }
            }
            PaneIdentifier::Users => {
                if !self.users.is_empty() {
                    let current_selected = self.selected_user.unwrap_or(0);
                    if current_selected == 0 {
                        self.selected_user = Some(self.users.len() - 1);
                    } else {
                        self.selected_user = Some(current_selected - 1);
                    }
                }
            }
            PaneIdentifier::Messages => {
                // TODO: Implement message scrolling/selection up
            }
            _ => {}
        }
    }

    fn handle_enter_key(&mut self) {
        match self.focused_pane {
            PaneIdentifier::Rooms => {
                self.current_room = self.selected_room_idx;
                self.notification = Some(format!("Joined room: {}", self.rooms[self.current_room].name));
            }
            PaneIdentifier::Users => {
                if let Some(user_idx) = self.selected_user {
                    self.notification = Some(format!("Selected user: {}", self.users[user_idx].name));
                }
            }
            PaneIdentifier::Messages => {
                // TODO: Implement action for selecting a message (e.g., reply, react)
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        // Update app state on tick
    }

    pub fn knock(&mut self, target_user: &str) {
        // Implement the knock functionality
        self.notification = Some(format!("░░░ KON KON ░░░\n> {}さんのドアをノックしました。", target_user));

        // Here we would play the knock sound
        // audio::play_knock_sound();
    }

    pub fn get_selected_user(&self) -> Option<&User> {
        self.selected_user.and_then(|idx| self.users.get(idx))
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn select_next_user(&mut self) {
        if !self.users.is_empty() {
            let new_idx = match self.selected_user {
                Some(idx) => (idx + 1) % self.users.len(),
                None => 0,
            };
            self.selected_user = Some(new_idx);
        }
    }

    pub fn handle_command(&mut self, input: &str) {
        if input.starts_with("nok @") {
            let username = input.trim_start_matches("nok @").trim();
            let user_exists = self.users.iter().any(|u| u.name == username);

            if user_exists {
                self.knock(username);

                // Play knock sound
                if let Err(e) = crate::audio::play_knock_sound() {
                    self.set_error(format!("Failed to play sound: {}", e));
                }
            } else {
                self.set_error(format!("User '{}' not found", username));
            }
        }
    }
}
