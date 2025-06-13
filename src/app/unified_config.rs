use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use crate::util::{NokError, NokResult};
use crate::matrix::MatrixConfig;
use super::state_manager::CommunicationMode;

/// Unified configuration that encompasses all settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConfig {
    /// Application metadata
    pub app: AppConfig,
    
    /// User information
    pub user: UserConfig,
    
    /// Matrix-specific settings
    pub matrix: MatrixConfigExt,
    
    /// Legacy WebSocket settings
    pub legacy: LegacyConfig,
    
    /// UI preferences
    pub ui: UiConfig,
    
    /// Logging preferences
    pub logging: LoggingConfig,
    
    /// Network settings
    pub network: NetworkConfig,
}

/// Core application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub communication_mode: CommunicationMode,
    pub auto_start_mode: bool,
    pub enable_sounds: bool,
    pub enable_notifications: bool,
}

/// User-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub user_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub default_status: String,
}

/// Extended Matrix configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixConfigExt {
    pub homeserver_url: String,
    pub server_name: String,
    pub device_name: String,
    pub store_path: String,
    pub auto_login: bool,
    pub enable_encryption: bool,
    pub sync_timeout_ms: u64,
    pub presence_enabled: bool,
}

/// Legacy WebSocket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyConfig {
    pub server_url: String,
    pub api_endpoint: String,
    pub websocket_endpoint: String,
    pub timeout_ms: u64,
    pub reconnect_attempts: u32,
    pub enable_fallback: bool,
}

/// UI preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub default_tab: String,
    pub show_ascii_art: bool,
    pub compact_mode: bool,
    pub show_timestamps: bool,
    pub max_message_history: usize,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub enable_debug: bool,
    pub max_log_entries: usize,
    pub log_to_file: bool,
    pub log_file_path: Option<String>,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub connection_timeout_ms: u64,
    pub retry_attempts: u32,
    pub retry_delay_ms: u64,
    pub enable_proxy: bool,
    pub proxy_url: Option<String>,
}

impl Default for UnifiedConfig {
    fn default() -> Self {
        Self {
            app: AppConfig::default(),
            user: UserConfig::default(),
            matrix: MatrixConfigExt::default(),
            legacy: LegacyConfig::default(),
            ui: UiConfig::default(),
            logging: LoggingConfig::default(),
            network: NetworkConfig::default(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            communication_mode: CommunicationMode::Matrix,
            auto_start_mode: true,
            enable_sounds: true,
            enable_notifications: true,
        }
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        let username = whoami::username();
        let user_id = uuid::Uuid::new_v4().to_string();
        
        Self {
            user_id,
            username,
            display_name: None,
            avatar_url: None,
            default_status: "online".to_string(),
        }
    }
}

impl Default for MatrixConfigExt {
    fn default() -> Self {
        Self {
            homeserver_url: "http://localhost:6167".to_string(),
            server_name: "nok.local".to_string(),
            device_name: "nok-client".to_string(),
            store_path: "/tmp/nok_matrix_store".to_string(),
            auto_login: false,
            enable_encryption: true,
            sync_timeout_ms: 30000,
            presence_enabled: true,
        }
    }
}

impl Default for LegacyConfig {
    fn default() -> Self {
        Self {
            server_url: "ws://localhost:8001".to_string(),
            api_endpoint: "http://localhost:8001/api".to_string(),
            websocket_endpoint: "ws://localhost:8001/ws".to_string(),
            timeout_ms: 10000,
            reconnect_attempts: 3,
            enable_fallback: true,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            default_tab: "rooms".to_string(),
            show_ascii_art: true,
            compact_mode: false,
            show_timestamps: true,
            max_message_history: 1000,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            enable_debug: false,
            max_log_entries: 1000,
            log_to_file: false,
            log_file_path: None,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            connection_timeout_ms: 10000,
            retry_attempts: 3,
            retry_delay_ms: 1000,
            enable_proxy: false,
            proxy_url: None,
        }
    }
}

impl UnifiedConfig {
    /// Load configuration from file, create default if not exists
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        
        if config_path.exists() {
            match Self::load_from_file(&config_path) {
                Ok(config) => {
                    // Migrate any old config format if needed
                    Self::migrate_if_needed(config)
                }
                Err(e) => {
                    eprintln!("Failed to load config: {}, using defaults", e);
                    Self::default()
                }
            }
        } else {
            // Create default config and save it
            let config = Self::default();
            if let Err(e) = config.save() {
                eprintln!("Failed to save default config: {}", e);
            }
            config
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> NokResult<()> {
        let config_path = Self::get_config_path();
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| NokError::FileSystemError(e))?;
        }
        
        // Write config file
        let config_json = serde_json::to_string_pretty(self)
            .map_err(|e| NokError::ConfigParseError(e.to_string()))?;
        
        fs::write(&config_path, config_json)
            .map_err(|e| NokError::FileSystemError(e))?;
        
        Ok(())
    }

    /// Get the configuration file path
    pub fn get_config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("nok").join("config.json")
        } else {
            PathBuf::from(".nok_config.json")
        }
    }

    /// Load configuration from a specific file
    fn load_from_file(path: &Path) -> NokResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| NokError::FileSystemError(e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| NokError::ConfigParseError(e.to_string()))
    }

    /// Migrate from older config formats if needed
    fn migrate_if_needed(mut config: Self) -> Self {
        // Update version
        config.app.version = env!("CARGO_PKG_VERSION").to_string();
        
        // Add any migration logic here for older config versions
        // For example:
        // if config.app.version < "0.2.0" {
        //     // Migrate from older format
        // }
        
        config
    }

    /// Convert to Matrix SDK configuration
    pub fn to_matrix_config(&self) -> MatrixConfig {
        MatrixConfig {
            homeserver_url: self.matrix.homeserver_url.clone(),
            server_name: self.matrix.server_name.clone(),
            device_name: self.matrix.device_name.clone(),
            state_store_path: self.matrix.store_path.clone(),
            store_path: self.matrix.store_path.clone(),
        }
    }

    /// Update from Matrix configuration
    pub fn update_from_matrix_config(&mut self, matrix_config: &MatrixConfig) {
        self.matrix.homeserver_url = matrix_config.homeserver_url.clone();
        self.matrix.server_name = matrix_config.server_name.clone();
        self.matrix.device_name = matrix_config.device_name.clone();
        self.matrix.store_path = matrix_config.store_path.clone();
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate Matrix config
        if self.matrix.homeserver_url.is_empty() {
            errors.push("Matrix homeserver URL cannot be empty".to_string());
        }
        if self.matrix.server_name.is_empty() {
            errors.push("Matrix server name cannot be empty".to_string());
        }

        // Validate user config
        if self.user.username.is_empty() {
            errors.push("Username cannot be empty".to_string());
        }
        if self.user.user_id.is_empty() {
            errors.push("User ID cannot be empty".to_string());
        }

        // Validate network settings
        if self.network.connection_timeout_ms == 0 {
            errors.push("Connection timeout must be greater than 0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Apply environment variable overrides
    pub fn apply_env_overrides(&mut self) {
        // Matrix overrides
        if let Ok(url) = std::env::var("NOK_MATRIX_HOMESERVER") {
            self.matrix.homeserver_url = url;
        }
        if let Ok(server) = std::env::var("NOK_MATRIX_SERVER") {
            self.matrix.server_name = server;
        }

        // Communication mode override
        if let Ok(mode) = std::env::var("NOK_COMMUNICATION_MODE") {
            match mode.to_lowercase().as_str() {
                "matrix" => self.app.communication_mode = CommunicationMode::Matrix,
                "legacy" => self.app.communication_mode = CommunicationMode::Legacy,
                "hybrid" => self.app.communication_mode = CommunicationMode::Hybrid,
                _ => {}
            }
        }

        // Logging overrides
        if let Ok(level) = std::env::var("NOK_LOG_LEVEL") {
            self.logging.level = level;
        }
        if let Ok(_) = std::env::var("NOK_DEBUG") {
            self.logging.enable_debug = true;
        }

        // User overrides
        if let Ok(username) = std::env::var("NOK_USERNAME") {
            self.user.username = username;
        }
    }

    /// Get a user-friendly summary of the configuration
    pub fn summary(&self) -> String {
        format!(
            "nok Configuration Summary:
  Version: {}
  Communication Mode: {:?}
  Username: {}
  Matrix Server: {}
  Legacy Server: {}
  UI Theme: {}
  Debug Enabled: {}",
            self.app.version,
            self.app.communication_mode,
            self.user.username,
            self.matrix.homeserver_url,
            self.legacy.server_url,
            self.ui.theme,
            self.logging.enable_debug
        )
    }
}

/// Configuration migration utilities
pub struct ConfigMigration;

impl ConfigMigration {
    /// Migrate from legacy config.json to unified config
    pub fn migrate_from_legacy(legacy_config_path: &Path) -> NokResult<UnifiedConfig> {
        // This would read the old config format and convert to new format
        // For now, return default config
        
        if legacy_config_path.exists() {
            // Read legacy config and extract relevant fields
            // This is a placeholder implementation
            let mut config = UnifiedConfig::default();
            
            // Try to read legacy fields if they exist
            if let Ok(content) = fs::read_to_string(legacy_config_path) {
                if let Ok(legacy_value) = serde_json::from_str::<serde_json::Value>(&content) {
                    // Extract username
                    if let Some(username) = legacy_value.get("username").and_then(|v| v.as_str()) {
                        config.user.username = username.to_string();
                    }
                    
                    // Extract user_id
                    if let Some(user_id) = legacy_value.get("user_id").and_then(|v| v.as_str()) {
                        config.user.user_id = user_id.to_string();
                    }
                    
                    // Extract server URL
                    if let Some(server_url) = legacy_value.get("last_server_url").and_then(|v| v.as_str()) {
                        config.legacy.server_url = server_url.to_string();
                    }
                }
            }
            
            // Save the migrated config
            config.save()?;
            
            Ok(config)
        } else {
            Ok(UnifiedConfig::default())
        }
    }

    /// Backup current config before migration
    pub fn backup_config(config_path: &Path) -> NokResult<PathBuf> {
        if !config_path.exists() {
            return Err(NokError::FileNotFound(config_path.to_string_lossy().to_string()));
        }

        let backup_path = config_path.with_extension("json.backup");
        fs::copy(config_path, &backup_path)
            .map_err(|e| NokError::FileSystemError(e))?;
        
        Ok(backup_path)
    }
}