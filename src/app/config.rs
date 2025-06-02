use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub user_id: String,
    pub username: String,
    pub last_server_url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4().to_string(),
            username: Self::generate_default_username(),
            last_server_url: Some("ws://localhost:8001".to_string()),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        match fs::read_to_string(&config_path) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(config) => config,
                    Err(_) => {
                        // 設定ファイルが壊れている場合、デフォルトで上書き
                        let default_config = Self::default();
                        default_config.save();
                        default_config
                    }
                }
            }
            Err(_) => {
                // 設定ファイルが存在しない場合、新規作成
                let default_config = Self::default();
                default_config.save();
                default_config
            }
        }
    }

    pub fn save(&self) {
        let config_path = Self::get_config_path();

        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(content) = serde_json::to_string_pretty(self) {
            println!("DEBUG: Saving config to {:?}", config_path);
            println!("DEBUG: Config content: {}", content);
            let result = fs::write(&config_path, content);
            if let Err(e) = result {
                println!("DEBUG: Failed to save config: {}", e);
            } else {
                println!("DEBUG: Config saved successfully");
            }
        } else {
            println!("DEBUG: Failed to serialize config");
        }
    }

    pub fn update_username(&mut self, new_username: String) {
        println!("DEBUG: Updating username from '{}' to '{}'", self.username, new_username);
        self.username = new_username;
        self.save();
    }

    fn get_config_path() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".config").join("nok").join("config.json")
    }

    fn generate_default_username() -> String {
        // システムユーザー名を取得してデフォルトとして使用
        if let Ok(username) = std::env::var("USER") {
            username
        } else if let Ok(username) = std::env::var("USERNAME") {
            username
        } else {
            format!("user_{}", Uuid::new_v4().to_string()[..8].to_lowercase())
        }
    }
}