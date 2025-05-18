mod state;
mod user;
mod room;
mod message;

use crossterm::event::KeyEvent;
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
        }
    }
    
    pub fn handle_key(&mut self, key: KeyEvent) {
        // Handle key events based on current state
        match self.state {
            AppState::Normal => {
                // Handle normal mode keys
            },
            AppState::Input => {
                // Handle input mode keys
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
}
