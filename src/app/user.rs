use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct User {
    pub id: Option<String>, // 既存の内部ID（UUIDなど）
    pub matrix_id: Option<String>, // Matrix User ID (@username:domain形式)
    pub name: String,
    pub status: UserStatus,
    pub last_active: u64,
    pub rooms: Vec<String>, // 所属している部屋のIDリスト
}

#[derive(Clone, PartialEq, Debug)]
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
            id: None,
            matrix_id: None,
            name,
            status: UserStatus::Online,
            last_active: now,
            rooms: Vec::new(),
        }
    }

    /// Create a new user from Matrix User ID
    pub fn from_matrix_id(matrix_id: String) -> Self {
        let username = extract_username_from_matrix_id(&matrix_id);
        let mut user = Self::new(username);
        user.matrix_id = Some(matrix_id);
        user
    }

    /// Set the Matrix User ID for this user
    pub fn set_matrix_id(&mut self, matrix_id: String) {
        self.matrix_id = Some(matrix_id);
    }

    /// Get the Matrix User ID if available
    pub fn matrix_id(&self) -> Option<&str> {
        self.matrix_id.as_deref()
    }

    /// Get a display name (prefer username over Matrix ID)
    pub fn display_name(&self) -> &str {
        &self.name
    }

    /// Convert to Matrix User ID format
    pub fn to_matrix_id(&self, server_name: &str) -> String {
        if let Some(ref matrix_id) = self.matrix_id {
            matrix_id.clone()
        } else {
            format!("@{}:{}", self.name, server_name)
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

    pub fn get_room_names(&self, rooms: &[crate::app::Room]) -> Vec<String> {
        self.rooms.iter()
            .filter_map(|room_id| {
                rooms.iter()
                    .find(|r| r.id.as_ref() == Some(room_id))
                    .map(|r| r.name.clone())
            })
            .collect()
    }
}

/// Helper function to extract username from Matrix User ID
/// @username:domain.com -> username
pub fn extract_username_from_matrix_id(matrix_id: &str) -> String {
    if matrix_id.starts_with('@') {
        if let Some(colon_pos) = matrix_id.find(':') {
            return matrix_id[1..colon_pos].to_string();
        }
    }
    // Fallback: return the original string without @
    matrix_id.trim_start_matches('@').to_string()
}
