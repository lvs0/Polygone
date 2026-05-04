//! All TUI views for POLYGONE.
//!
//! Mirrors the Python Textual TUI (polygone-tui) with 4 tabs:
//! Accueil, Favoris, Services, Paramètres.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::App;
use super::widgets::{
    render_header, render_messages, render_tabs, render_statusbar,
};

// ── Color constants ────────────────────────────────────────────────────────────

const GREEN: Color = Color::Rgb(0x00, 0xFF, 0x88);
const CYAN: Color = Color::Cyan;

// ── View enum ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,  // Tab 1: Accueil
    Favorites,  // Tab 2: Favoris
    Services,   // Tab 3: Services
    Settings,   // Tab 4: Paramètres
}

impl View {
    fn tab_index(self) -> usize {
        match self {
            Self::Dashboard => 0,
            Self::Favorites => 1,
            Self::Services  => 2,
            Self::Settings  => 3,
        }
    }
}

// ── Root render dispatcher ────────────────────────────────────────────────────

/// Root render function — called every frame.
pub fn render_view(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // Layout: header (4) | tabs (1) | content (fill) | messages (6) | statusbar (1)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Min(10),
            Constraint::Length(6),
            Constraint::Length(1),
        ])
        .split(size);

    render_header(frame, chunks[0]);
    render_tabs(frame, chunks[1], app.current_view.tab_index());

    match app.current_view {
        View::Dashboard => render_dashboard(frame, chunks[2]),
        View::Favorites => render_favorites(frame, chunks[2]),
        View::Services  => render_services(frame, chunks[2]),
        View::Settings  => render_settings(frame, chunks[2]),
    }

    render_messages(frame, chunks[3], app);
    render_statusbar(frame, chunks[4]);
}

// ── Dashboard (Accueil) ──────────────────────────────────────────────────────

/// Render the Accueil tab with stat cards, node status, and action buttons.
fn render_dashboard(frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);

    // Left column: NodeStatus + action buttons
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[0]);

    // NodeStatus panel with green "● Actif" indicator
    let node_status_lines = vec![
        Line::from(vec![Span::styled("Nœud", Style::default().fg(Color::White).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("● ", GREEN),
            Span::styled("Actif", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::styled("  depuis 18 min", Color::DarkGray)]),
    ];

    let node_block = Block::default()
        .title("Status")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(CYAN);
    let p = Paragraph::new(node_status_lines).block(node_block);
    frame.render_widget(p, left_chunks[0]);

    // Action buttons panel
    let action_lines = vec![
        Line::from(vec![Span::styled("Options", Style::default().fg(CYAN).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![Span::styled("  [D] Désactiver", Color::LightRed)]),
        Line::from(vec![Span::styled("  [M] Mettre à jour", GREEN)]),
        Line::from(vec![Span::styled("  [R] Redémarrer", GREEN)]),
        Line::from(vec![Span::styled("  [P] Mettre en pause", Color::Yellow)]),
    ];

    let action_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(CYAN);
    let p = Paragraph::new(action_lines).block(action_block);
    frame.render_widget(p, left_chunks[1]);

    // Right column: StatCards
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // StatCard: Solde
    let balance_lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled("Solde", Color::DarkGray)]),
        Line::from(""),
        Line::from(vec![
            Span::styled("10", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::raw("⬡"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("18h de puissance à volonté", Color::DarkGray)]),
    ];

    let balance_block = Block::default()
        .title("Solde")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(CYAN)
        .style(Style::default().bg(Color::Rgb(0x26, 0x26, 0x26)));
    let p = Paragraph::new(balance_lines).block(balance_block);
    frame.render_widget(p, right_chunks[0]);

    // StatCard: Consommation
    let usage_lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled("Consommé actuellement", Color::DarkGray)]),
        Line::from(""),
        Line::from(vec![
            Span::styled("0.1", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" / Min"),
        ]),
    ];

    let usage_block = Block::default()
        .title("Consommation")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(CYAN)
        .style(Style::default().bg(Color::Rgb(0x26, 0x26, 0x26)));
    let p = Paragraph::new(usage_lines).block(usage_block);
    frame.render_widget(p, right_chunks[1]);
}

// ── Favorites (Favoris) ───────────────────────────────────────────────────────

/// Render the Favoris tab.
fn render_favorites(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "Vos contacts et nœuds favoris apparaîtront ici.",
            Color::DarkGray,
        )]),
    ];

    let block = Block::default()
        .title("Favoris")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(CYAN);
    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

// ── Services ─────────────────────────────────────────────────────────────────

/// Render the Services tab.
fn render_services(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![Span::raw("Exploration des services décentralisés Polygone.")]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Aucun service actif pour le moment.",
            Color::DarkGray,
        )]),
    ];

    let block = Block::default()
        .title("Services")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(CYAN);
    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

// ── Settings (Paramètres) ─────────────────────────────────────────────────────

/// Render the Paramètres tab with all system controls.
fn render_settings(frame: &mut Frame, area: Rect) {
    // Split into two columns: controls | status
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    // Left: system controls
    let control_lines = vec![
        Line::from(vec![Span::styled(
            "Gestion du système",
            Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::raw("  [D] Désactiver le node")]),
        Line::from(vec![Span::raw("  [U] Mettre à jour (check GitHub)")]),
        Line::from(vec![Span::raw("  [R] Redémarrer le node")]),
        Line::from(vec![Span::raw("  [P] Mettre en pause")]),
        Line::from(vec![Span::raw("  [X] Supprimer (uninstall)")]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Configuration",
            Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::raw("  [A] MAJ Auto toggle")]),
        Line::from(vec![Span::raw("  [M] MAJ Manuelle")]),
    ];

    let control_block = Block::default()
        .title("Paramètres")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(CYAN);
    let p = Paragraph::new(control_lines).block(control_block);
    frame.render_widget(p, chunks[0]);

    // Right: system status
    let active = true; // TODO: read from app state when node integration exists
    let auto_update = true;

    let status_lines = vec![
        Line::from(vec![Span::styled(
            "État du système",
            Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  État: "),
            if active {
                Span::styled("● Actif", GREEN)
            } else {
                Span::styled("○ Inactif", Color::LightRed)
            },
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  MAJ auto: "),
            if auto_update {
                Span::styled("ON", GREEN)
            } else {
                Span::styled("OFF", Color::LightRed)
            },
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("Polygone TUI v1.0.0", Color::DarkGray)]),
    ];

    let status_block = Block::default()
        .title("Status")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(CYAN);
    let p = Paragraph::new(status_lines).block(status_block);
    frame.render_widget(p, chunks[1]);
}
