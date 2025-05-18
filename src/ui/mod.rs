use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &mut App) {
    // Create the layout - optimized for small terminals
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)  // Reduce margin to save space
        .constraints(
            [
                Constraint::Length(1),  // Smaller title area
                Constraint::Min(0),
                Constraint::Length(1),  // Smaller input area
            ]
            .as_ref(),
        )
        .split(f.size());

    // Create the title
    let title = Paragraph::new("nok - Terminal Virtual Office")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Create the main area with rooms and users - optimized for small terminals
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),  // Increase room area
                Constraint::Percentage(40),  // Decrease main content
                Constraint::Percentage(30),  // Increase user area
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Render rooms list
    render_rooms(f, app, main_chunks[0]);

    // Render main content area
    render_main_content(f, app, main_chunks[1]);

    // Render users list
    render_users(f, app, main_chunks[2]);

    // Render input box
    render_input(f, app, chunks[2]);

    // Render notification if any
    if let Some(notification) = &app.notification {
        render_notification(f, notification, f.size());
    }
    
    // Render error if any
    if let Some(error) = &app.error {
        render_error(f, error, f.size());
    }
}

fn render_rooms(f: &mut Frame, app: &App, area: Rect) {
    
    let rooms: Vec<ListItem> = app
        .rooms
        .iter()
        .enumerate()
        .map(|(i, room)| {
            let style = if i == app.current_room {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            // Compact format for small terminals
            ListItem::new(Text::from(Span::styled(
                room.name.clone(),
                style,
            )))
        })
        .collect();

    let rooms_list = List::new(rooms)
        .block(Block::default().borders(Borders::NONE).title("R"))  // Remove borders, shorten title
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">");

    f.render_widget(rooms_list, area);
}

fn render_users(f: &mut Frame, app: &App, area: Rect) {
    
    let users: Vec<ListItem> = app
        .users
        .iter()
        .enumerate()
        .map(|(i, user)| {
            let status_color = match user.status {
                crate::app::user::UserStatus::Online => Color::Green,
                crate::app::user::UserStatus::Away => Color::Yellow,
                crate::app::user::UserStatus::Busy => Color::Red,
                crate::app::user::UserStatus::Offline => Color::Gray,
            };
            
            let status_symbol = match user.status {
                crate::app::user::UserStatus::Online => "●",
                crate::app::user::UserStatus::Away => "○",
                crate::app::user::UserStatus::Busy => "◆",
                crate::app::user::UserStatus::Offline => "◇",
            };
            
            let style = if app.selected_user == Some(i) {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            use ratatui::text::Line;
            let spans = vec![
                Span::styled(
                    format!("{}", status_symbol), // Remove extra spaces
                    Style::default().fg(status_color),
                ),
                Span::styled(format!("{}", user.name), style),
            ];
            let line = Text::from(Line::from(spans));
            ListItem::new(line)
        })
        .collect();

    let users_list = List::new(users)
        .block(Block::default().borders(Borders::NONE).title("U")) // Remove borders, shorten title
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">");

    f.render_widget(users_list, area);
}

fn render_main_content(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::NONE)  // Remove borders to save space
        .title(format!("{}", app.rooms[app.current_room].name));
    
    f.render_widget(block, area);
}

fn render_input(f: &mut Frame, app: &App, area: Rect) {
    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::NONE));  // Remove borders and title to save space
    
    f.render_widget(input, area);
}

fn render_notification(f: &mut Frame, notification: &str, area: Rect) {
    let width = 50;
    let height = 6;
    
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - height) / 2),
                Constraint::Length(height),
                Constraint::Percentage((100 - height) / 2),
            ]
            .as_ref(),
        )
        .split(area);

    let popup_horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - width) / 2),
                Constraint::Percentage(width),
                Constraint::Percentage((100 - width) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1]);

    let popup_area = popup_horizontal_layout[1];
    
    let notification_text = Paragraph::new(notification.to_string())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow))
                .title("Notification"),
        );
    
    f.render_widget(notification_text, popup_area);
}

fn render_error(f: &mut Frame, error: &str, area: Rect) {
    let width = 50;
    let height = 6;
    
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - height) / 2),
                Constraint::Length(height),
                Constraint::Percentage((100 - height) / 2),
            ]
            .as_ref(),
        )
        .split(area);

    let popup_horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - width) / 2),
                Constraint::Percentage(width),
                Constraint::Percentage((100 - width) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1]);

    let popup_area = popup_horizontal_layout[1];
    
    let error_text = Paragraph::new(error.to_string())
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red))
                .title("Error"),
        );
    
    f.render_widget(error_text, popup_area);
}
