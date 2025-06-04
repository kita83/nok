use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Legacy configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyConfig {
    pub user_id: String,
    pub username: String,
    pub server_url: Option<String>,
    pub auto_connect: Option<bool>,
    pub theme: Option<String>,
}

/// New Matrix configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixConfig {
    /// Matrix User ID (@username:domain)
    pub matrix_user_id: String,
    /// Display name
    pub display_name: String,
    /// Homeserver URL
    pub homeserver_url: String,
    /// Server name (domain part)
    pub server_name: String,
    /// Matrix state store path
    pub state_store_path: String,
    /// Auto-login on startup
    pub auto_login: bool,
    /// Theme setting (preserved from legacy)
    pub theme: String,
    /// Legacy user ID (for reference)
    pub legacy_user_id: Option<String>,
    /// Migration timestamp
    pub migrated_at: String,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        Self {
            matrix_user_id: String::new(),
            display_name: String::new(),
            homeserver_url: "http://nok.local:6167".to_string(),
            server_name: "nok.local".to_string(),
            state_store_path: "matrix_state.db".to_string(),
            auto_login: true,
            theme: "default".to_string(),
            legacy_user_id: None,
            migrated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Configuration migrator
pub struct ConfigMigrator {
    legacy_config_path: PathBuf,
    new_config_path: PathBuf,
}

impl ConfigMigrator {
    /// Create a new configuration migrator
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;

        let nok_config_dir = config_dir.join("nok");

        Ok(Self {
            legacy_config_path: nok_config_dir.join("config.json"),
            new_config_path: nok_config_dir.join("matrix_config.json"),
        })
    }

    /// Load legacy configuration
    pub fn load_legacy_config(&self) -> Result<Option<LegacyConfig>, Box<dyn std::error::Error>> {
        if !self.legacy_config_path.exists() {
            println!("📄 No legacy config found at: {}", self.legacy_config_path.display());
            return Ok(None);
        }

        println!("📖 Loading legacy config from: {}", self.legacy_config_path.display());
        let content = std::fs::read_to_string(&self.legacy_config_path)?;
        let config: LegacyConfig = serde_json::from_str(&content)?;

        println!("✅ Legacy config loaded:");
        println!("  - User ID: {}", config.user_id);
        println!("  - Username: {}", config.username);
        if let Some(ref server_url) = config.server_url {
            println!("  - Server URL: {}", server_url);
        }

        Ok(Some(config))
    }

    /// Convert legacy config to Matrix config
    pub fn convert_to_matrix_config(
        &self,
        legacy_config: &LegacyConfig,
        matrix_user_id: &str,
    ) -> MatrixConfig {
        let mut matrix_config = MatrixConfig::default();

        matrix_config.matrix_user_id = matrix_user_id.to_string();
        matrix_config.display_name = legacy_config.username.clone();
        matrix_config.theme = legacy_config.theme.clone().unwrap_or_else(|| "default".to_string());
        matrix_config.auto_login = legacy_config.auto_connect.unwrap_or(true);
        matrix_config.legacy_user_id = Some(legacy_config.user_id.clone());

        matrix_config
    }

    /// Save Matrix configuration
    pub fn save_matrix_config(&self, config: &MatrixConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure config directory exists
        if let Some(parent) = self.new_config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(config)?;
        std::fs::write(&self.new_config_path, json)?;

        println!("✅ Matrix config saved to: {}", self.new_config_path.display());
        Ok(())
    }

    /// Create backup of legacy configuration
    pub fn backup_legacy_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.legacy_config_path.exists() {
            println!("📄 No legacy config to backup");
            return Ok(());
        }

        let timestamp = chrono::Utc::now().timestamp();
        let backup_path = self.legacy_config_path.with_extension(format!("json.backup.{}", timestamp));

        std::fs::copy(&self.legacy_config_path, &backup_path)?;
        println!("💾 Legacy config backed up to: {}", backup_path.display());

        Ok(())
    }

    /// Execute full config migration
    pub fn migrate_config(&self, matrix_user_id: &str) -> Result<Option<MatrixConfig>, Box<dyn std::error::Error>> {
        println!("🔄 Starting configuration migration...");

        // Load legacy config
        let legacy_config = match self.load_legacy_config()? {
            Some(config) => config,
            None => {
                println!("⚠️  No legacy config found, creating default Matrix config");
                let mut default_config = MatrixConfig::default();
                default_config.matrix_user_id = matrix_user_id.to_string();
                self.save_matrix_config(&default_config)?;
                return Ok(Some(default_config));
            }
        };

        // Create backup
        self.backup_legacy_config()?;

        // Convert to Matrix config
        let matrix_config = self.convert_to_matrix_config(&legacy_config, matrix_user_id);

        // Save new config
        self.save_matrix_config(&matrix_config)?;

        println!("✅ Configuration migration completed!");
        Ok(Some(matrix_config))
    }

    /// Check if Matrix config already exists
    pub fn matrix_config_exists(&self) -> bool {
        self.new_config_path.exists()
    }

    /// Load existing Matrix configuration
    pub fn load_matrix_config(&self) -> Result<Option<MatrixConfig>, Box<dyn std::error::Error>> {
        if !self.new_config_path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&self.new_config_path)?;
        let config: MatrixConfig = serde_json::from_str(&content)?;
        Ok(Some(config))
    }
}