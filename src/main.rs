mod app;
mod ui;
mod audio;
mod util;
mod api;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use tokio::time::interval;

use app::App;
use ui::{ui, TabView};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Try to connect to backend (but continue even if it fails)
    match app.initialize_connection().await {
        Ok(_) => {
            // Show connection success
            app.notification = Some("Connected to backend!".to_string());
        }
        Err(e) => {
            // Restore terminal before showing error
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;

            eprintln!("❌ Failed to connect to backend: {}", e);
            eprintln!("💡 Make sure the backend is running on port 8001:");
            eprintln!("   cd backend && python main.py");
            eprintln!("📚 See backend/README.md for setup instructions");

            return Ok(());
        }
    }

    // Run app
    let res = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tick_interval = interval(Duration::from_millis(100));
    let mut websocket_interval = interval(Duration::from_millis(50)); // WebSocketメッセージチェック用

    loop {
        tokio::select! {
            // ターミナル描画とキー入力処理
            _ = tick_interval.tick() => {
                // UI描画
                terminal.draw(|f| ui(f, &mut app))?;

                // キー入力チェック
                if crossterm::event::poll(Duration::from_millis(0))? {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('n') => {
                                if app.focused_pane == app::PaneIdentifier::Users {
                                    if let Some(user) = app.get_selected_user() {
                                        let username = user.name.clone();
                                        if let (Some(sender_id), Some(target_id)) = (&app.current_user.id, &user.id) {
                                            if let Err(e) = app.websocket_client.send_knock(sender_id, target_id) {
                                                app.set_error(format!("Failed to send knock: {}", e));
                                            } else {
                                                app.notification = Some(format!("Knocked on {}", username));
                                                // Play knock sound
                                                if let Err(_) = audio::play_knock_sound() {
                                                    // Silently handle audio errors
                                                }
                                            }
                                        }
                                    } else {
                                        app.set_error("No user selected to knock.".to_string());
                                    }
                                }
                            },
                            KeyCode::Char('r') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                                // Ctrl+R でデータリフレッシュ
                                if let Err(e) = app.refresh_data().await {
                                    app.set_error(format!("Failed to refresh data: {}", e));
                                } else {
                                    app.notification = Some("Data refreshed!".to_string());
                                }
                            },
                            _ => app.handle_key(key),
                        }
                    }
                }

                app.tick();
            }

            // WebSocketメッセージ処理
            _ = websocket_interval.tick() => {
                app.handle_websocket_message().await;
            }
        }
    }
}
