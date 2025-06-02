mod app;
mod ui;
mod audio;
mod util;
mod api;

use std::io;
use std::time::Duration;

use tokio::time;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};


use app::App;
use ui::{ui, TabView};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("🚀 Starting NOK Chat Application...");

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Try to connect to backend
    match app.initialize_connection().await {
        Ok(_) => {
            eprintln!("✅ Connected to backend successfully");
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
            eprintln!("💡 Make sure the backend is running:");
            eprintln!("   cd backend && uv run python main.py");

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

    eprintln!("👋 Goodbye!");
    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("🔄 Starting event loop...");

    let mut last_tick = std::time::Instant::now();
    let tick_rate = Duration::from_millis(250);

    // 初回描画
    terminal.draw(|f| ui(f, &mut app))?;
    eprintln!("✅ Application ready! Use 'q' to quit.");

    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // キー入力をチェック（タイムアウト付き）
        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('q') => {
                            eprintln!("👋 Quitting...");
                            break;
                        },
                        KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                            eprintln!("👋 Ctrl+C received");
                            break;
                        },
                        KeyCode::Tab => {
                            app.cycle_focus(false);
                        },
                        KeyCode::Up => {
                            app.handle_up_key();
                        },
                        KeyCode::Down => {
                            app.handle_down_key();
                        },
                        KeyCode::Enter => {
                            app.handle_confirm_key();
                        },
                        KeyCode::Char('n') => {
                            if app.focused_pane == app::PaneIdentifier::Users {
                                if let Some(user) = app.get_selected_user() {
                                    let username = user.name.clone();
                                    if let (Some(sender_id), Some(target_id)) = (&app.current_user.id, &user.id) {
                                        if let Err(e) = app.websocket_client.send_knock(sender_id, target_id) {
                                            app.set_error(format!("Failed to send knock: {}", e));
                                        } else {
                                            app.notification = Some(format!("Knocked on {}", username));
                                        }
                                    }
                                } else {
                                    app.set_error("No user selected to knock.".to_string());
                                }
                            }
                        },
                        _ => {
                            app.handle_key(key);
                        }
                    }

                    // キー入力後に再描画
                    terminal.draw(|f| ui(f, &mut app))?;
                }
                _ => {} // その他のイベントは無視
            }
        }

        // 定期的な更新処理
        if last_tick.elapsed() >= tick_rate {
            // WebSocketメッセージ処理を安全に実行（ログ出力なし）
            tokio::select! {
                _ = app.handle_websocket_message() => {}
                _ = time::sleep(Duration::from_millis(10)) => {}
            }

            // 定期的な再描画
            terminal.draw(|f| ui(f, &mut app))?;

            app.tick();
            last_tick = std::time::Instant::now();
        }
    }

    Ok(())
}
