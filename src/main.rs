mod app;
mod ui;
mod audio;
mod util;
mod api;
mod matrix;
mod migration;

use std::env;
use app::{App, NewApp};
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
    println!("🚀 Starting nok in TUI mode with new architecture...");

    // Create new app instance with modular architecture
    let app = NewApp::new();

    // Start TUI with new architecture
    ui::run_app_new(app).await?;

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
