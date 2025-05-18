use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub status: UserStatus,
    pub last_active: u64,
}

#[derive(Clone, PartialEq)]
pub enum UserStatus {
    Online,
    Away,
    Busy,
    Offline,
}

impl User {
    pub fn new(name: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Self {
            name,
            status: UserStatus::Online,
            last_active: now,
        }
    }
    
    pub fn update_status(&mut self, status: UserStatus) {
        self.status = status;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        self.last_active = now;
    }
    
    pub fn is_available(&self) -> bool {
        self.status == UserStatus::Online || self.status == UserStatus::Away
    }
}
