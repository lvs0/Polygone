//! Main TUI application loop.

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
            // View navigation (1-4 for tabs)
            KeyCode::Char('1') => self.current_view = View::Dashboard,
            KeyCode::Char('2') => self.current_view = View::Favorites,
            KeyCode::Char('3') => self.current_view = View::Services,
            KeyCode::Char('4') => self.current_view = View::Settings,
            // System actions (when in Settings or Dashboard)
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.push_msg(MessageKind::Warn, "Node désactivé (action simulée)");
            }
            KeyCode::Char('u') | KeyCode::Char('U') => {
                self.push_msg(MessageKind::Info, "Mise à jour en cours... check GitHub releases");
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.push_msg(MessageKind::Info, "Redémarrage du node...");
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                self.push_msg(MessageKind::Warn, "Node en pause");
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                self.push_msg(MessageKind::Error, "Suppression du node (simulé)");
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.push_msg(MessageKind::Info, "MAJ auto toggled");
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                self.push_msg(MessageKind::Info, "MAJ manuelle — vérifier les releases GitHub");
            }
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
