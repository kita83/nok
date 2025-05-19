use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap},
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
            Constraint::Percentage(30), // Rooms
            Constraint::Percentage(30), // Users
            Constraint::Percentage(40), // Messages
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
        })
        .padding(Padding::uniform(1));
    let actual_rooms_content_area = rooms_block.inner(rooms_area);
    f.render_widget(rooms_block, rooms_area);
    let room_items: Vec<ListItem> = app.rooms.iter().enumerate().map(|(i, r)| {
        let style = if app.focused_pane == PaneIdentifier::Rooms && i == app.selected_room_idx {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else if i == app.current_room {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        let content = if i == app.current_room {
            format!("* {}", r.name)
        } else {
            r.name.clone()
        };
        ListItem::new(Span::styled(content, style))
    }).collect();
    let rooms_list = List::new(room_items);
    f.render_widget(rooms_list, actual_rooms_content_area);

    // --- Render Users Pane ---
    let users_block = Block::default()
        .title("Users")
        .borders(Borders::ALL)
        .border_style(if app.focused_pane == PaneIdentifier::Users {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        })
        .padding(Padding::uniform(1));
    let actual_users_content_area = users_block.inner(users_area);
    f.render_widget(users_block, users_area);
    let user_items: Vec<ListItem> = app.users.iter().enumerate().map(|(i, u)| {
        let status_symbol = match u.status {
            crate::app::user::UserStatus::Online => Span::styled("● ", Style::default().fg(Color::Green)),
            crate::app::user::UserStatus::Away => Span::styled("○ ", Style::default().fg(Color::Yellow)),
            crate::app::user::UserStatus::Busy => Span::styled("◆ ", Style::default().fg(Color::Red)),
            crate::app::user::UserStatus::Offline => Span::styled("◇ ", Style::default().fg(Color::Gray)),
        };
        let name_style = if app.focused_pane == PaneIdentifier::Users && app.selected_user == Some(i) {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        ListItem::new(Line::from(vec![status_symbol, Span::styled(&u.name, name_style)]))
    }).collect();
    let users_list = List::new(user_items);
    f.render_widget(users_list, actual_users_content_area);

    // --- Render Messages Pane ---
    let messages_block = Block::default()
        .title("Messages")
        .borders(Borders::ALL)
        .border_style(if app.focused_pane == PaneIdentifier::Messages {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        })
        .padding(Padding::uniform(1));
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
        ListItem::new(Line::from(vec![
            Span::styled(format!("[{}] ", time_str), Style::default().fg(Color::DarkGray)),
            Span::styled(format!("<{}>: ", m.sender), Style::default().fg(Color::Yellow)),
            Span::raw(&m.content),
        ]))
    }).collect();
    let messages_list = List::new(message_items);
    f.render_widget(messages_list, actual_messages_content_area);
    if let Some(input_area) = input_display_area {
        let input_text = format!("> {}", app.input);
        let input_paragraph = Paragraph::new(input_text)
            .style(Style::default().fg(Color::Yellow).bg(Color::Black));
        f.render_widget(input_paragraph, input_area);
    }

    // --- Render ASCII Art Pane (Right Pane) --- Simplified for debugging ---
    let ascii_art_block_instance = Block::default()
        .title("Room Visualizer")
        .borders(Borders::ALL)
        .border_style(if app.focused_pane == PaneIdentifier::AsciiArt {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        });

    let content_area = ascii_art_block_instance.inner(right_pane_area);
    f.render_widget(ascii_art_block_instance, right_pane_area);

    let mut test_lines: Vec<Line> = Vec::new();
    let room_name_for_display = app.rooms.get(app.current_room).map_or("Unknown", |r| &r.name);
    let room_name_line = format!("+---[ {} ]---+", room_name_for_display);
    test_lines.push(Line::from(Span::styled(room_name_line, Style::default().fg(Color::Yellow))));

    let line1_content = "User A (^^)";
    let line2_content = "Mid Content";
    let line3_content = "User B (..)";

    let display_width = content_area.width.saturating_sub(4).max(1) as usize;

    test_lines.push(Line::from(format!("| {:^width$} |", line1_content, width = display_width)));
    test_lines.push(Line::from(format!("| {:^width$} |", line2_content, width = display_width)));
    test_lines.push(Line::from(format!("| {:^width$} |", line3_content, width = display_width)));

    for _ in 0..(content_area.height.saturating_sub(test_lines.len() as u16).saturating_sub(1).max(0)) {
         test_lines.push(Line::from(format!("| {:^width$} |", "", width = display_width)));
    }

    let bottom_border_content_len = room_name_for_display.len() + 8; // Length of content within +---+ e.g. "---[ Room Name ]---"
    let bottom_border_line = format!("+{}+", "-".repeat(bottom_border_content_len));
    test_lines.push(Line::from(Span::styled(bottom_border_line, Style::default().fg(Color::Yellow))));

    let ascii_art_paragraph = Paragraph::new(test_lines)
        .wrap(Wrap { trim: false })
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(ascii_art_paragraph, content_area);

    // --- Display Global Notifications/Errors ---
    let mut bottom_line_occupied = false;
    if app.state == AppState::Input && input_display_area.is_some() {
        bottom_line_occupied = true;
    }
    if !bottom_line_occupied {
        if let Some(notification) = &app.notification {
            let notif_area = Rect {
                x: size.x,
                y: size.height.saturating_sub(1),
                width: size.width,
                height: 1,
            };
            let notif_text = format!("KON KON: {}", notification);
            let notif_paragraph = Paragraph::new(notif_text)
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD).bg(Color::Black));
            f.render_widget(notif_paragraph, notif_area);
            bottom_line_occupied = true;
        } else if let Some(error) = &app.error {
            let error_area = Rect {
                x: size.x,
                y: size.height.saturating_sub(1),
                width: size.width,
                height: 1,
            };
            let error_text = format!("ERROR: {}", error);
            let error_paragraph = Paragraph::new(error_text)
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD).bg(Color::Black));
            f.render_widget(error_paragraph, error_area);
            bottom_line_occupied = true;
        }
    }
    if app.state == AppState::Input && input_display_area.is_none() && !bottom_line_occupied {
         let input_area = Rect {
            x: size.x,
            y: size.height.saturating_sub(1),
            width: size.width,
            height: 1,
        };
        let input_text = format!("Input: {}", app.input);
        let input_paragraph = Paragraph::new(input_text)
            .style(Style::default().fg(Color::Yellow).bg(Color::Black));
        f.render_widget(input_paragraph, input_area);
    }
}
