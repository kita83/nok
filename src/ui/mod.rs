use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use tokio::time;

use crate::app::{App, AppState, PaneIdentifier, LoginField};

#[derive(Clone, Copy, PartialEq)]
pub enum TabView {
    Rooms,
    Users,
    Chat,
    Settings,
}

pub fn ui(f: &mut Frame, app: &mut App) {
    match app.state {
        AppState::Login => {
            render_login(f, app);
            return;
        }
        AppState::Settings => {
            render_settings(f, app);
            return;
        }
        _ => {}
    }
    render_main_ui(f, app);
}

fn render_login(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Main layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),      // Title
            Constraint::Length(6),      // Username field
            Constraint::Length(6),      // Password field
            Constraint::Length(4),      // Login button
            Constraint::Length(3),      // Error message
            Constraint::Min(3),         // Help
        ].as_ref())
        .split(size);

    // Title
    let title_block = Block::default()
        .title("nok Matrix Login")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let title_area = title_block.inner(main_chunks[0]);
    f.render_widget(title_block, main_chunks[0]);

    let title_text = "Welcome to nok Matrix Edition!\n\nPlease enter your Matrix credentials:";
    let title_paragraph = Paragraph::new(title_text)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });
    f.render_widget(title_paragraph, title_area);

    // Username field
    let username_border_style = if app.login_field_focus == LoginField::Username {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };
    let username_block = Block::default()
        .title("Username")
        .borders(Borders::ALL)
        .border_style(username_border_style);
    let username_area = username_block.inner(main_chunks[1]);
    f.render_widget(username_block, main_chunks[1]);

    let username_text = if app.login_field_focus == LoginField::Username {
        format!("{}_", app.login_username)
    } else {
        app.login_username.clone()
    };
    let username_paragraph = Paragraph::new(username_text)
        .style(Style::default().fg(Color::White));
    f.render_widget(username_paragraph, username_area);

    // Password field
    let password_border_style = if app.login_field_focus == LoginField::Password {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };
    let password_block = Block::default()
        .title("Password")
        .borders(Borders::ALL)
        .border_style(password_border_style);
    let password_area = password_block.inner(main_chunks[2]);
    f.render_widget(password_block, main_chunks[2]);

    let password_text = if app.login_field_focus == LoginField::Password {
        format!("{}_", "*".repeat(app.login_password.len()))
    } else {
        "*".repeat(app.login_password.len())
    };
    let password_paragraph = Paragraph::new(password_text)
        .style(Style::default().fg(Color::White));
    f.render_widget(password_paragraph, password_area);

    // Login button
    let login_block = Block::default()
        .title("Login")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    let login_area = login_block.inner(main_chunks[3]);
    f.render_widget(login_block, main_chunks[3]);

    let login_text = "Press Enter to login";
    let login_paragraph = Paragraph::new(login_text)
        .style(Style::default().fg(Color::Green));
    f.render_widget(login_paragraph, login_area);

    // Error message
    if let Some(ref error) = app.login_error {
        let error_block = Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red));
        let error_area = error_block.inner(main_chunks[4]);
        f.render_widget(error_block, main_chunks[4]);

        let error_paragraph = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true });
        f.render_widget(error_paragraph, error_area);
    }

    // Help
    let help_block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));
    let help_area = help_block.inner(main_chunks[5]);
    f.render_widget(help_block, main_chunks[5]);

    let help_text = "• Tab: Switch between fields\n• Enter: Login\n• Esc: Exit\n\nExample: demo_user / demo1234";
    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .wrap(Wrap { trim: true });
    f.render_widget(help_paragraph, help_area);
}

fn render_settings(f: &mut Frame, app: &mut App) {
    let size = f.size();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // タイトル
            Constraint::Length(8),      // ユーザー名設定
            Constraint::Length(6),      // ステータス設定
            Constraint::Length(3),      // 保存ボタン
            Constraint::Length(8),      // ログ表示エリア
            Constraint::Min(3),         // ヘルプ
        ].as_ref())
        .split(size);

    // タイトル
    let title_block = Block::default()
        .title("Settings")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    f.render_widget(title_block, main_chunks[0]);

    // ユーザー名設定
    let username_block = Block::default()
        .title("Username")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let username_area = username_block.inner(main_chunks[1]);
    f.render_widget(username_block, main_chunks[1]);

    let username_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),      // 現在のユーザー名
            Constraint::Length(1),      // 空行
            Constraint::Length(2),      // 編集中のユーザー名
        ].as_ref())
        .split(username_area);

    // 現在のユーザー名
    let current_username_text = format!("Current username: {}", app.config.username);
    let current_username_paragraph = Paragraph::new(current_username_text)
        .style(Style::default().fg(Color::White));
    f.render_widget(current_username_paragraph, username_chunks[0]);

    // 編集中のユーザー名
    let edit_username_text = format!("New username: {}_", app.username_edit_buffer);
    let edit_username_paragraph = Paragraph::new(edit_username_text)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    f.render_widget(edit_username_paragraph, username_chunks[2]);

    // ステータス設定
    let status_block = Block::default()
        .title("Status")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let status_area = status_block.inner(main_chunks[2]);
    f.render_widget(status_block, main_chunks[2]);

    let status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),      // 現在のステータス
            Constraint::Length(1),      // 空行
            Constraint::Length(2),      // ステータス選択
        ].as_ref())
        .split(status_area);

    // 現在のステータス
    let current_status_char = match app.current_user.status {
        crate::app::user::UserStatus::Online => "●",
        crate::app::user::UserStatus::Away => "○",
        crate::app::user::UserStatus::Busy => "◆",
        crate::app::user::UserStatus::Offline => "◇",
    };
    let current_status_text = format!("Current status: {} {:?}", current_status_char, app.current_user.status);
    let current_status_paragraph = Paragraph::new(current_status_text)
        .style(Style::default().fg(Color::White));
    f.render_widget(current_status_paragraph, status_chunks[0]);

    // ステータス選択
    let status_options = ["Online ●", "Away ○", "Busy ◆"];
    let selected_option = &status_options[app.status_selection_index];
    let status_selection_text = format!("Select: {} (Tab to cycle, Space to apply)", selected_option);
    let status_selection_paragraph = Paragraph::new(status_selection_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(status_selection_paragraph, status_chunks[2]);

    // 保存ボタン
    let save_block = Block::default()
        .title("Actions")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let save_area = save_block.inner(main_chunks[3]);
    f.render_widget(save_block, main_chunks[3]);

    let save_text = "Press Enter to save, Esc to cancel";
    let save_paragraph = Paragraph::new(save_text)
        .style(Style::default().fg(Color::Green));
    f.render_widget(save_paragraph, save_area);

    // ログ表示エリア
    let log_block = Block::default()
        .title("Settings Log")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let log_area = log_block.inner(main_chunks[4]);
    f.render_widget(log_block, main_chunks[4]);

    // 最新のログから表示（最大5行）
    let visible_logs: Vec<&String> = app.settings_logs.iter()
        .rev()
        .take(5)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let log_text = if visible_logs.is_empty() {
        "No settings logs yet...".to_string()
    } else {
        visible_logs.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join("\n")
    };

    let log_paragraph = Paragraph::new(log_text)
        .style(Style::default().fg(Color::Gray))
        .wrap(Wrap { trim: true });
    f.render_widget(log_paragraph, log_area);

    // ヘルプ
    let help_block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let help_area = help_block.inner(main_chunks[5]);
    f.render_widget(help_block, main_chunks[5]);

    let help_text = "Settings Help:\n\n\
                     Username:\n\
                     • Type to edit your username\n\
                     • Press Enter to save changes\n\
                     • Press Backspace to delete characters\n\n\
                     Status:\n\
                     • Press Tab to cycle through status options\n\
                     • Press Space to apply selected status\n\n\
                     • Press Esc to cancel and return\n\
                     Note: Username changes take effect after reconnecting.";
    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .wrap(Wrap { trim: true });
    f.render_widget(help_paragraph, help_area);
}

fn render_main_ui(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // ターミナルサイズチェック
    if size.width < 20 || size.height < 10 {
        let error_paragraph = Paragraph::new("Terminal too small!\nMinimum: 20x10")
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true });
        f.render_widget(error_paragraph, size);
        return;
    }

    // Main layout: top area and debug log area at bottom
    let main_vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Main content area
            Constraint::Length(6),  // Debug log area at bottom
        ].as_ref())
        .split(size);

    let main_content_area = main_vertical_chunks[0];
    let debug_log_area = main_vertical_chunks[1];

    // Main content layout: two horizontal panes (left and right)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(main_content_area);

    let left_pane_area = main_chunks[0];
    let right_pane_area = main_chunks[1];

    // Left pane layout: three vertical chunks (Rooms, Users, Messages)
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),      // Rooms - 固定行数
            Constraint::Length(8),      // Users - 固定行数
            Constraint::Min(4),         // Messages - 残りの領域
        ].as_ref())
        .split(left_pane_area);

    let rooms_area = left_chunks[0];
    let users_area = left_chunks[1];
    let messages_area = left_chunks[2];

    // --- Render Rooms Pane ---
    let rooms_block = Block::default()
        .title("Rooms")
        .borders(Borders::ALL)
        .border_style(if app.focused_pane == PaneIdentifier::Rooms {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        });
    let actual_rooms_content_area = rooms_block.inner(rooms_area);
    f.render_widget(rooms_block, rooms_area);

        let room_items: Vec<ListItem> = app.rooms.iter().enumerate().map(|(i, r)| {
        let content = if i == app.current_room {
            format!("* {}", r.name)
        } else {
            format!("  {}", r.name)  // インデントを統一
        };
        // 現在のルームだけ色を変える（選択状態はListStateに任せる）
        let style = if i == app.current_room {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(content).style(style)
    }).collect();

        let rooms_list = List::new(room_items)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
        .highlight_symbol("> ");

    let mut rooms_state = ListState::default();
    if app.focused_pane == PaneIdentifier::Rooms && !app.rooms.is_empty() {
        // 範囲チェックを追加
        let safe_idx = if app.selected_room_idx < app.rooms.len() {
            app.selected_room_idx
        } else {
            0
        };
        rooms_state.select(Some(safe_idx));
    }

    f.render_stateful_widget(rooms_list, actual_rooms_content_area, &mut rooms_state);

    // --- Render Users Pane ---
    let users_block = Block::default()
        .title("Users")
        .borders(Borders::ALL)
        .border_style(if app.focused_pane == PaneIdentifier::Users {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        });
    let actual_users_content_area = users_block.inner(users_area);
    f.render_widget(users_block, users_area);

    let user_items: Vec<ListItem> = app.users.iter().enumerate().map(|(_i, u)| {
        let status_char = match u.status {
            crate::app::user::UserStatus::Online => "●",
            crate::app::user::UserStatus::Away => "○",
            crate::app::user::UserStatus::Busy => "◆",
            crate::app::user::UserStatus::Offline => "◇",
        };
        let status_color = match u.status {
            crate::app::user::UserStatus::Online => Color::Green,
            crate::app::user::UserStatus::Away => Color::Yellow,
            crate::app::user::UserStatus::Busy => Color::Red,
            crate::app::user::UserStatus::Offline => Color::Gray,
        };

        // ユーザーの所属部屋を取得
        let room_names = u.get_room_names(&app.rooms);
        let rooms_display = if room_names.is_empty() {
            "".to_string()
        } else if room_names.len() == 1 {
            format!(" [{}]", room_names[0])
        } else {
            format!(" [{}]", room_names.join(", "))
        };

        let content = format!("{} {}{}", status_char, u.name, rooms_display);
        ListItem::new(content).style(Style::default().fg(status_color))
    }).collect();

    let users_list = List::new(user_items)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
        .highlight_symbol("> ");

    let mut users_state = ListState::default();
    if app.focused_pane == PaneIdentifier::Users && !app.users.is_empty() {
        if let Some(selected) = app.selected_user {
            // 範囲チェックを追加
            let safe_idx = if selected < app.users.len() {
                selected
            } else {
                0
            };
            users_state.select(Some(safe_idx));
        }
    }

    f.render_stateful_widget(users_list, actual_users_content_area, &mut users_state);

    // --- Render Messages Pane ---
    let messages_block = Block::default()
        .title("Messages")
        .borders(Borders::ALL)
        .border_style(if app.focused_pane == PaneIdentifier::Messages {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        });
    let (actual_messages_content_area, input_display_area) =
        if app.state == AppState::Input && app.focused_pane == PaneIdentifier::Messages {
            let temp_messages_block_for_inner_calc = messages_block.clone();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(temp_messages_block_for_inner_calc.inner(messages_area));
            (chunks[0], Some(chunks[1]))
        } else {
            (messages_block.inner(messages_area), None)
        };
    f.render_widget(messages_block, messages_area);

    let message_items: Vec<ListItem> = app.messages.iter().map(|m| {
        let time_str = m.formatted_time();
        let content = format!("[{}] <{}>: {}", time_str, m.sender, m.content);
        ListItem::new(content).style(Style::default().fg(Color::White))
    }).collect();

    let messages_list = List::new(message_items)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
        .highlight_symbol("> ");

    let mut messages_state = ListState::default();
    if app.focused_pane == PaneIdentifier::Messages && !app.messages.is_empty() {
        if let Some(selected) = app.selected_message_idx {
            // 範囲チェックを追加
            let safe_idx = if selected < app.messages.len() {
                selected
            } else {
                0
            };
            messages_state.select(Some(safe_idx));
        }
    }

    f.render_stateful_widget(messages_list, actual_messages_content_area, &mut messages_state);

    if let Some(input_area) = input_display_area {
        let input_text = format!("> {}", app.input);
        let input_paragraph = Paragraph::new(input_text)
            .style(Style::default().fg(Color::Yellow).bg(Color::Black));
        f.render_widget(input_paragraph, input_area);
    }

    // --- Render Right Pane (Room Visualizer) ---
    let right_block = Block::default()
        .title("Room Visualizer")
        .borders(Borders::ALL)
        .border_style(if app.focused_pane == PaneIdentifier::AsciiArt {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        });

    let content_area = right_block.inner(right_pane_area);
    f.render_widget(right_block, right_pane_area);

    // Status information for the right pane
    let mut status_text = format!("Terminal: {}x{}\nConnection: {:?}\nFocus: {:?}",
                             size.width, size.height,
                             app.connection_status,
                             app.focused_pane);

    // ターミナル環境の詳細情報を追加
    status_text.push_str(&format!("\n\nTerminal Info:"));
    status_text.push_str(&format!("\nTERM: {}", std::env::var("TERM").unwrap_or_else(|_| "unknown".to_string())));
    status_text.push_str(&format!("\nTERM_PROGRAM: {}", std::env::var("TERM_PROGRAM").unwrap_or_else(|_| "unknown".to_string())));
    status_text.push_str(&format!("\nCOLORTERM: {}", std::env::var("COLORTERM").unwrap_or_else(|_| "unknown".to_string())));

    // Add room info if available
    if !app.rooms.is_empty() && app.current_room < app.rooms.len() {
        status_text.push_str(&format!("\n\nCurrent Room: {}", app.rooms[app.current_room].name));
    }

    // Add current user status
    let status_char = match app.current_user.status {
        crate::app::user::UserStatus::Online => "●",
        crate::app::user::UserStatus::Away => "○",
        crate::app::user::UserStatus::Busy => "◆",
        crate::app::user::UserStatus::Offline => "◇",
    };
    status_text.push_str(&format!("\nYour Status: {} {:?}", status_char, app.current_user.status));

    // Add controls
    status_text.push_str("\n\nControls:");
    status_text.push_str("\nTab: Switch focus");
    status_text.push_str("\n↑↓: Navigate");
    status_text.push_str("\nEnter: Select");
    status_text.push_str("\nn: Knock (Users)");
    status_text.push_str("\ns: Settings");
    status_text.push_str("\nF5: Reconnect");
    status_text.push_str("\nq: Quit");


    // Show notifications if any
    if let Some(ref notification) = app.notification {
        status_text.push_str(&format!("\n\n✅ {}", notification));
    }

    // Show errors if any
    if let Some(ref error) = app.error {
        status_text.push_str(&format!("\n\n❌ {}", error));
    }

    let status_paragraph = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Green))
        .wrap(Wrap { trim: true });
    f.render_widget(status_paragraph, content_area);

    // --- Render Debug Log Section ---
    let debug_block = Block::default()
        .title("Debug Logs")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let debug_content_area = debug_block.inner(debug_log_area);
    f.render_widget(debug_block, debug_log_area);

    // 最新のログから表示（最大4行）
    let visible_logs: Vec<&String> = app.debug_logs.iter()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let debug_text = if visible_logs.is_empty() {
        "No debug logs yet...".to_string()
    } else {
        visible_logs.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join("\n")
    };

    let debug_paragraph = Paragraph::new(debug_text)
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: true });
    f.render_widget(debug_paragraph, debug_content_area);
}

/// Main TUI application loop
pub async fn run_app(app: App) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Run main loop
    let result = run_app_loop(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_app_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_tick = std::time::Instant::now();
    let tick_rate = Duration::from_millis(250);

    // Initial draw
    terminal.draw(|f| ui(f, &mut app))?;

    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Check for key input
        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    // Login screen has highest priority
                    if app.state == AppState::Login {
                        app.handle_key(key);

                        // Check if login should be attempted
                        if app.notification.as_ref().map_or(false, |n| n.contains("Attempting login")) {
                            app.notification = Some("Logging in...".to_string());
                            terminal.draw(|f| ui(f, &mut app))?;

                            // Attempt login
                            match app.matrix_login(&app.login_username.clone(), &app.login_password.clone()).await {
                                Ok(_) => {
                                    app.login_error = None;
                                    app.notification = Some("Login successful!".to_string());

                                    // Start Matrix sync
                                    if let Err(e) = app.start_matrix_sync().await {
                                        app.login_error = Some(format!("Failed to start sync: {}", e));
                                    } else {
                                        app.state = AppState::Normal;
                                        app.notification = Some("Connected to Matrix!".to_string());
                                    }
                                }
                                Err(e) => {
                                    app.login_error = Some(format!("Login failed: {}", e));
                                    app.notification = None;
                                }
                            }
                        }

                        terminal.draw(|f| ui(f, &mut app))?;
                        continue;
                    }

                    // Settings screen has priority
                    if app.state == AppState::Settings {
                        app.add_settings_log(format!("Settings key pressed: {:?}", key.code));
                        app.handle_key(key);
                        terminal.draw(|f| ui(f, &mut app))?;
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') if app.state != AppState::Login => {
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
                        KeyCode::F(5) => {
                            // F5 key for reconnection
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

                    // Redraw after key input
                    terminal.draw(|f| ui(f, &mut app))?;
                }
                _ => {} // Ignore other events
            }
        }

        // Regular update processing
        if last_tick.elapsed() >= tick_rate {
            // Execute reconnection if flag is set
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

            // Process WebSocket messages safely
            tokio::select! {
                _ = app.handle_websocket_message() => {}
                _ = time::sleep(Duration::from_millis(10)) => {}
            }

            // Regular redraw
            terminal.draw(|f| ui(f, &mut app))?;

            app.tick();
            last_tick = std::time::Instant::now();
        }
    }

    Ok(())
}
