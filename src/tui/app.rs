//! Main TUI application loop.

use std::collections::HashSet;
use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use super::views::{View, render_view};
use super::favorites::{load_favorites, save_favorites};

/// Global application state.
pub struct App {
    /// Which view is currently displayed.
    pub current_view: View,
    /// Whether the user requested quit.
    pub should_quit: bool,
    /// Status/log messages shown in the footer.
    pub messages: Vec<(MessageKind, String)>,
    /// Tick counter (incremented each ~100ms for animations).
    pub tick: u64,
    /// Whether the pause modal is shown (Dashboard sub-action).
    pub show_pause_modal: bool,
    /// Favorites state (Polygone services).
    pub favorites: HashSet<String>,
    /// Currently selected service index (for favorites toggle).
    pub selected_service: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    Info,
    Success,
    Error,
    Warn,
}

impl MessageKind {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Info    => "ℹ",
            Self::Success => "✔",
            Self::Error   => "✖",
            Self::Warn    => "⚠",
        }
    }

    pub fn style(self) -> ratatui::style::Style {
        use ratatui::style::{Color, Style, Modifier};
        match self {
            Self::Info    => Style::default().fg(Color::Cyan),
            Self::Success => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            Self::Error   => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            Self::Warn    => Style::default().fg(Color::Yellow),
        }
    }
}

impl App {
    pub fn new(initial_view: View) -> Self {
        Self {
            current_view: initial_view,
            should_quit: false,
            messages: vec![
                (MessageKind::Info, "Welcome to POLYGONE v1.0.0 — post-quantum ephemeral network".into()),
            ],
            tick: 0,
            show_pause_modal: false,
            favorites: load_favorites(),
            selected_service: 0,
        }
    }

    pub fn push_msg(&mut self, kind: MessageKind, msg: impl Into<String>) {
        let s = msg.into();
        if self.messages.len() >= 100 {
            self.messages.remove(0);
        }
        self.messages.push((kind, s));
    }

    pub fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Global shortcuts
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
                return;
            }
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
                return;
            }
            // View navigation
            KeyCode::Char('f') => {
                if self.current_view == View::Services {
                    let services = [
                        "Polygone",
                        "Polygone-Brain",
                        "Polygone-CLI",
                        "Polygone-Drive",
                        "Polygone-Hide",
                        "Polygone-Petals",
                        "Polygone-Shell",
                        "Polygone-Server",
                    ];
                    if let Some(selected) = services.get(self.selected_service) {
                        if self.favorites.contains(*selected) {
                            self.favorites.remove(*selected);
                            self.push_msg(MessageKind::Info, format!("Removed {} from favorites", selected));
                        } else {
                            self.favorites.insert(selected.to_string());
                            self.push_msg(MessageKind::Info, format!("Added {} to favorites", selected));
                        }
                        save_favorites(&self.favorites);
                    }
                }
            }
            KeyCode::Char('1') => { self.current_view = View::Dashboard; self.show_pause_modal = false; }
            KeyCode::Char('2') => { self.current_view = View::Favoris; self.show_pause_modal = false; }
            KeyCode::Char('3') => { self.current_view = View::Services; self.show_pause_modal = false; }
            KeyCode::Char('4') => { self.current_view = View::Params; self.show_pause_modal = false; }
            KeyCode::Char('5') => { self.current_view = View::Keygen; self.show_pause_modal = false; }
            KeyCode::Char('6') => { self.current_view = View::Send; self.show_pause_modal = false; }
            KeyCode::Char('7') => { self.current_view = View::Receive; self.show_pause_modal = false; }
            KeyCode::Char('8') => { self.current_view = View::Node; self.show_pause_modal = false; }
            KeyCode::Char('9') => { self.current_view = View::SelfTest; self.show_pause_modal = false; }
            _ => {}
        }
    }
}

/// Initialize the terminal, run the TUI, and restore the terminal on exit.
pub fn run_tui(initial_view: View) -> io::Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(initial_view);

    // Main loop
    loop {
        terminal.draw(|frame| {
            render_view(frame, &app);
        })?;

        // Poll with 100ms timeout for smooth animations
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code, key.modifiers);
                }
            }
        }

        app.tick = app.tick.wrapping_add(1);

        if app.should_quit {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
