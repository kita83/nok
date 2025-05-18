use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, AppState};

#[derive(Clone, Copy, PartialEq)]
pub enum TabView {
    Rooms,
    Users,
    Chat,
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();
    
    // Create a horizontal layout for the single line UI
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(size);
    
    // Render different content based on view and state
    match (app.view, app.state) {
        (TabView::Rooms, AppState::Normal) => {
            let room_names: Vec<String> = app.rooms.iter().map(|r| r.name.clone()).collect();
            
            let mut spans = vec![
                Span::styled("nok", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" | "),
                Span::styled("Rooms:", Style::default().fg(Color::Yellow)),
                Span::raw(" "),
            ];
            
            for (i, name) in room_names.iter().enumerate() {
                let style = if i == app.current_room {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                
                spans.push(Span::styled(name.clone(), style));
                if i < room_names.len() - 1 {
                    spans.push(Span::raw(" "));
                }
            }
            
            spans.extend(vec![
                Span::raw(" | "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":rooms "),
                Span::styled("u", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":users "),
                Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":chat "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":input "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":quit"),
            ]);
            
            let line = Line::from(spans);
            let paragraph = Paragraph::new(line);
            f.render_widget(paragraph, chunks[0]);
        },
        
        (TabView::Users, AppState::Normal) => {
            let mut spans = vec![
                Span::styled("nok", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" | "),
                Span::styled("Users:", Style::default().fg(Color::Yellow)),
                Span::raw(" "),
            ];
            
            for (i, user) in app.users.iter().enumerate() {
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
                
                spans.push(Span::styled(status_symbol, Style::default().fg(status_color)));
                spans.push(Span::raw(" "));
                spans.push(Span::styled(user.name.clone(), style));
                
                if i < app.users.len() - 1 {
                    spans.push(Span::raw(" "));
                }
            }
            
            spans.extend(vec![
                Span::raw(" | "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":rooms "),
                Span::styled("u", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":users "),
                Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":chat "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":input "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":quit"),
            ]);
            
            let line = Line::from(spans);
            let paragraph = Paragraph::new(line);
            f.render_widget(paragraph, chunks[0]);
        },
        
        (TabView::Chat, AppState::Normal) => {
            let current_room = &app.rooms[app.current_room].name;
            
            let spans = vec![
                Span::styled("nok", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" | "),
                Span::styled(format!("Chat - {}", current_room), Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":rooms "),
                Span::styled("u", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":users "),
                Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":chat "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":input "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(":quit"),
            ];
            
            let line = Line::from(spans);
            let paragraph = Paragraph::new(line);
            f.render_widget(paragraph, chunks[0]);
        },
        
        (_, AppState::Input) => {
            let spans = vec![
                Span::styled("nok", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" | "),
                Span::styled("Input:", Style::default().fg(Color::Yellow)),
                Span::raw(" "),
                Span::styled(app.input.as_str(), Style::default().fg(Color::White)),
            ];
            
            let line = Line::from(spans);
            let paragraph = Paragraph::new(line);
            f.render_widget(paragraph, chunks[0]);
        },
    }
    
    // Render notification or error if any (single-line approach)
    if let Some(notification) = &app.notification {
        let spans = vec![
            Span::styled("nok", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled("KON KON", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled(notification, Style::default().fg(Color::Yellow)),
        ];
        
        let line = Line::from(spans);
        let paragraph = Paragraph::new(line);
        f.render_widget(paragraph, chunks[0]);
    } else if let Some(error) = &app.error {
        let spans = vec![
            Span::styled("nok", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled("ERROR", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled(error, Style::default().fg(Color::Red)),
        ];
        
        let line = Line::from(spans);
        let paragraph = Paragraph::new(line);
        f.render_widget(paragraph, chunks[0]);
    }
}
