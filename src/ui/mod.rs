use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},

    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppState, PaneIdentifier};

#[derive(Clone, Copy, PartialEq)]
pub enum TabView {
    Rooms,
    Users,
    Chat,
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // ターミナルサイズチェック
    if size.width < 20 || size.height < 10 {
        let error_paragraph = Paragraph::new("Terminal too small!\nMinimum: 20x10")
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true });
        f.render_widget(error_paragraph, size);
        return;
    }

    // Main layout: two horizontal panes (left and right)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(size);

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

    // Add controls
    status_text.push_str("\n\nControls:");
    status_text.push_str("\nTab: Switch focus");
    status_text.push_str("\n↑↓: Navigate");
    status_text.push_str("\nEnter: Select");
    status_text.push_str("\nn: Knock (Users)");
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
}
