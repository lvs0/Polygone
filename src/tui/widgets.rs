//! Reusable TUI widgets for POLYGONE v2 — inspired by dashboard mockups.

use std::collections::HashSet;


use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::App;

// ── Header v2 ────────────────────────────────────────────────────────────────

/// Render the top banner — minimal dark bar with logo + live status line.
pub fn render_header(frame: &mut Frame, area: Rect) {
    let logo = vec![
        Line::from(vec![
            Span::styled("⬡ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("POLYGONE", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(" v1.0.0", Style::default().fg(Color::DarkGray)),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled("Online", Style::default().fg(Color::Green)),
            Span::styled("  │  Nodes: ", Style::default().fg(Color::DarkGray)),
            Span::styled("7/7", Style::default().fg(Color::Cyan)),
            Span::styled("  │  Traffic: ", Style::default().fg(Color::DarkGray)),
            Span::styled("1.2 Mbps", Style::default().fg(Color::Cyan)),
        ]),
    ];
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Black));
    let p = Paragraph::new(logo).block(block);
    frame.render_widget(p, area);
}

// ── Tab bar v2 — pill-style ──────────────────────────────────────────────────

/// Render the pill-shaped tab navigation bar (Acceuil / Favoris / Services / Paramètres).
pub fn render_tabs(frame: &mut Frame, area: Rect, active_idx: usize) {
    let tabs = [
        ("[1] Acceuil",    0),
        ("[2] Favoris",    1),
        ("[3] Services",   2),
        ("[4] Parametres", 3),
    ];

    let spans: Vec<Span> = tabs.iter().flat_map(|(label, idx)| {
        let style = if *idx == active_idx {
            Style::default().fg(Color::White).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::White)
                .bg(Color::Reset)
                .add_modifier(Modifier::BOLD)
        };
        vec![
            Span::styled(format!(" {label} "), style),
            Span::styled("  ", Style::default().fg(Color::Reset)),
        ]
    }).collect();

    let line = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::Black));
    frame.render_widget(line, area);
}

// ── Message log ───────────────────────────────────────────────────────────────

/// Render the bottom message log.
pub fn render_messages(frame: &mut Frame, area: Rect, app: &App) {
    let height = area.height.saturating_sub(2) as usize;
    let msgs = &app.messages;
    let start = msgs.len().saturating_sub(height);

    let lines: Vec<Line> = msgs[start..].iter().map(|(kind, msg)| {
        Line::from(vec![
            Span::styled(format!(" {} ", kind.symbol()), kind.style()),
            Span::styled(msg.clone(), Style::default().fg(Color::White)),
        ])
    }).collect();

    let block = Block::default()
        .title(Span::styled(" Log ", Style::default().fg(Color::DarkGray)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let p = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p, area);
}

// ── Status bar ────────────────────────────────────────────────────────────────

/// Render the bottom status bar.
pub fn render_statusbar(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(":quit  ", Style::default().fg(Color::DarkGray)),
        Span::styled("1-4", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(":navigate  ", Style::default().fg(Color::DarkGray)),
        Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(":select  ", Style::default().fg(Color::DarkGray)),
        Span::styled("ENTER", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(":confirm", Style::default().fg(Color::DarkGray)),
    ]);
    let p = Paragraph::new(line)
        .style(Style::default().bg(Color::Reset));
    frame.render_widget(p, area);
}

// ── Section block ─────────────────────────────────────────────────────────────

/// A titled, bordered block for content sections.
pub fn section_block(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
}

// ── Node grid widget ──────────────────────────────────────────────────────────

/// Render a visual grid of 7 ephemeral nodes with status indicators.
pub fn render_node_grid(frame: &mut Frame, area: Rect, active_count: usize, tick: u64) {
    let total = 7usize;
    let threshold = 4usize;

    let spinner = ["◐", "◓", "◑", "◒"];
    let spin = spinner[(tick / 3) as usize % 4];

    let mut lines = vec![
        Line::from(vec![
            Span::styled("  Ephemeral nodes (7)  —  threshold 4-of-7", 
                Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
    ];

    // Node indicators
    let mut node_spans = vec![Span::raw("  ")];
    for i in 0..total {
        let (sym, style) = if i < active_count {
            if active_count >= threshold {
                (spin, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                ("◉", Style::default().fg(Color::Yellow))
            }
        } else {
            ("○", Style::default().fg(Color::DarkGray))
        };
        node_spans.push(Span::styled(format!("{sym} "), style));
    }
    lines.push(Line::from(node_spans));
    lines.push(Line::from(""));

    // Threshold bar
    let filled = (active_count * 20) / total;
    let bar: String = (0..20).map(|i| if i < filled { '█' } else { '░' }).collect();
    let bar_color = if active_count >= threshold { Color::Green } else { Color::Yellow };
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(bar, Style::default().fg(bar_color)),
        Span::styled(
            format!("  {active_count}/{total} active"),
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    let block = section_block("Network Nodes");
    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

// ── Modal pause widget ───────────────────────────────────────────────────────

/// Render a dark overlay modal with time-selection buttons.
pub fn render_pause_modal(frame: &mut Frame, area: Rect) {
    let durations = ["10 Min", "30 Min", "1 Heure", "2 Heures", "4 Heures", "8 Heures"];

    let lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled("Jusqu'a :", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Reactivation :", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
    ];

    // Build button grid — 3 columns
    let mut btn_lines: Vec<Line> = vec![];
    for chunk in durations.chunks(3) {
        let mut line_spans = vec![Span::raw("  ")];
        for d in chunk {
            let _btn = format!("[{d}]");
            line_spans.push(Span::styled(
                format!(" {:>10} ", d),
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ));
            line_spans.push(Span::styled("  ", Style::default().fg(Color::Reset)));
        }
        btn_lines.push(Line::from(line_spans));
        btn_lines.push(Line::from(""));
    }

    let all_lines: Vec<Line> = lines.into_iter().chain(btn_lines).collect();

    let block = Block::default()
        .title(Span::styled(" Pause Noeud ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let p = Paragraph::new(all_lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p, area);
}

// ── Favorites view ────────────────────────────────────────────────────────────

/// Render the 3-column favorites layout (Serveur | Drive | Storage).
pub fn render_favorites_grid(frame: &mut Frame, area: Rect, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(area);

    // Column 1: Serveur
    let serveur_lines = vec![
        Line::from(vec![
            Span::styled(" Serveur", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  URL :", Style::default().fg(Color::DarkGray))]),
        Line::from(vec![Span::styled("  Port :", Style::default().fg(Color::DarkGray))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Ouvrir le terminal]", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  [Redeployer]", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled("[Arrete]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  [Voir logs]", Style::default().fg(Color::Green)),
        ]),
    ];
    let p1 = Paragraph::new(serveur_lines)
        .block(section_block(""))
        .wrap(ratatui::widgets::Wrap { trim: true });

    // Column 2: Drive
    let drive_lines = vec![
        Line::from(vec![
            Span::styled(" Drive", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Ouvrir l'interface web]", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  10 GB / ", Style::default().fg(Color::White)),
            Span::styled("/ ∞", Style::default().fg(Color::Cyan)),
        ]),
    ];
    let p2 = Paragraph::new(drive_lines)
        .block(section_block(""))
        .wrap(ratatui::widgets::Wrap { trim: true });

    // Column 3: Empty
    let p3 = Paragraph::new(vec![
        Line::from(vec![Span::styled("", Style::default().fg(Color::Reset))]),
    ])
    .block(section_block(""))
    .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p1, chunks[0]);
    frame.render_widget(p2, chunks[1]);
    frame.render_widget(p3, chunks[2]);
}

// ── Service list widget ──────────────────────────────────────────────────────

/// Render the left-side service list (8 items) with favorites toggle.
pub fn render_service_list(frame: &mut Frame, area: Rect, services: &[&str], selected: usize, favorites: &HashSet<String>) {
    let lines: Vec<Line> = services.iter().enumerate().map(|(i, name)| {
        let marker = if i == selected { "▶" } else { " " };
        let is_favorite = favorites.contains(*name);
        let favorite_marker = if is_favorite { "★" } else { "  " };
        let style = if i == selected {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        Line::from(vec![
            Span::styled(format!("{marker} "), style),
            Span::styled(favorite_marker, Style::default().fg(Color::Yellow)),
            Span::styled(" ", style),
            Span::styled(*name, style),
        ])
    }).collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

// ── Balance card ─────────────────────────────────────────────────────────────

/// Render the POLY balance card (top-right in dashboard).
pub fn render_balance_card(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" Solde :", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  10 ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("⬡ POLY", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  18h de puissance a volonte", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    let block = Block::default()
        .title(Span::styled(" Solde ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue));
    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

// ── Quick actions widget ─────────────────────────────────────────────────────

/// Render the quick action buttons (Self-Test | Services | Node Control | Favorites).
pub fn render_quick_actions(frame: &mut Frame, area: Rect) {
    let actions = ["Self-Test", "Services", "Node Control", "Favoris"];
    let highlights = [Color::Cyan, Color::Cyan, Color::Cyan, Color::Yellow];

    let spans: Vec<Span> = actions.iter().zip(highlights.iter()).flat_map(|(label, color)| {
        vec![
            Span::styled(
                format!(" ▶ {label} "),
                Style::default()
                    .fg(Color::Black)
                    .bg(*color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ", Style::default().fg(Color::Reset)),
        ]
    }).collect();

    let line = Paragraph::new(Line::from(spans));
    frame.render_widget(line, area);
}

// ── Helpers ─────────────────────────────────────────────────────────────────

use ratatui::layout::{Constraint, Direction, Layout};
