use matrix_sdk::ruma::{
    events::macros::EventContent,
    OwnedUserId,
};
use serde::{Deserialize, Serialize};

/// Custom event content for nok knock functionality
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "com.nok.knock", kind = MessageLike)]
pub struct NokKnockEventContent {
    /// The user being knocked
    pub target_user: OwnedUserId,
    /// Timestamp when the knock was sent
    pub timestamp: i64,
}

/// Helper to create a knock event content
impl NokKnockEventContent {
    pub fn new(target_user: OwnedUserId) -> Self {
        Self {
            target_user,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }

    /// Get the target user ID
    pub fn target_user(&self) -> &OwnedUserId {
        &self.target_user
    }

    /// Get the timestamp
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    /// Convert to a human-readable string
    pub fn to_display_string(&self, sender: &str) -> String {
        let time = chrono::DateTime::from_timestamp_millis(self.timestamp)
            .map(|dt| dt.format("%H:%M").to_string())
            .unwrap_or_else(|| "??:??".to_string());

        format!("🚪 {} knocked on {}'s door at {}", sender, self.target_user, time)
    }
}