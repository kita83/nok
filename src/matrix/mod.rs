pub mod client;
pub mod events;
pub mod presence;

pub use client::MatrixClient;
pub use events::NokKnockEventContent;
pub use presence::PresenceManager;

/// Matrix User ID type alias for nok
pub type NokUserId = matrix_sdk::ruma::UserId;

/// Matrix Room ID type alias for nok
pub type NokRoomId = matrix_sdk::ruma::RoomId;

/// Matrix configuration for nok client
#[derive(Debug, Clone)]
pub struct MatrixConfig {
    /// Homeserver URL (e.g., "http://localhost:8008")
    pub homeserver_url: String,
    /// Local server name (e.g., "nok.local")
    pub server_name: String,
    /// Database path for Matrix state storage
    pub state_store_path: String,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        Self {
            homeserver_url: "http://nok.local:6167".to_string(),
            server_name: "nok.local".to_string(),
            state_store_path: "matrix_state.db".to_string(),
        }
    }
}