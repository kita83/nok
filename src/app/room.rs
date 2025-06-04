pub struct Room {
    pub id: Option<String>,
    pub matrix_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub users: Vec<String>,
    pub topic: Option<String>,
    pub is_encrypted: bool,
    pub member_count: usize,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            matrix_id: None,
            name,
            description: None,
            users: Vec::new(),
            topic: None,
            is_encrypted: false,
            member_count: 0,
        }
    }

    pub fn from_matrix_room(matrix_id: String, name: String) -> Self {
        let mut room = Self::new(name);
        room.matrix_id = Some(matrix_id);
        room
    }

    pub fn set_matrix_id(&mut self, matrix_id: String) {
        self.matrix_id = Some(matrix_id);
    }

    pub fn matrix_id(&self) -> Option<&str> {
        self.matrix_id.as_deref()
    }

    pub fn display_name(&self) -> &str {
        &self.name
    }

    pub fn set_topic(&mut self, topic: Option<String>) {
        self.topic = topic;
    }

    pub fn set_encrypted(&mut self, encrypted: bool) {
        self.is_encrypted = encrypted;
    }

    pub fn set_member_count(&mut self, count: usize) {
        self.member_count = count;
    }

    pub fn add_user(&mut self, username: String) {
        if !self.users.contains(&username) {
            self.users.push(username);
            self.member_count = self.users.len();
        }
    }

    pub fn remove_user(&mut self, username: &str) {
        self.users.retain(|u| u != username);
        self.member_count = self.users.len();
    }

    pub fn has_user(&self, username: &str) -> bool {
        self.users.contains(&username.to_string())
    }

    pub fn info_string(&self) -> String {
        let encryption_status = if self.is_encrypted { "🔒" } else { "" };
        let member_info = if self.member_count > 0 {
            format!(" ({} members)", self.member_count)
        } else {
            String::new()
        };

        format!("{}{}{}", encryption_status, self.name, member_info)
    }
}
