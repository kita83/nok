use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct User {
    pub id: Option<String>,
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
            name,
            status: UserStatus::Online,
            last_active: now,
            rooms: Vec::new(),
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
