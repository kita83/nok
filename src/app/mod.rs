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
    pub error: Option<String>,
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
            error: None,
        }
    }
    
    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Normal => {
                match key.code {
                    KeyCode::Char('i') => self.state = AppState::Input,
                    KeyCode::Tab => self.select_next_user(),
                    _ => {}
                }
            },
            AppState::Input => {
                match key.code {
                    KeyCode::Enter => {
                        let input_clone = self.input.clone();
                        self.handle_command(&input_clone);
                        self.input.clear();
                        self.state = AppState::Normal;
                    },
                    KeyCode::Esc => {
                        self.input.clear();
                        self.state = AppState::Normal;
                    },
                    KeyCode::Char(c) => self.input.push(c),
                    KeyCode::Backspace => { self.input.pop(); },
                    _ => {}
                }
            },
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
