use regex::Regex;
use std::collections::HashMap;

/// Convert UUID to Matrix-compatible username
pub fn uuid_to_matrix_username(uuid: &str) -> String {
    // Remove hyphens and convert to lowercase for Matrix username
    // UUID: 0e06fba5-6474-43a0-964a-4fb934b781db -> user0e06fba56474...
    let clean_uuid = uuid.replace('-', "");
    format!("user{}", &clean_uuid[..16]) // Use first 16 chars to keep it reasonable
}

/// Convert room name to Matrix room alias
pub fn room_name_to_matrix_alias(room_name: &str) -> String {
    // Convert Japanese/spaces to ASCII-compatible alias
    // "メインルーム" -> "main_room" or similar
    let alias = room_name
        .chars()
        .filter_map(|c| {
            match c {
                // Keep alphanumeric ASCII characters
                'a'..='z' | 'A'..='Z' | '0'..='9' => Some(c.to_ascii_lowercase()),
                // Convert spaces and punctuation to underscores
                ' ' | '-' | '.' | '/' | '\\' => Some('_'),
                // Handle common Japanese room names
                _ => None, // Remove other characters
            }
        })
        .collect::<String>();

    // Clean up multiple underscores and trim
    let re = Regex::new(r"_+").unwrap();
    let cleaned = re.replace_all(&alias, "_");
    let trimmed = cleaned.trim_matches('_');

    // If empty or too short, use fallback
    if trimmed.is_empty() || trimmed.len() < 3 {
        generate_fallback_alias(room_name)
    } else {
        trimmed.to_string()
    }
}

/// Generate fallback alias for non-ASCII room names
fn generate_fallback_alias(room_name: &str) -> String {
    // Create hash-based alias for non-ASCII names
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    room_name.hash(&mut hasher);
    let hash = hasher.finish();

    format!("room{:x}", hash & 0xFFFFFFFF) // Use lower 32 bits as hex
}

/// Convert legacy User ID to Matrix User ID
pub fn legacy_to_matrix_user_id(legacy_id: &str, server_name: &str) -> String {
    let username = uuid_to_matrix_username(legacy_id);
    format!("@{}:{}", username, server_name)
}

/// Convert legacy Room ID to Matrix Room ID (for internal use)
pub fn legacy_to_matrix_room_id(legacy_id: &str, server_name: &str) -> String {
    // Generate a consistent room ID based on legacy ID
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    legacy_id.hash(&mut hasher);
    let hash = hasher.finish();

    let hash_str = format!("{:016x}", hash); // Ensure 16 characters
    format!("!{}:{}", &hash_str[..std::cmp::min(18, hash_str.len())].to_uppercase(), server_name)
}

/// Matrix ID mapping for migration
#[derive(Debug, Clone)]
pub struct IdMappings {
    pub user_mappings: HashMap<String, String>, // legacy_id -> matrix_id
    pub room_mappings: HashMap<String, String>, // legacy_id -> matrix_id
    pub room_aliases: HashMap<String, String>,  // legacy_id -> matrix_alias
}

impl IdMappings {
    pub fn new() -> Self {
        Self {
            user_mappings: HashMap::new(),
            room_mappings: HashMap::new(),
            room_aliases: HashMap::new(),
        }
    }

    /// Add user mapping
    pub fn add_user_mapping(&mut self, legacy_id: String, matrix_id: String) {
        self.user_mappings.insert(legacy_id, matrix_id);
    }

    /// Add room mapping
    pub fn add_room_mapping(&mut self, legacy_id: String, matrix_id: String, alias: String) {
        self.room_mappings.insert(legacy_id.clone(), matrix_id);
        self.room_aliases.insert(legacy_id, alias);
    }

    /// Get Matrix user ID from legacy ID
    pub fn get_matrix_user_id(&self, legacy_id: &str) -> Option<&String> {
        self.user_mappings.get(legacy_id)
    }

    /// Get Matrix room ID from legacy ID
    pub fn get_matrix_room_id(&self, legacy_id: &str) -> Option<&String> {
        self.room_mappings.get(legacy_id)
    }

    /// Get Matrix room alias from legacy ID
    pub fn get_matrix_room_alias(&self, legacy_id: &str) -> Option<&String> {
        self.room_aliases.get(legacy_id)
    }

    /// Generate all mappings from legacy data
    pub fn generate_from_legacy_data(
        users: &[crate::migration::legacy::LegacyUser],
        rooms: &[crate::migration::legacy::LegacyRoom],
        server_name: &str,
    ) -> Self {
        let mut mappings = Self::new();

        // Generate user mappings
        for user in users {
            let matrix_id = legacy_to_matrix_user_id(&user.id, server_name);
            mappings.add_user_mapping(user.id.clone(), matrix_id);
        }

        // Generate room mappings
        for room in rooms {
            let matrix_id = legacy_to_matrix_room_id(&room.id, server_name);
            let alias = room_name_to_matrix_alias(&room.name);
            mappings.add_room_mapping(room.id.clone(), matrix_id, alias);
        }

        mappings
    }

    /// Save mappings to JSON file for reference
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        println!("💾 ID mappings saved to: {}", path);
        Ok(())
    }

    /// Load mappings from JSON file
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let mappings: Self = serde_json::from_str(&json)?;
        println!("📖 ID mappings loaded from: {}", path);
        Ok(mappings)
    }
}

impl serde::Serialize for IdMappings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("IdMappings", 3)?;
        state.serialize_field("user_mappings", &self.user_mappings)?;
        state.serialize_field("room_mappings", &self.room_mappings)?;
        state.serialize_field("room_aliases", &self.room_aliases)?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for IdMappings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct IdMappingsVisitor;

        impl<'de> Visitor<'de> for IdMappingsVisitor {
            type Value = IdMappings;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct IdMappings")
            }

            fn visit_map<V>(self, mut map: V) -> Result<IdMappings, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut user_mappings = None;
                let mut room_mappings = None;
                let mut room_aliases = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "user_mappings" => {
                            if user_mappings.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_mappings"));
                            }
                            user_mappings = Some(map.next_value()?);
                        }
                        "room_mappings" => {
                            if room_mappings.is_some() {
                                return Err(serde::de::Error::duplicate_field("room_mappings"));
                            }
                            room_mappings = Some(map.next_value()?);
                        }
                        "room_aliases" => {
                            if room_aliases.is_some() {
                                return Err(serde::de::Error::duplicate_field("room_aliases"));
                            }
                            room_aliases = Some(map.next_value()?);
                        }
                        _ => {
                            let _: serde_json::Value = map.next_value()?;
                        }
                    }
                }

                Ok(IdMappings {
                    user_mappings: user_mappings.unwrap_or_default(),
                    room_mappings: room_mappings.unwrap_or_default(),
                    room_aliases: room_aliases.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct("IdMappings", &["user_mappings", "room_mappings", "room_aliases"], IdMappingsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_to_matrix_username() {
        let uuid = "0e06fba5-6474-43a0-964a-4fb934b781db";
        let username = uuid_to_matrix_username(uuid);
        assert_eq!(username, "user0e06fba56474");
    }

    #[test]
    fn test_room_name_to_matrix_alias() {
        assert_eq!(room_name_to_matrix_alias("Main Room"), "main_room");
        assert_eq!(room_name_to_matrix_alias("Dev Team"), "dev_team");
        assert_eq!(room_name_to_matrix_alias("メインルーム").len() > 0, true); // Should generate fallback
    }

    #[test]
    fn test_legacy_to_matrix_ids() {
        let legacy_user_id = "0e06fba5-6474-43a0-964a-4fb934b781db";
        let matrix_user_id = legacy_to_matrix_user_id(legacy_user_id, "nok.local");
        assert!(matrix_user_id.starts_with("@user"));
        assert!(matrix_user_id.ends_with(":nok.local"));
    }
}