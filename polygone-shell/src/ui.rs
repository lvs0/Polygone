use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct AppState {
    pub peer_id: String,
    pub session_id: String,
    pub active_tab: usize,
    pub logs: Vec<String>,
}

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ].as_ref())
        .split(f.size());

    // --- Header ---
    let header = Paragraph::new(format!(" ⬡ POLYGONE-SHELL | Peer: {} | Session: {}", app.peer_id, app.session_id))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(header, chunks[0]);

    // --- Body ---
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[1]);

    // Main Content (Dashboard)
    let dashboard = Block::default()
        .title(" [ ACTIVITÉ RÉSEAU ] ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));
    f.render_widget(dashboard, body_chunks[0]);

    // Logs / Activity Feed
    let logs_text = app.logs.iter().rev().cloned().collect::<Vec<_>>().join("\n");
    let logs = Paragraph::new(logs_text)
        .block(Block::default().title(" [ LOGS ] ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green));
    f.render_widget(logs, body_chunks[1]);

    // --- Footer ---
    let footer = Paragraph::new(" [Q] Quitter | [TAB] Switch Modules | [D] Drive | [P] Petals ")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
