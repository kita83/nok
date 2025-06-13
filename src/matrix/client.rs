use std::sync::Arc;
use tokio::sync::RwLock;

use matrix_sdk::{
    config::SyncSettings,
    Client, Room,
    ruma::{
        events::room::message::{RoomMessageEventContent, SyncRoomMessageEvent},
        UserId, OwnedUserId, OwnedRoomId, RoomOrAliasId,
    },
};

use crate::matrix::{MatrixConfig, NokKnockEventContent};

/// Matrix client wrapper for nok application
#[derive(Clone)]
pub struct MatrixClient {
    inner: Client,
    config: MatrixConfig,
    sync_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl std::fmt::Debug for MatrixClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MatrixClient")
            .field("config", &self.config)
            .field("sync_active", &"<sync_handle>")
            .finish()
    }
}

impl MatrixClient {
    /// Create a new Matrix client
    pub async fn new(config: MatrixConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .homeserver_url(&config.homeserver_url)
            .sqlite_store(&config.state_store_path, None)
            .build()
            .await?;

        Ok(Self {
            inner: client,
            config,
            sync_handle: Arc::new(RwLock::new(None)),
        })
    }

    /// Login with username and password
    pub async fn login(&self, username: &str, password: &str) -> Result<(), matrix_sdk::Error> {
        let user_id_str = format!("@{}:{}", username, self.config.server_name);
        let user_id = UserId::parse(&user_id_str)?;

        self.inner
            .matrix_auth()
            .login_username(&user_id, password)
            .send()
            .await?;

        println!("Logged in as: {}", self.inner.user_id().unwrap());
        Ok(())
    }

    /// Start syncing with the homeserver
    pub async fn start_sync(&self) -> Result<(), matrix_sdk::Error> {
        // Check if sync is already running
        if self.sync_handle.read().await.is_some() {
            return Ok(()); // Early return to prevent multiple sync threads
        }

        let client = self.inner.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = client.sync(SyncSettings::default()).await {
                eprintln!("Sync error: {}", e);
            }
        });

        *self.sync_handle.write().await = Some(handle);
        Ok(())
    }

    /// Stop syncing
    pub async fn stop_sync(&self) {
        if let Some(handle) = self.sync_handle.write().await.take() {
            handle.abort();
        }
    }

    /// Join a room by room ID or alias
    pub async fn join_room(&self, room_id_or_alias: &str) -> Result<Room, matrix_sdk::Error> {
        let room_id = RoomOrAliasId::parse(room_id_or_alias)?;
        let room = self.inner.join_room_by_id_or_alias(&room_id, &[]).await?;
        Ok(room)
    }

    /// Leave a room
    pub async fn leave_room(&self, room_id: &OwnedRoomId) -> Result<(), matrix_sdk::Error> {
        if let Some(room) = self.inner.get_room(room_id) {
            room.leave().await?;
        }
        Ok(())
    }

    /// Send a text message to a room
    pub async fn send_message(&self, room_id: &OwnedRoomId, content: &str) -> Result<(), matrix_sdk::Error> {
        if let Some(room) = self.inner.get_room(room_id) {
            let content = RoomMessageEventContent::text_plain(content);
            room.send(content).await?;
        }
        Ok(())
    }

    /// Send a knock event to a user
    pub async fn send_knock(&self, room_id: &OwnedRoomId, target_user: &OwnedUserId) -> Result<(), matrix_sdk::Error> {
        if let Some(room) = self.inner.get_room(room_id) {
            let _knock_content = NokKnockEventContent {
                target_user: target_user.clone(),
                timestamp: chrono::Utc::now().timestamp_millis(),
            };

            // Send as custom event - this will be implemented in events.rs
            // For now, send as a regular message with special format
            let message = format!("🚪 *knock knock* for {}", target_user);
            let content = RoomMessageEventContent::text_plain(&message);
            room.send(content).await?;
        }
        Ok(())
    }


    /// Get all joined rooms
    pub fn rooms(&self) -> Vec<Room> {
        self.inner.rooms()
    }

    /// Get a specific room by ID
    pub fn get_room(&self, room_id: &OwnedRoomId) -> Option<Room> {
        self.inner.get_room(room_id)
    }

    /// Add simple event handler for incoming messages
    pub fn add_simple_message_handler(&self) {
        self.inner.add_event_handler(|event: SyncRoomMessageEvent, room: Room| async move {
            let room_name = room.display_name().await
                .map(|name| name.to_string())
                .unwrap_or_else(|_| "Unknown Room".to_string());
            println!("Received message in room {}: {:?}", room_name, event);
        });
    }

    /// Get the current user ID if logged in
    pub fn user_id(&self) -> Option<OwnedUserId> {
        self.inner.user_id().map(|user_id| user_id.to_owned())
    }

    /// Get the underlying Matrix SDK client
    pub fn inner(&self) -> &Client {
        &self.inner
    }
}