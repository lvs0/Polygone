//! Services view — available Polygone modules (Drive, Hide, Compute, Petals).

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Table, Row},
    Frame,
};

use super::app::App;
use super::widgets::section_block;

// ── Module descriptor ─────────────────────────────────────────────────────────

struct Module {
    name: &'static str,
    icon: &'static str,
    description: &'static str,
    status: ModuleStatus,
    version: &'static str,
}

#[derive(Clone, Copy)]
enum ModuleStatus {
    Available,
    Running,
    ComingSoon,
    Disabled,
}

impl ModuleStatus {
    fn label(&self) -> &'static str {
        match self {
            Self::Available  => "Available",
            Self::Running    => "Running",
            Self::ComingSoon => "v2.0",
            Self::Disabled   => "Disabled",
        }
    }
    fn color(&self) -> Color {
        match self {
            Self::Available  => Color::Green,
            Self::Running    => Color::Cyan,
            Self::ComingSoon => Color::Yellow,
            Self::Disabled   => Color::DarkGray,
        }
    }
}

fn get_modules(app: &App) -> Vec<Module> {
    // We don't have actual runtime status here, but we show what's available.
    // In a real integration these would be populated from runtime state.
    vec![
        Module {
            name: "Drive",
            icon: "▣",
            description: "Encrypted distributed file storage. Files are sharded with Shamir and stored across the network. End-to-end encrypted, zero-knowledge.",
            status: ModuleStatus::ComingSoon,
            version: "v2.0",
        },
        Module {
            name: "Hide",
            icon: "◈",
            description: "Traffic obfuscation and routing through ephemeral nodes. No observer can prove a connection was made. Metadata resistant.",
            status: ModuleStatus::ComingSoon,
            version: "v2.0",
        },
        Module {
            name: "Compute",
            icon: "⬡",
            description: "Lend idle CPU/RAM to the network. Earn credits. Integrate with Ollama for local inference sharing. Pauses when you're active.",
            status: ModuleStatus::Available,
            version: "v1.0",
        },
        Module {
            name: "Petals",
            icon: "✿",
            description: "Distributed ML inference through a peer-to-peer network of contributors. Run large models collaboratively without central servers.",
            status: ModuleStatus::ComingSoon,
            version: "v2.0",
        },
    ]
}

// ── Render ───────────────────────────────────────────────────────────────────

pub fn render_services(frame: &mut Frame, area: Rect, app: &App) {
    let modules = get_modules(app);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(8),
        ])
        .split(area);

    // Header
    let header_lines = vec![
        Line::from(vec![
            Span::styled(" Polygone Services ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("—  network modules and extensions", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Run ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone compute start", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" to begin lending power. Use ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone compute status", Style::default().fg(Color::Yellow)),
            Span::styled(" to view stats.", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    let p = Paragraph::new(header_lines).block(section_block("Services"));
    frame.render_widget(p, chunks[0]);

    // Module cards
    let card_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    let left_modules: Vec<&Module> = modules.iter().filter(|m| {
        matches!(m.status, ModuleStatus::Available | ModuleStatus::ComingSoon)
    }).take(2).collect();
    let right_modules: Vec<&Module> = modules.iter().filter(|m| {
        matches!(m.status, ModuleStatus::Running | ModuleStatus::Disabled)
    }).take(2).collect();

    render_module_card(frame, card_chunks[0], left_modules.get(0).copied());
    if left_modules.len() > 1 {
        let sub_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(card_chunks[0]);
        render_module_card(frame, sub_chunks[0], left_modules.get(0).copied());
        render_module_card(frame, sub_chunks[1], left_modules.get(1).copied());
    }

    if right_modules.len() == 2 {
        let sub_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(card_chunks[1]);
        render_module_card(frame, sub_chunks[0], right_modules.get(0).copied());
        render_module_card(frame, sub_chunks[1], right_modules.get(1).copied());
    } else {
        render_module_card(frame, card_chunks[1], right_modules.get(0).copied());
    }

    // Compute daemon quick-panel
    render_compute_panel(frame, chunks[2], app);
}

fn render_module_card(frame: &mut Frame, area: Rect, module: Option<&Module>) {
    let Some(m) = module else {
        let p = Paragraph::new(vec![Line::from("")])
            .block(section_block(""));
        frame.render_widget(p, area);
        return;
    };

    let status_color = m.status.color();
    let status_style = Style::default().fg(status_color).add_modifier(Modifier::BOLD);

    let lines = vec![
        Line::from(vec![
            Span::styled(format!("{} ", m.icon), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(m.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(format!("[{}]", m.status.label()), status_style),
        ]),
        Line::from(vec![
            Span::styled(format!("  {}", m.description), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Version: ", Style::default().fg(Color::DarkGray)),
            Span::styled(m.version, Style::default().fg(Color::Cyan)),
        ]),
    ];

    let p = Paragraph::new(lines)
        .block(Block::default()
            .title(Span::styled(format!(" {} ", m.name), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if matches!(m.status, ModuleStatus::Available) { Color::Blue } else { Color::DarkGray })))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p, area);
}

fn render_compute_panel(frame: &mut Frame, area: Rect, app: &App) {
    // Show a quick compute status summary
    let compute_status = get_compute_summary();

    let spinner = ["◐", "◓", "◑", "◒"];
    let spin = spinner[(app.tick / 3) as usize % 4];

    let lines = vec![
        Line::from(vec![
            Span::styled("⬡ Compute", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("  Power Lending — Ollama Inference Sharing", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Status    ", Style::default().fg(Color::DarkGray)),
            Span::styled(compute_status.state_label, Style::default().fg(compute_status.state_color).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            if compute_status.is_lending {
                Span::styled(spin, Style::default().fg(Color::Green))
            } else {
                Span::styled("○", Style::default().fg(Color::DarkGray))
            },
        ]),
        Line::from(vec![
            Span::styled("  Idle      ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:.0}s since last activity", compute_status.idle_sec), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  CPU usage ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:.0}%", compute_status.cpu), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  RAM usage ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}", compute_status.ram_str), Style::default().fg(Color::White)),
            Span::styled(format!(" ({:.0}%)", compute_status.ram_pct * 100.0), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Commands  ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone compute start | stop | status", Style::default().fg(Color::Yellow)),
        ]),
    ];

    let p = Paragraph::new(lines)
        .block(Block::default()
            .title(Span::styled(" Polygone-Compute ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Blue)))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p, area);
}

// ── Compute summary (polled from daemon) ─────────────────────────────────────

use std::sync::atomic::{AtomicBool, Ordering};

/// Summary of compute daemon state for TUI display.
#[derive(Debug, Clone)]
struct ComputeSummary {
    state_label: String,
    state_color: Color,
    is_lending: bool,
    idle_sec: f64,
    cpu: f32,
    ram_str: String,
    ram_pct: f32,
}

fn get_compute_summary() -> ComputeSummary {
    use std::process::Command;

    // Try running `polygone compute status` and parse output
    let output = Command::new("polygone")
        .args(["compute", "status"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            parse_compute_status(&stdout)
        }
        _ => ComputeSummary {
            state_label: "Stopped".to_string(),
            state_color: Color::DarkGray,
            is_lending: false,
            idle_sec: 0.0,
            cpu: 0.0,
            ram_str: "—".to_string(),
            ram_pct: 0.0,
        },
    }
}

fn parse_compute_status(s: &str) -> ComputeSummary {
    let mut state_label = "Unknown".to_string();
    let mut state_color = Color::DarkGray;
    let mut is_lending = false;
    let mut idle_sec = 0.0;
    let mut cpu = 0.0;
    let mut ram_str = "—".to_string();
    let mut ram_pct = 0.0;

    for line in s.lines() {
        let line = line.trim();
        if line.starts_with("  Status") || line.starts_with("Status") {
            let val = line.split(':').nth(1).unwrap_or("").trim();
            state_label = val.to_string();
            is_lending = val.contains("Lending") || val.contains("Ollama");
            state_color = if is_lending { Color::Green } else if val.contains("Paused") { Color::Yellow } else { Color::DarkGray };
        }
        if line.starts_with("  Idle") || line.starts_with("Idle") {
            if let Some(n) = extract_number(line) {
                idle_sec = n;
            }
        }
        if line.starts_with("  CPU") || line.starts_with("CPU") {
            if let Some(n) = extract_number(line) {
                cpu = n as f32;
            }
        }
        if line.contains("RAM") && line.contains("/") {
            ram_str = line.split(':').nth(1).unwrap_or("—").trim().to_string();
            if let Some(pct) = line.split('(').nth(1) {
                if let Some(p) = pct.split('%').next() {
                    ram_pct = p.trim().parse().unwrap_or(0.0) / 100.0;
                }
            }
        }
    }

    ComputeSummary { state_label, state_color, is_lending, idle_sec, cpu, ram_str, ram_pct }
}

fn extract_number(s: &str) -> Option<f64> {
    s.chars()
        .skip_while(|c| !c.is_ascii_digit() && *c != '-')
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect::<String>()
        .parse()
        .ok()
}
