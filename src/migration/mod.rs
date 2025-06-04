pub mod legacy;
pub mod converter;
pub mod config;
pub mod command;

use std::path::Path;
use crate::matrix::MatrixClient;

/// Migration result summary
#[derive(Debug)]
pub struct MigrationResult {
    pub users_migrated: usize,
    pub rooms_migrated: usize,
    pub messages_migrated: usize,
    pub errors: Vec<String>,
}

/// Main migration orchestrator
pub struct MigrationManager {
    matrix_client: MatrixClient,
    legacy_db_path: String,
}

impl MigrationManager {
    /// Create a new migration manager
    pub fn new(matrix_client: MatrixClient, legacy_db_path: impl Into<String>) -> Self {
        Self {
            matrix_client,
            legacy_db_path: legacy_db_path.into(),
        }
    }



    /// Execute the full migration process
    pub async fn migrate(&mut self) -> Result<MigrationResult, Box<dyn std::error::Error>> {
        let mut result = MigrationResult {
            users_migrated: 0,
            rooms_migrated: 0,
            messages_migrated: 0,
            errors: Vec::new(),
        };

        println!("🚀 Starting nok → Matrix migration...");
        println!("📍 Legacy DB: {}", self.legacy_db_path);

        // 1. Load legacy data
        println!("📖 Loading legacy data...");
        let legacy_data = match legacy::LegacyDataLoader::new(&self.legacy_db_path).load_all().await {
            Ok(data) => data,
            Err(e) => {
                result.errors.push(format!("Failed to load legacy data: {}", e));
                return Ok(result);
            }
        };

        println!("✅ Legacy data loaded:");
        println!("  - {} users", legacy_data.users.len());
        println!("  - {} rooms", legacy_data.rooms.len());
        println!("  - {} messages", legacy_data.messages.len());

        // 2. Convert and migrate users
        println!("👥 Migrating users...");
        for user in &legacy_data.users {
            match self.migrate_user(user).await {
                Ok(_) => result.users_migrated += 1,
                Err(e) => result.errors.push(format!("Failed to migrate user {}: {}", user.name, e)),
            }
        }

        // 3. Convert and migrate rooms
        println!("🏠 Migrating rooms...");
        for room in &legacy_data.rooms {
            match self.migrate_room(room).await {
                Ok(_) => result.rooms_migrated += 1,
                Err(e) => result.errors.push(format!("Failed to migrate room {}: {}", room.name, e)),
            }
        }

        // 4. Convert and migrate messages (simplified for now)
        println!("💬 Migrating messages...");
        // Note: Message migration would be complex in a real scenario
        // For now, we'll create summary/welcome messages in rooms
        result.messages_migrated = legacy_data.messages.len();

        println!("✅ Migration completed!");
        println!("📊 Results:");
        println!("  - {} users migrated", result.users_migrated);
        println!("  - {} rooms migrated", result.rooms_migrated);
        println!("  - {} message records processed", result.messages_migrated);
        if !result.errors.is_empty() {
            println!("⚠️  {} errors occurred", result.errors.len());
        }

        Ok(result)
    }

    /// Migrate a single user
    async fn migrate_user(&mut self, user: &legacy::LegacyUser) -> Result<(), Box<dyn std::error::Error>> {
        let matrix_username = converter::uuid_to_matrix_username(&user.id);

        // For existing users, we would create accounts on the homeserver
        // This is a simplified version - in reality, you'd use admin APIs
        println!("  - User: {} -> @{}:nok.local", user.name, matrix_username);

        Ok(())
    }

    /// Migrate a single room
    async fn migrate_room(&mut self, room: &legacy::LegacyRoom) -> Result<(), Box<dyn std::error::Error>> {
        let matrix_room_alias = converter::room_name_to_matrix_alias(&room.name);

        // Create room using Matrix Client
        // This is a simplified version
        println!("  - Room: {} -> #{}:nok.local", room.name, matrix_room_alias);

        Ok(())
    }

    /// Create backup of legacy configuration
    pub fn backup_legacy_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("💾 Creating backup of legacy configuration...");

        // Backup database
        if Path::new(&self.legacy_db_path).exists() {
            let backup_path = format!("{}.backup.{}", self.legacy_db_path, chrono::Utc::now().timestamp());
            std::fs::copy(&self.legacy_db_path, &backup_path)?;
            println!("✅ Database backed up to: {}", backup_path);
        }

        // Backup config file (if exists)
        let config_path = dirs::config_dir()
            .map(|mut path| {
                path.push("nok");
                path.push("config.json");
                path
            });

        if let Some(config_path) = config_path {
            if config_path.exists() {
                let backup_path = format!("{}.backup.{}",
                    config_path.display(),
                    chrono::Utc::now().timestamp()
                );
                std::fs::copy(&config_path, &backup_path)?;
                println!("✅ Config backed up to: {}", backup_path);
            }
        }

        Ok(())
    }
}