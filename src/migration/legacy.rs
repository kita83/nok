use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Legacy user data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyUser {
    pub id: String,
    pub name: String,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Legacy room data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyRoom {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Legacy message data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyMessage {
    pub id: String,
    pub content: String,
    pub message_type: Option<String>,
    pub sender_id: String,
    pub room_id: Option<String>,
    pub target_user_id: Option<String>,
    pub created_at: Option<String>,
}

/// Legacy room membership data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyRoomMembership {
    pub user_id: String,
    pub room_id: String,
    pub joined_at: Option<String>,
}

/// Complete legacy data set
#[derive(Debug)]
pub struct LegacyData {
    pub users: Vec<LegacyUser>,
    pub rooms: Vec<LegacyRoom>,
    pub messages: Vec<LegacyMessage>,
    pub room_memberships: Vec<LegacyRoomMembership>,
    pub user_rooms: HashMap<String, Vec<String>>, // user_id -> room_ids
    pub room_members: HashMap<String, Vec<String>>, // room_id -> user_ids
}

/// Legacy data loader from SQLite database
pub struct LegacyDataLoader {
    db_path: String,
}

impl LegacyDataLoader {
    /// Create a new legacy data loader
    pub fn new(db_path: impl Into<String>) -> Self {
        Self {
            db_path: db_path.into(),
        }
    }

    /// Load all legacy data from the database
    pub async fn load_all(&self) -> Result<LegacyData, Box<dyn std::error::Error>> {
        // For this implementation, we'll use rusqlite to read the SQLite database
        // Note: In a real async context, you might want to use sqlx or similar

        use std::path::Path;
        if !Path::new(&self.db_path).exists() {
            return Err(format!("Legacy database not found: {}", self.db_path).into());
        }

        let conn = rusqlite::Connection::open(&self.db_path)?;

        // Load users
        let users = self.load_users(&conn)?;
        println!("📊 Loaded {} users", users.len());

        // Load rooms
        let rooms = self.load_rooms(&conn)?;
        println!("📊 Loaded {} rooms", rooms.len());

        // Load messages
        let messages = self.load_messages(&conn)?;
        println!("📊 Loaded {} messages", messages.len());

        // Load room memberships
        let room_memberships = self.load_room_memberships(&conn)?;
        println!("📊 Loaded {} room memberships", room_memberships.len());

        // Build lookup maps
        let (user_rooms, room_members) = self.build_membership_maps(&room_memberships);

        Ok(LegacyData {
            users,
            rooms,
            messages,
            room_memberships,
            user_rooms,
            room_members,
        })
    }

    /// Load users from database
    fn load_users(&self, conn: &rusqlite::Connection) -> Result<Vec<LegacyUser>, rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT id, name, status, created_at, updated_at FROM users")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(LegacyUser {
                id: row.get(0)?,
                name: row.get(1)?,
                status: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        let mut users = Vec::new();
        for user in user_iter {
            users.push(user?);
        }
        Ok(users)
    }

    /// Load rooms from database
    fn load_rooms(&self, conn: &rusqlite::Connection) -> Result<Vec<LegacyRoom>, rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT id, name, description, is_public, created_at, updated_at FROM rooms")?;
        let room_iter = stmt.query_map([], |row| {
            Ok(LegacyRoom {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                is_public: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut rooms = Vec::new();
        for room in room_iter {
            rooms.push(room?);
        }
        Ok(rooms)
    }

    /// Load messages from database
    fn load_messages(&self, conn: &rusqlite::Connection) -> Result<Vec<LegacyMessage>, rusqlite::Error> {
        let mut stmt = conn.prepare(
            "SELECT id, content, message_type, sender_id, room_id, target_user_id, created_at FROM messages"
        )?;
        let message_iter = stmt.query_map([], |row| {
            Ok(LegacyMessage {
                id: row.get(0)?,
                content: row.get(1)?,
                message_type: row.get(2)?,
                sender_id: row.get(3)?,
                room_id: row.get(4)?,
                target_user_id: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;

        let mut messages = Vec::new();
        for message in message_iter {
            messages.push(message?);
        }
        Ok(messages)
    }

    /// Load room memberships from database
    fn load_room_memberships(&self, conn: &rusqlite::Connection) -> Result<Vec<LegacyRoomMembership>, rusqlite::Error> {
        let mut stmt = conn.prepare("SELECT user_id, room_id, joined_at FROM room_members")?;
        let membership_iter = stmt.query_map([], |row| {
            Ok(LegacyRoomMembership {
                user_id: row.get(0)?,
                room_id: row.get(1)?,
                joined_at: row.get(2)?,
            })
        })?;

        let mut memberships = Vec::new();
        for membership in membership_iter {
            memberships.push(membership?);
        }
        Ok(memberships)
    }

    /// Build lookup maps for user-room relationships
    fn build_membership_maps(&self, memberships: &[LegacyRoomMembership])
        -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {

        let mut user_rooms: HashMap<String, Vec<String>> = HashMap::new();
        let mut room_members: HashMap<String, Vec<String>> = HashMap::new();

        for membership in memberships {
            // Add room to user's room list
            user_rooms.entry(membership.user_id.clone())
                .or_insert_with(Vec::new)
                .push(membership.room_id.clone());

            // Add user to room's member list
            room_members.entry(membership.room_id.clone())
                .or_insert_with(Vec::new)
                .push(membership.user_id.clone());
        }

        (user_rooms, room_members)
    }

    /// Get user by ID
    pub fn get_user_by_id<'a>(&self, users: &'a [LegacyUser], user_id: &str) -> Option<&'a LegacyUser> {
        users.iter().find(|u| u.id == user_id)
    }

    /// Get room by ID
    pub fn get_room_by_id<'a>(&self, rooms: &'a [LegacyRoom], room_id: &str) -> Option<&'a LegacyRoom> {
        rooms.iter().find(|r| r.id == room_id)
    }

    /// Convert legacy status to Matrix presence
    pub fn legacy_status_to_matrix_presence(status: &Option<String>) -> crate::app::user::UserStatus {
        match status.as_deref() {
            Some("online") => crate::app::user::UserStatus::Online,
            Some("away") => crate::app::user::UserStatus::Away,
            Some("busy") => crate::app::user::UserStatus::Busy,
            Some("offline") | None => crate::app::user::UserStatus::Offline,
            _ => crate::app::user::UserStatus::Offline,
        }
    }
}