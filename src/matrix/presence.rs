use matrix_sdk::{
    Client,
    ruma::{
        api::client::presence::{get_presence, set_presence},
        presence::PresenceState,
        OwnedUserId, UserId,
    },
};
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::app::user::UserStatus;

/// Manages presence information for Matrix users
pub struct PresenceManager {
    client: Client,
    cached_presence: RwLock<HashMap<OwnedUserId, UserPresence>>,
}

/// User presence information
#[derive(Debug, Clone)]
pub struct UserPresence {
    pub state: PresenceState,
    pub status_msg: Option<String>,
    pub last_active_ago: Option<u64>,
}

impl PresenceManager {
    /// Create a new presence manager
    pub fn new(client: Client) -> Self {
        Self {
            client,
            cached_presence: RwLock::new(HashMap::new()),
        }
    }

    /// Set own presence state
    pub async fn set_presence(&self, status: UserStatus, status_msg: Option<String>) -> Result<(), matrix_sdk::Error> {
        let presence_state = match status {
            UserStatus::Online => PresenceState::Online,
            UserStatus::Away => PresenceState::Unavailable,
            UserStatus::Busy => PresenceState::Unavailable,
            UserStatus::Offline => PresenceState::Offline,
        };

        // Get current user ID
        let user_id = self.client.user_id()
            .ok_or_else(|| matrix_sdk::Error::AuthenticationRequired)?
            .to_owned();

        let mut request = set_presence::v3::Request::new(user_id, presence_state);
        if let Some(msg) = status_msg {
            request.status_msg = Some(msg);
        }

        self.client.send(request).await?;
        Ok(())
    }

    /// Get presence for a specific user
    pub async fn get_presence(&self, user_id: &UserId) -> Result<UserPresence, matrix_sdk::Error> {
        // Check cache first
        {
            let cache = self.cached_presence.read().await;
            if let Some(presence) = cache.get(user_id) {
                return Ok(presence.clone());
            }
        }

        // Fetch from server
        let request = get_presence::v3::Request::new(user_id.to_owned());
        let response = self.client.send(request).await?;

        let presence = UserPresence {
            state: response.presence,
            status_msg: response.status_msg,
            last_active_ago: response.last_active_ago.map(|d| d.as_millis() as u64),
        };

        // Update cache
        {
            let mut cache = self.cached_presence.write().await;
            cache.insert(user_id.to_owned(), presence.clone());
        }

        Ok(presence)
    }

    /// Convert Matrix PresenceState to UserStatus
    pub fn presence_to_user_status(presence: &PresenceState) -> UserStatus {
        match presence {
            PresenceState::Online => UserStatus::Online,
            PresenceState::Offline => UserStatus::Offline,
            PresenceState::Unavailable => UserStatus::Away,
            _ => UserStatus::Offline,
        }
    }

    /// Convert UserStatus to Matrix PresenceState
    pub fn user_status_to_presence(status: &UserStatus) -> PresenceState {
        match status {
            UserStatus::Online => PresenceState::Online,
            UserStatus::Away => PresenceState::Unavailable,
            UserStatus::Busy => PresenceState::Unavailable,
            UserStatus::Offline => PresenceState::Offline,
        }
    }

    /// Update cached presence for a user
    pub async fn update_cached_presence(&self, user_id: OwnedUserId, presence: UserPresence) {
        let mut cache = self.cached_presence.write().await;
        cache.insert(user_id, presence);
    }

    /// Get all cached presence data
    pub async fn get_all_cached_presence(&self) -> HashMap<OwnedUserId, UserPresence> {
        self.cached_presence.read().await.clone()
    }

    /// Clear presence cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cached_presence.write().await;
        cache.clear();
    }
}