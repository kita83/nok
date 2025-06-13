use crossterm::event::{KeyEvent, KeyCode};
use crate::util::{NokError, NokResult};
use crate::api::{ApiClient, WebSocketClient};
use crate::matrix::MatrixConfig;

use super::core::{AppCore, UiState, DataState, LogState, NetworkState, PaneIdentifier};
use super::matrix_state::{MatrixState, LoginField};
use super::legacy_state::LegacyState;
use super::state_manager::{StateManager, CommunicationMode};
use super::config::Config;
use super::unified_config::UnifiedConfig;
use super::user::User;

/// New modular App structure
/// Separates concerns into focused, manageable components
pub struct App {
    // Core application state
    pub core: AppCore,
    
    // UI-specific state
    pub ui: UiState,
    
    // Data and content
    pub data: DataState,
    
    // Logging and debugging
    pub logs: LogState,
    
    // Network state
    pub network: NetworkState,
    
    // Unified state manager for Matrix/Legacy coordination
    pub state_manager: StateManager,
    
    // Unified configuration
    pub config: UnifiedConfig,
}

impl App {
    pub fn new() -> Self {
        // Load unified configuration
        let mut config = UnifiedConfig::load();
        
        // Apply environment variable overrides
        config.apply_env_overrides();
        
        // Validate configuration
        if let Err(errors) = config.validate() {
            eprintln!("Configuration validation errors:");
            for error in errors {
                eprintln!("  - {}", error);
            }
            eprintln!("Continuing with current configuration...");
        }
        
        // Create current user from config
        let mut current_user = User::new(config.user.username.clone());
        current_user.id = Some(config.user.user_id.clone());
        
        // Initialize API clients using legacy config
        let api_client = ApiClient::new();
        let websocket_client = WebSocketClient::new();
        
        // Create Matrix and Legacy states
        let matrix_config = config.to_matrix_config();
        let matrix_state = MatrixState::new(matrix_config);
        let legacy_state = LegacyState::new(api_client, websocket_client);
        
        // Create unified state manager with configured mode
        let mut state_manager = StateManager::new(matrix_state, legacy_state);
        state_manager.set_mode(config.app.communication_mode);
        
        // Create legacy config for backward compatibility
        let legacy_config = Config::from_unified(&config);
        
        let mut app = Self {
            core: AppCore::new(legacy_config),
            ui: UiState::new(),
            data: DataState::new(current_user),
            logs: LogState::new(),
            network: NetworkState::new(),
            state_manager,
            config: config.clone(),
        };
        
        // Configure logging based on config
        app.logs.max_debug_logs = config.logging.max_log_entries;
        
        // Log initialization
        app.logs.add_debug_log("Application initialized with unified configuration".to_string());
        app.logs.add_debug_log(format!("Config loaded from: {:?}", UnifiedConfig::get_config_path()));
        app.logs.add_debug_log(format!("Communication mode: {:?}", config.app.communication_mode));
        
        if config.logging.enable_debug {
            app.logs.add_debug_log("Debug logging enabled".to_string());
            app.logs.add_debug_log(config.summary());
        }
        
        app
    }

    /// Initialize the application (async setup)
    pub async fn initialize(&mut self) -> NokResult<()> {
        // Determine communication mode
        let mode = self.determine_communication_mode();
        self.state_manager.set_mode(mode);
        
        // Initialize the state manager
        self.state_manager.initialize(&mut self.logs).await?;
        
        // Update network state based on connection status
        self.update_network_state();
        
        Ok(())
    }

    /// Determine which communication mode to use
    fn determine_communication_mode(&self) -> CommunicationMode {
        // Check environment variable first
        if let Ok(mode_str) = std::env::var("NOK_COMMUNICATION_MODE") {
            match mode_str.to_lowercase().as_str() {
                "matrix" => return CommunicationMode::Matrix,
                "legacy" => return CommunicationMode::Legacy,
                "hybrid" => return CommunicationMode::Hybrid,
                _ => {}
            }
        }
        
        // Default to Matrix mode
        CommunicationMode::Matrix
    }

    /// Update network state based on current connections
    fn update_network_state(&mut self) {
        let status = self.state_manager.get_connection_status();
        self.network.connection_status = status;
    }

    /// Determine which mode to use based on configuration and availability
    fn should_use_matrix_mode(&self) -> bool {
        // For now, default to Matrix mode
        // This could be made configurable via environment variable or config file
        std::env::var("NOK_USE_MATRIX").unwrap_or_else(|_| "true".to_string()) == "true"
    }

    /// Handle keyboard input
    pub async fn handle_key(&mut self, key: KeyEvent) -> NokResult<()> {
        use super::state::AppState;

        match self.core.state {
            AppState::Login => self.handle_login_key(key).await,
            AppState::Normal => self.handle_normal_key(key).await,
            AppState::Input => self.handle_input_key(key).await,
            AppState::Settings => self.handle_settings_key(key).await,
        }
    }

    /// Handle login screen input
    async fn handle_login_key(&mut self, key: KeyEvent) -> NokResult<()> {
        let login_state = &mut self.state_manager.matrix_mut().login;
        
        match key.code {
            KeyCode::Char(c) => {
                match login_state.field_focus {
                    LoginField::Username => {
                        login_state.username.push(c);
                    }
                    LoginField::Password => {
                        login_state.password.push(c);
                    }
                }
            }
            KeyCode::Backspace => {
                match login_state.field_focus {
                    LoginField::Username => {
                        login_state.username.pop();
                    }
                    LoginField::Password => {
                        login_state.password.pop();
                    }
                }
            }
            KeyCode::Tab => {
                login_state.next_field();
            }
            KeyCode::Enter => {
                if login_state.can_submit() {
                    self.attempt_login().await?;
                }
            }
            KeyCode::Esc => {
                self.core.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle normal mode input
    async fn handle_normal_key(&mut self, key: KeyEvent) -> NokResult<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.core.should_quit = true;
            }
            KeyCode::Char('s') => {
                self.core.state = super::state::AppState::Settings;
            }
            KeyCode::Char('i') => {
                self.core.state = super::state::AppState::Input;
            }
            KeyCode::Char('k') => {
                self.send_knock().await?;
            }
            KeyCode::Up => {
                self.navigate_up();
            }
            KeyCode::Down => {
                self.navigate_down();
            }
            KeyCode::Tab => {
                self.cycle_focus();
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle input mode
    async fn handle_input_key(&mut self, key: KeyEvent) -> NokResult<()> {
        match key.code {
            KeyCode::Char(c) => {
                self.ui.input.push(c);
            }
            KeyCode::Backspace => {
                self.ui.input.pop();
            }
            KeyCode::Enter => {
                self.process_input().await?;
                self.core.state = super::state::AppState::Normal;
            }
            KeyCode::Esc => {
                self.ui.clear_input();
                self.core.state = super::state::AppState::Normal;
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle settings mode
    async fn handle_settings_key(&mut self, key: KeyEvent) -> NokResult<()> {
        match key.code {
            KeyCode::Esc => {
                self.core.state = super::state::AppState::Normal;
            }
            KeyCode::Char('m') => {
                self.toggle_matrix_mode().await?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Attempt to login using current credentials
    async fn attempt_login(&mut self) -> NokResult<()> {
        let username = self.state_manager.matrix().login.username.clone();
        let password = self.state_manager.matrix().login.password.clone();

        match self.state_manager.matrix_mut().login(&username, &password).await {
            Ok(()) => {
                self.logs.add_debug_log(format!("Logged in as {}", username));
                self.core.state = super::state::AppState::Normal;
                
                // Start Matrix sync
                if let Err(e) = self.state_manager.matrix().start_sync().await {
                    self.logs.add_debug_log(format!("Failed to start sync: {}", e));
                } else {
                    self.logs.add_debug_log("Matrix sync started successfully".to_string());
                    
                    // Wait longer for initial sync and then load rooms
                    self.logs.add_debug_log("Waiting for Matrix sync to stabilize...".to_string());
                    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
                    
                    self.logs.add_debug_log("Attempting to sync rooms from Matrix...".to_string());
                    if let Err(e) = self.sync_rooms_from_matrix().await {
                        self.logs.add_debug_log(format!("Failed to sync rooms: {}", e));
                    } else {
                        self.logs.add_debug_log("Room sync completed successfully".to_string());
                    }
                }
                
                // Update network state
                self.update_network_state();
            }
            Err(e) => {
                let error_msg = e.user_message();
                self.state_manager.matrix_mut().login.set_error(error_msg.clone());
                self.logs.add_debug_log(format!("Login failed: {}", error_msg));
            }
        }

        Ok(())
    }

    /// Send a knock to the selected user
    async fn send_knock(&mut self) -> NokResult<()> {
        if let Some(selected_idx) = self.ui.selected_user {
            if let Some(user) = self.data.users.get(selected_idx) {
                if let Some(user_id) = &user.id {
                    self.state_manager.send_knock(user_id, &mut self.logs).await?;
                    self.core.set_notification(format!("Knocked on {}", user.name));
                } else {
                    self.core.set_error("User ID not available".to_string());
                }
            }
        } else {
            self.core.set_error("No user selected".to_string());
        }
        Ok(())
    }

    /// Process input command
    async fn process_input(&mut self) -> NokResult<()> {
        let input = self.ui.input.trim().to_string();
        
        if input.starts_with("/") {
            // Handle commands
            self.process_command(&input).await?;
        } else if input.starts_with("nok @") {
            // Handle knock command
            self.process_knock_command(&input).await?;
        } else {
            // Regular message
            self.send_message(&input).await?;
        }

        self.ui.clear_input();
        Ok(())
    }

    /// Process slash commands
    async fn process_command(&mut self, command: &str) -> NokResult<()> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        match parts.get(0).copied() {
            Some("/help") => {
                self.show_help();
            }
            Some("/status") => {
                if let Some(status) = parts.get(1) {
                    self.set_status(status).await?;
                }
            }
            Some("/join") => {
                if let Some(room) = parts.get(1) {
                    self.join_room(room).await?;
                }
            }
            _ => {
                self.core.set_error("Unknown command".to_string());
            }
        }
        
        Ok(())
    }

    /// Process knock command
    async fn process_knock_command(&mut self, command: &str) -> NokResult<()> {
        if let Some(username) = command.strip_prefix("nok @") {
            let username = username.trim();
            
            // Find user and send knock
            if let Some(_user) = self.data.users.iter().find(|u| u.name == username) {
                self.logs.add_debug_log(format!("Knocking {}", username));
                // Implementation would depend on Matrix vs legacy mode
            } else {
                self.core.set_error(format!("User '{}' not found", username));
            }
        }
        
        Ok(())
    }

    /// Send a regular message
    async fn send_message(&mut self, _message: &str) -> NokResult<()> {
        if let Some(room) = self.data.get_current_room() {
            self.logs.add_debug_log(format!("Sending message to room: {}", room.name));
            // Implementation would depend on Matrix vs legacy mode
        }
        
        Ok(())
    }

    /// Toggle between communication modes
    async fn toggle_matrix_mode(&mut self) -> NokResult<()> {
        let current_mode = self.state_manager.get_mode();
        let new_mode = match current_mode {
            CommunicationMode::Matrix => CommunicationMode::Legacy,
            CommunicationMode::Legacy => CommunicationMode::Hybrid,
            CommunicationMode::Hybrid => CommunicationMode::Matrix,
        };
        
        self.logs.add_debug_log(format!("Switching from {:?} to {:?} mode", current_mode, new_mode));
        self.state_manager.set_mode(new_mode);
        self.state_manager.initialize(&mut self.logs).await?;
        self.update_network_state();
        
        Ok(())
    }

    /// Set user status
    async fn set_status(&mut self, status: &str) -> NokResult<()> {
        self.state_manager.set_presence(status, &mut self.logs).await?;
        self.core.set_notification(format!("Status set to: {}", status));
        Ok(())
    }

    /// Join a room
    async fn join_room(&mut self, room_name: &str) -> NokResult<()> {
        self.logs.add_debug_log(format!("Joining room: {}", room_name));
        // Implementation would join room in Matrix or legacy system
        Ok(())
    }

    /// Show help information
    fn show_help(&mut self) {
        let help_text = r#"
Commands:
  /help - Show this help
  /status <status> - Set your status
  /join <room> - Join a room
  nok @username - Send knock to user
  
Keys:
  q - Quit
  s - Settings
  i - Input mode
  k - Send knock to selected user
  Tab - Cycle focus
        "#;
        
        self.core.set_notification(help_text.to_string());
    }

    /// Navigation helpers
    fn navigate_up(&mut self) {
        match self.core.focused_pane {
            PaneIdentifier::Users => {
                if let Some(selected) = self.ui.selected_user {
                    if selected > 0 {
                        self.ui.selected_user = Some(selected - 1);
                    }
                } else if !self.data.users.is_empty() {
                    self.ui.selected_user = Some(self.data.users.len() - 1);
                }
            }
            PaneIdentifier::Rooms => {
                if self.ui.selected_room_idx > 0 {
                    self.ui.selected_room_idx -= 1;
                }
            }
            _ => {}
        }
    }

    fn navigate_down(&mut self) {
        match self.core.focused_pane {
            PaneIdentifier::Users => {
                if let Some(selected) = self.ui.selected_user {
                    if selected < self.data.users.len() - 1 {
                        self.ui.selected_user = Some(selected + 1);
                    }
                } else if !self.data.users.is_empty() {
                    self.ui.selected_user = Some(0);
                }
            }
            PaneIdentifier::Rooms => {
                if self.ui.selected_room_idx < self.data.rooms.len().saturating_sub(1) {
                    self.ui.selected_room_idx += 1;
                }
            }
            _ => {}
        }
    }

    fn cycle_focus(&mut self) {
        self.core.focused_pane = match self.core.focused_pane {
            PaneIdentifier::Rooms => PaneIdentifier::Users,
            PaneIdentifier::Users => PaneIdentifier::Messages,
            PaneIdentifier::Messages => PaneIdentifier::AsciiArt,
            PaneIdentifier::AsciiArt => PaneIdentifier::Rooms,
        };
    }

    /// Public accessors for UI components
    pub fn should_quit(&self) -> bool {
        self.core.should_quit()
    }

    pub fn get_error(&self) -> Option<&String> {
        self.core.error.as_ref()
    }

    pub fn get_notification(&self) -> Option<&String> {
        self.core.notification.as_ref()
    }

    /// Cleanup when app shuts down
    pub async fn shutdown(&mut self) -> NokResult<()> {
        self.logs.add_debug_log("Application shutting down".to_string());
        
        // Shutdown state manager (handles both Matrix and legacy cleanup)
        self.state_manager.shutdown(&mut self.logs).await?;
        
        // Update unified configuration with current state
        self.config.app.communication_mode = self.state_manager.get_mode();
        self.config.user.username = self.data.current_user.name.clone();
        
        // Save unified configuration
        if let Err(e) = self.config.save() {
            self.logs.add_debug_log(format!("Failed to save configuration: {}", e));
        } else {
            self.logs.add_debug_log("Configuration saved successfully".to_string());
        }
        
        // Also save legacy config for backward compatibility
        self.core.config.save();
        
        self.logs.add_debug_log("Application shutdown complete".to_string());
        Ok(())
    }

    /// Sync Matrix rooms to UI state
    async fn sync_rooms_from_matrix(&mut self) -> NokResult<()> {
        self.logs.add_debug_log("Starting room sync from Matrix client...".to_string());
        
        if let Some(matrix_client) = self.state_manager.matrix().get_client() {
            self.logs.add_debug_log("Matrix client found, retrieving rooms...".to_string());
            let matrix_rooms = matrix_client.rooms();
            self.logs.add_debug_log(format!("Found {} Matrix rooms from client", matrix_rooms.len()));
            
            if matrix_rooms.is_empty() {
                self.logs.add_debug_log("No Matrix rooms found - user may not have joined any rooms yet".to_string());
                self.logs.add_debug_log("Try creating a room with: cargo run --bin create_test_room".to_string());
                return Ok(());
            }
            
            for (i, matrix_room) in matrix_rooms.iter().enumerate() {
                let room_id = matrix_room.room_id().to_string();
                self.logs.add_debug_log(format!("Processing room {} ({}): {}", i+1, room_id, room_id));
                
                // Get room name
                let room_name = match matrix_room.display_name().await {
                    Ok(name) => {
                        self.logs.add_debug_log(format!("Room display name: {}", name));
                        name.to_string()
                    },
                    Err(e) => {
                        self.logs.add_debug_log(format!("Failed to get display name: {}, using room ID", e));
                        room_id.clone()
                    }
                };
                
                // Create nok Room from Matrix room
                let mut room = super::room::Room::from_matrix_room(room_id.clone(), room_name.clone());
                
                // Set additional room properties
                // Note: is_encrypted() might not be available in all matrix-sdk versions
                // room.is_encrypted = matrix_room.is_encrypted().await.unwrap_or(false);
                room.member_count = matrix_room.joined_members_count() as usize;
                self.logs.add_debug_log(format!("Room member count: {}", room.member_count));
                
                // Add room if it doesn't already exist
                if !self.data.rooms.iter().any(|r| r.matrix_id.as_ref() == Some(&room_id)) {
                    self.logs.add_debug_log(format!("Adding new room to UI: '{}'", room.name));
                    self.data.add_room(room);
                } else {
                    self.logs.add_debug_log(format!("Room already exists in UI: '{}'", room.name));
                }
            }
            
            self.logs.add_debug_log(format!("Room sync complete - Total UI rooms: {}", self.data.rooms.len()));
            
            // Set first room as current if none is selected
            if self.data.get_current_room().is_none() && !self.data.rooms.is_empty() {
                self.data.set_current_room_idx(0);
                self.logs.add_debug_log(format!("Set '{}' as current room", self.data.rooms[0].name));
            }
        } else {
            self.logs.add_debug_log("ERROR: Matrix client not found!".to_string());
            return Err(crate::util::NokError::MatrixClientNotInitialized);
        }
        
        Ok(())
    }
}