mod app;
mod ui;
mod audio;
mod util;

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

use app::App;
use ui::{ui, TabView};

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let app = App::new();

    // Run app
    let res = run_app(&mut terminal, app);

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

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('n') => {
                        if app.focused_pane == app::PaneIdentifier::Users {
                            if let Some(user_idx) = app.selected_user {
                                // Ensure the user_idx is valid before trying to access app.users[user_idx]
                                if let Some(user) = app.users.get(user_idx) {
                                    let username = user.name.clone();
                                    app.knock(&username);
                                    // Play knock sound
                                    if let Err(e) = audio::play_knock_sound() {
                                        app.set_error(format!("Failed to play sound: {}", e));
                                    }
                                } else {
                                    // This case should ideally not happen if selected_user is managed correctly
                                    app.set_error("Selected user index is out of bounds.".to_string());
                                }
                            } else {
                                app.set_error("No user selected to knock.".to_string());
                            }
                        } else {
                            // Optionally, provide feedback if 'n' is pressed outside Users pane focus
                            // app.set_error("Switch to Users pane (Tab or 'u') and select a user to knock ('n').".to_string());
                            // For now, do nothing if not in Users pane focus, to avoid spamming errors
                        }
                    },
                    _ => app.handle_key(key),
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = std::time::Instant::now();
        }
    }
}
