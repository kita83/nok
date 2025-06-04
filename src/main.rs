mod app;
mod ui;
mod audio;
mod util;
mod api;
mod matrix;
mod migration;

use std::env;
use app::App;
use migration::command::MigrationCommand;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Handle migration commands
    if args.len() > 1 {
        match args[1].as_str() {
            "--migrate" => {
                println!("🚀 Starting nok Matrix migration...");
                let migration = MigrationCommand::new("backend/nok.db", "nok.local");
                migration.execute().await?;
                return Ok(());
            }
            "--migrate-dry-run" => {
                println!("🔍 Running migration analysis...");
                let migration = MigrationCommand::new("backend/nok.db", "nok.local");
                migration.dry_run().await?;
                return Ok(());
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--test-audio" => {
                println!("Testing knock sound...");
                match audio::play_knock_sound() {
                    Ok(_) => println!("✅ Knock sound played successfully!"),
                    Err(e) => println!("❌ Error playing knock sound: {}", e),
                }
                return Ok(());
            }
            _ => {
                println!("❌ Unknown command: {}", args[1]);
                println!("Use --help for available commands");
                return Ok(());
            }
        }
    }

    // Regular TUI mode
    println!("🚀 Starting nok in TUI mode...");

    // Create app instance
    let mut app = App::new();

    // Check if already migrated to Matrix
    if let Ok(config_migrator) = migration::config::ConfigMigrator::new() {
        if config_migrator.matrix_config_exists() {
            println!("📡 Matrix configuration detected - starting in Matrix mode");
            // Initialize Matrix mode (when available)
            app.toggle_matrix_mode();
            if let Err(e) = app.initialize_matrix().await {
                eprintln!("❌ Failed to initialize Matrix mode: {}", e);
                println!("💡 You may need to run migration first: nok --migrate");
            }
        } else {
            println!("📻 Starting in legacy mode");
            println!("💡 To migrate to Matrix, run: nok --migrate");
        }
    }

    // Initialize connection
    if let Err(e) = app.initialize_connection().await {
        eprintln!("❌ Failed to initialize connection: {}", e);
        eprintln!("💡 If you have migrated to Matrix, ensure Conduit is running");
    }

    // Start TUI
    ui::run_app(app).await?;

    Ok(())
}

fn print_help() {
    println!(r#"
nok - Terminal-based virtual office application

USAGE:
    nok [COMMAND]

COMMANDS:
    (no command)     Start nok in TUI mode
    --migrate        Execute full migration from legacy to Matrix
    --migrate-dry-run Run migration analysis without making changes
    --test-audio     Test knock sound playback
    --help, -h       Show this help message

EXAMPLES:
    nok                      # Start TUI
    nok --migrate-dry-run    # Analyze migration
    nok --migrate            # Execute migration
    nok --test-audio         # Test audio

MIGRATION:
    Before migrating, ensure:
    1. Conduit homeserver is running (backend/conduit/start_conduit.sh)
    2. Legacy database exists (backend/nok.db)
    3. Backup important data

    After migration:
    - Legacy data is backed up automatically
    - New Matrix configuration is created
    - ID mappings are saved for reference
    "#);
}
