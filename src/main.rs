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
    // Setup terminal immediately (no console output before this)
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Create app state
    let mut app = App::new();

    // Try to connect to backend
    app.add_debug_log("Initializing connection to backend...".to_string());
    match app.initialize_connection().await {
        Ok(_) => {
            app.add_debug_log("Successfully connected to backend".to_string());
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
        eprintln!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_tick = std::time::Instant::now();
    let tick_rate = Duration::from_millis(250);

    // 初回描画
    terminal.draw(|f| ui(f, &mut app))?;

    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // キー入力をチェック（タイムアウト付き）
        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    // 設定画面の場合は専用処理を最優先
                    if app.state == app::AppState::Settings {
                        app.add_settings_log(format!("Settings key pressed: {:?}", key.code));
                        app.handle_key(key);
                        terminal.draw(|f| ui(f, &mut app))?;
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') => {
                            app.add_debug_log("Quit key pressed".to_string());
                            break;
                        },
                        KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                            app.add_debug_log("Ctrl+C received".to_string());
                            break;
                        },
                        KeyCode::Tab => {
                            app.add_debug_log("Tab key pressed - cycling focus".to_string());
                            app.cycle_focus(false);
                        },
                        KeyCode::Up => {
                            app.add_debug_log("Up key pressed".to_string());
                            app.handle_up_key();
                        },
                        KeyCode::Down => {
                            app.add_debug_log("Down key pressed".to_string());
                            app.handle_down_key();
                        },
                        KeyCode::Enter => {
                            app.add_debug_log("Enter key pressed".to_string());
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
                        KeyCode::F(5) => {
                            // F5キーで再接続
                            match app.reconnect().await {
                                Ok(_) => {
                                    app.notification = Some("Reconnected successfully!".to_string());
                                }
                                Err(e) => {
                                    app.set_error(format!("Reconnection failed: {}", e));
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
            // リコネクトフラグがセットされている場合は再接続を実行
            if app.should_reconnect {
                app.should_reconnect = false;
                match app.reconnect().await {
                    Ok(_) => {
                        app.add_settings_log("Automatic reconnection successful!".to_string());
                        app.notification = Some("Reconnected successfully with new username!".to_string());
                    }
                    Err(e) => {
                        app.add_settings_log(format!("Automatic reconnection failed: {}", e));
                        app.set_error(format!("Reconnection failed: {}", e));
                    }
                }
            }

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
