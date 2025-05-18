pub struct Room {
    pub name: String,
    pub users: Vec<String>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            name,
            users: Vec::new(),
        }
    }
    
    pub fn add_user(&mut self, username: String) {
        if !self.users.contains(&username) {
            self.users.push(username);
        }
    }
    
    pub fn remove_user(&mut self, username: &str) {
        self.users.retain(|u| u != username);
    }
}
