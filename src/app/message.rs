use std::time::{SystemTime, UNIX_EPOCH};

pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
    pub room: String,
}

impl Message {
    pub fn new(sender: String, content: String, room: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Self {
            sender,
            content,
            timestamp: now,
            room,
        }
    }
    
    pub fn formatted_time(&self) -> String {
        // Simple formatting for now
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let diff = now - self.timestamp;
        
        if diff < 60 {
            format!("{}s ago", diff)
        } else if diff < 3600 {
            format!("{}m ago", diff / 60)
        } else if diff < 86400 {
            format!("{}h ago", diff / 3600)
        } else {
            format!("{}d ago", diff / 86400)
        }
    }
}
