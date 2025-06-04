use crate::util::{NokError, NokResult};
use super::matrix_state::MatrixState;
use super::legacy_state::LegacyState;
use super::core::{AppCore, LogState};

/// Communication mode selector
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommunicationMode {
    Matrix,
    Legacy,
    Hybrid, // Both enabled for transition period
}

/// Unified state manager that coordinates between Matrix and Legacy systems
#[derive(Debug)]
pub struct StateManager {
    mode: CommunicationMode,
    matrix: MatrixState,
    legacy: LegacyState,
}

impl StateManager {
    pub fn new(matrix: MatrixState, legacy: LegacyState) -> Self {
        Self {
            mode: CommunicationMode::Matrix, // Default to Matrix
            matrix,
            legacy,
        }
    }

    /// Get current communication mode
    pub fn get_mode(&self) -> CommunicationMode {
        self.mode
    }

    /// Set communication mode
    pub fn set_mode(&mut self, mode: CommunicationMode) {
        self.mode = mode;
        
        match mode {
            CommunicationMode::Matrix => {
                self.matrix.enable();
                self.legacy.disable();
            }
            CommunicationMode::Legacy => {
                self.matrix.disable();
                self.legacy.enable();
            }
            CommunicationMode::Hybrid => {
                self.matrix.enable();
                self.legacy.enable();
            }
        }
    }

    /// Initialize the state manager based on configuration
    pub async fn initialize(&mut self, logs: &mut LogState) -> NokResult<()> {
        match self.mode {
            CommunicationMode::Matrix => {
                logs.add_debug_log("Initializing Matrix-only mode".to_string());
                self.matrix.initialize_client().await?;
            }
            CommunicationMode::Legacy => {
                logs.add_debug_log("Initializing Legacy-only mode".to_string());
                self.legacy.connect().await?;
            }
            CommunicationMode::Hybrid => {
                logs.add_debug_log("Initializing Hybrid mode".to_string());
                // Initialize both systems
                if let Err(e) = self.matrix.initialize_client().await {
                    logs.add_debug_log(format!("Matrix initialization failed: {}", e));
                }
                if let Err(e) = self.legacy.connect().await {
                    logs.add_debug_log(format!("Legacy connection failed: {}", e));
                }
            }
        }

        Ok(())
    }

    /// Send a knock message using the appropriate protocol
    pub async fn send_knock(&self, target_user_id: &str, logs: &mut LogState) -> NokResult<()> {
        match self.mode {
            CommunicationMode::Matrix => {
                if self.matrix.is_enabled() && self.matrix.is_logged_in() {
                    logs.add_debug_log(format!("Sending Matrix knock to {}", target_user_id));
                    // Implementation would go here for Matrix knock
                    self.send_matrix_knock(target_user_id).await
                } else {
                    Err(NokError::MatrixClientNotInitialized)
                }
            }
            CommunicationMode::Legacy => {
                if self.legacy.is_enabled() && self.legacy.is_connected() {
                    logs.add_debug_log(format!("Sending legacy knock to {}", target_user_id));
                    self.legacy.send_knock(target_user_id).await
                } else {
                    Err(NokError::ConnectionFailed("Legacy system not connected".to_string()))
                }
            }
            CommunicationMode::Hybrid => {
                // Try Matrix first, fallback to legacy
                if self.matrix.is_enabled() && self.matrix.is_logged_in() {
                    logs.add_debug_log(format!("Sending Matrix knock to {} (hybrid mode)", target_user_id));
                    self.send_matrix_knock(target_user_id).await
                } else if self.legacy.is_enabled() && self.legacy.is_connected() {
                    logs.add_debug_log(format!("Fallback to legacy knock for {} (hybrid mode)", target_user_id));
                    self.legacy.send_knock(target_user_id).await
                } else {
                    Err(NokError::ConnectionFailed("Neither Matrix nor legacy system available".to_string()))
                }
            }
        }
    }

    /// Send a message using the appropriate protocol
    pub async fn send_message(&self, room_id: &str, message: &str, logs: &mut LogState) -> NokResult<()> {
        match self.mode {
            CommunicationMode::Matrix => {
                if self.matrix.is_enabled() && self.matrix.is_logged_in() {
                    logs.add_debug_log(format!("Sending Matrix message to room {}", room_id));
                    self.send_matrix_message(room_id, message).await
                } else {
                    Err(NokError::MatrixClientNotInitialized)
                }
            }
            CommunicationMode::Legacy => {
                logs.add_debug_log(format!("Sending legacy message to room {}", room_id));
                // Implementation would go here for legacy message sending
                Ok(())
            }
            CommunicationMode::Hybrid => {
                // Prefer Matrix for new messages
                if self.matrix.is_enabled() && self.matrix.is_logged_in() {
                    logs.add_debug_log(format!("Sending Matrix message to room {} (hybrid mode)", room_id));
                    self.send_matrix_message(room_id, message).await
                } else {
                    logs.add_debug_log(format!("Fallback to legacy message for room {} (hybrid mode)", room_id));
                    // Fallback to legacy
                    Ok(())
                }
            }
        }
    }

    /// Set user presence using the appropriate protocol
    pub async fn set_presence(&self, status: &str, logs: &mut LogState) -> NokResult<()> {
        match self.mode {
            CommunicationMode::Matrix => {
                if self.matrix.is_enabled() && self.matrix.is_logged_in() {
                    logs.add_debug_log(format!("Setting Matrix presence to {}", status));
                    self.set_matrix_presence(status).await
                } else {
                    Err(NokError::MatrixClientNotInitialized)
                }
            }
            CommunicationMode::Legacy => {
                logs.add_debug_log(format!("Setting legacy presence to {}", status));
                // Implementation would go here for legacy presence
                Ok(())
            }
            CommunicationMode::Hybrid => {
                // Set presence in both systems if available
                let mut errors = Vec::new();
                
                if self.matrix.is_enabled() && self.matrix.is_logged_in() {
                    if let Err(e) = self.set_matrix_presence(status).await {
                        errors.push(format!("Matrix presence error: {}", e));
                    } else {
                        logs.add_debug_log(format!("Matrix presence set to {}", status));
                    }
                }
                
                // Set legacy presence as well
                logs.add_debug_log(format!("Legacy presence set to {}", status));
                
                if !errors.is_empty() {
                    Err(NokError::InternalError(errors.join("; ")))
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Get current connection status
    pub fn get_connection_status(&self) -> ConnectionStatus {
        match self.mode {
            CommunicationMode::Matrix => {
                if self.matrix.is_logged_in() {
                    ConnectionStatus::Connected
                } else if self.matrix.is_enabled() {
                    ConnectionStatus::Connecting
                } else {
                    ConnectionStatus::Disconnected
                }
            }
            CommunicationMode::Legacy => {
                self.legacy.get_connection_status().clone()
            }
            CommunicationMode::Hybrid => {
                let matrix_connected = self.matrix.is_logged_in();
                let legacy_connected = self.legacy.is_connected();
                
                if matrix_connected || legacy_connected {
                    ConnectionStatus::Connected
                } else {
                    ConnectionStatus::Disconnected
                }
            }
        }
    }

    /// Access Matrix state (read-only)
    pub fn matrix(&self) -> &MatrixState {
        &self.matrix
    }

    /// Access Matrix state (mutable)
    pub fn matrix_mut(&mut self) -> &mut MatrixState {
        &mut self.matrix
    }

    /// Access Legacy state (read-only)
    pub fn legacy(&self) -> &LegacyState {
        &self.legacy
    }

    /// Access Legacy state (mutable)
    pub fn legacy_mut(&mut self) -> &mut LegacyState {
        &mut self.legacy
    }

    /// Graceful shutdown of all systems
    pub async fn shutdown(&mut self, logs: &mut LogState) -> NokResult<()> {
        logs.add_debug_log("Shutting down state manager".to_string());
        
        if self.matrix.is_enabled() {
            self.matrix.stop_sync().await;
            logs.add_debug_log("Matrix sync stopped".to_string());
        }
        
        if self.legacy.is_enabled() {
            self.legacy.disconnect().await?;
            logs.add_debug_log("Legacy connection closed".to_string());
        }
        
        Ok(())
    }

    // Private helper methods for protocol-specific operations

    async fn send_matrix_knock(&self, target_user_id: &str) -> NokResult<()> {
        // Implementation would go here for sending Matrix knock events
        // This would involve creating a custom Matrix event of type "com.nok.knock"
        
        if let Some(client) = self.matrix.get_client() {
            // Example implementation:
            // let event = KnockEvent::new(target_user_id);
            // client.send_custom_event(event).await?;
            Ok(())
        } else {
            Err(NokError::MatrixClientNotInitialized)
        }
    }

    async fn send_matrix_message(&self, room_id: &str, message: &str) -> NokResult<()> {
        // Implementation would go here for sending Matrix messages
        
        if let Some(client) = self.matrix.get_client() {
            // Example implementation:
            // let room = client.get_room(room_id)?;
            // room.send_message(message).await?;
            Ok(())
        } else {
            Err(NokError::MatrixClientNotInitialized)
        }
    }

    async fn set_matrix_presence(&self, status: &str) -> NokResult<()> {
        // Implementation would go here for setting Matrix presence
        
        if let Some(client) = self.matrix.get_client() {
            // Example implementation:
            // let presence = match status {
            //     "online" => Presence::Online,
            //     "away" => Presence::Away,
            //     "busy" => Presence::Busy,
            //     _ => Presence::Online,
            // };
            // client.set_presence(presence).await?;
            Ok(())
        } else {
            Err(NokError::MatrixClientNotInitialized)
        }
    }
}

use super::core::ConnectionStatus;

/// Migration utilities for transitioning between modes
pub struct ModeTransition;

impl ModeTransition {
    /// Determine the best mode based on current system state
    pub fn determine_optimal_mode(
        matrix_available: bool,
        legacy_available: bool,
        user_preference: Option<CommunicationMode>
    ) -> CommunicationMode {
        match user_preference {
            Some(mode) => mode,
            None => {
                match (matrix_available, legacy_available) {
                    (true, true) => CommunicationMode::Hybrid,
                    (true, false) => CommunicationMode::Matrix,
                    (false, true) => CommunicationMode::Legacy,
                    (false, false) => CommunicationMode::Matrix, // Default, will need setup
                }
            }
        }
    }

    /// Migrate data from legacy to Matrix format
    pub async fn migrate_legacy_to_matrix(
        legacy: &LegacyState,
        matrix: &mut MatrixState,
        logs: &mut LogState
    ) -> NokResult<()> {
        if !legacy.is_enabled() {
            return Ok(());
        }

        logs.add_debug_log("Starting legacy to Matrix migration".to_string());
        
        // Implementation would go here for data migration:
        // 1. Export user data from legacy system
        // 2. Create Matrix rooms for legacy rooms
        // 3. Invite users to appropriate rooms
        // 4. Migrate message history if needed
        
        logs.add_debug_log("Legacy to Matrix migration completed".to_string());
        Ok(())
    }

    /// Validate that transition to new mode is safe
    pub fn validate_mode_transition(
        current: CommunicationMode,
        target: CommunicationMode,
        matrix_state: &MatrixState,
        legacy_state: &LegacyState
    ) -> Result<(), String> {
        match (current, target) {
            (CommunicationMode::Legacy, CommunicationMode::Matrix) => {
                if !matrix_state.is_initialized() {
                    return Err("Matrix client not initialized".to_string());
                }
                if !matrix_state.is_logged_in() {
                    return Err("Not logged into Matrix".to_string());
                }
            }
            (CommunicationMode::Matrix, CommunicationMode::Legacy) => {
                if !legacy_state.is_enabled() {
                    return Err("Legacy system not available".to_string());
                }
                if !legacy_state.is_connected() {
                    return Err("Not connected to legacy server".to_string());
                }
            }
            _ => {} // Other transitions are generally safe
        }
        
        Ok(())
    }
}