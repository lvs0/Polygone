//! Reusable TUI widgets for POLYGONE.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::{App, MessageKind};

// ── Header ────────────────────────────────────────────────────────────────────

/// Render the top banner with the POLYGONE logo.
pub fn render_header(frame: &mut Frame, area: Rect) {
    let logo = vec![
        Line::from(vec![
            Span::styled("⬡ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("POLYGONE", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(" v1.0.0", Style::default().fg(Color::DarkGray)),
            Span::styled("  —  ", Style::default().fg(Color::DarkGray)),
            Span::styled("ML-KEM-1024", Style::default().fg(Color::Cyan)),
            Span::styled(" · ", Style::default().fg(Color::DarkGray)),
            Span::styled("Shamir 4-of-7", Style::default().fg(Color::Cyan)),
            Span::styled(" · ", Style::default().fg(Color::DarkGray)),
            Span::styled("AES-256-GCM", Style::default().fg(Color::Cyan)),
            Span::styled(" · ", Style::default().fg(Color::DarkGray)),
            Span::styled("BLAKE3", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled(
                "L'information n'existe pas. Elle traverse.",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            ),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Reset));

    let p = Paragraph::new(logo).block(block);
    frame.render_widget(p, area);
}

// ── Tab bar ───────────────────────────────────────────────────────────────────

/// Render the view navigation tab bar.
pub fn render_tabs(frame: &mut Frame, area: Rect, active_idx: usize) {
    let tabs = [
        ("[1] Dashboard", 0),
        ("[2] Keygen",    1),
        ("[3] Send",      2),
        ("[4] Receive",   3),
        ("[5] Node",      4),
        ("[6] Self-Test", 5),
        ("[?] Help",      6),
    ];

    let spans: Vec<Span> = tabs.iter().flat_map(|(label, idx)| {
        let style = if *idx == active_idx {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        vec![
            Span::styled(format!(" {label} "), style),
            Span::styled("│", Style::default().fg(Color::DarkGray)),
        ]
    }).collect();

    let line = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::Reset));
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
        Span::styled("1-6", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(":navigate  ", Style::default().fg(Color::DarkGray)),
        Span::styled("?", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(":help  ", Style::default().fg(Color::DarkGray)),
        Span::styled("Ctrl+C", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(":force quit", Style::default().fg(Color::DarkGray)),
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
