//! Parameters view — configure network, ports, paths.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::App;
use super::widgets::section_block;

// ── Parameter descriptor ──────────────────────────────────────────────────────

struct Param {
    key: &'static str,
    value: String,
    description: &'static str,
    editable: bool,
}

impl Param {
    fn new(key: &'static str, value: &str, description: &'static str) -> Self {
        Self { key, value: value.to_string(), description, editable: false }
    }
    fn editable(key: &'static str, value: &str, description: &'static str) -> Self {
        Self { key, value: value.to_string(), description, editable: true }
    }
}

// ── Gather current config ─────────────────────────────────────────────────────

fn get_params() -> Vec<Param> {
    use std::env;

    let key_dir = dirs::data_dir()
        .map(|d| d.join("polygone").join("keys").display().to_string())
        .unwrap_or_else(|| "~/.polygone/keys".to_string());

    let config_dir = dirs::data_dir()
        .map(|d| d.join("polygone").display().to_string())
        .unwrap_or_else(|| "~/.polygone".to_string());

    vec![
        Param::new("version", env!("CARGO_PKG_VERSION"), "Polygone version"),
        Param::new("network.listen_addr", "0.0.0.0:4001", "P2P listen address (v2.0)"),
        Param::new("network.port", "4001", "Legacy relay port"),
        Param::new("network.dht_mode", "memory (Kademlia in v2.0)", "DHT mode"),
        Param::new("node.ram_mb", "256", "Maximum RAM allocation per node"),
        Param::new("node.ttl_sec", "3600", "Ephemeral node TTL in seconds"),
        Param::new("node.threshold", "4", "Shamir reconstruction threshold"),
        Param::new("node.fragment_count", "7", "Total number of fragments (nodes)"),
        Param::new("compute.idle_threshold_sec", "300", "Seconds idle before lending starts"),
        Param::new("compute.max_ram_fraction", "0.50", "Max RAM fraction for lending (0.0–1.0)"),
        Param::new("compute.max_cpu_fraction", "80.0", "Max CPU usage % before pausing lending"),
        Param::new("compute.ollama_enabled", "true", "Enable Ollama inference sharing"),
        Param::new("compute.status_listen", "127.0.0.1:4002", "Compute daemon status endpoint"),
        Param::new("paths.keys", &key_dir, "Key storage directory"),
        Param::new("paths.config", &config_dir, "Polygone config directory"),
        Param::new("paths.data", &config_dir, "Polygone data directory"),
        Param::new("crypto.kem", "ML-KEM-1024 (FIPS 203)", "Key encapsulation mechanism"),
        Param::new("crypto.sign", "Ed25519 (ML-DSA ready)", "Digital signature algorithm"),
        Param::new("crypto.cipher", "AES-256-GCM", "Symmetric encryption"),
        Param::new("crypto.kdf", "BLAKE3 (domain-separated)", "Key derivation function"),
        Param::new("crypto.threshold", "Shamir 4-of-7", "Secret sharing scheme"),
    ]
}

// ── Render ───────────────────────────────────────────────────────────────────

pub fn render_params(frame: &mut Frame, area: Rect, _app: &App) {
    let params = get_params();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let mid = params.len() / 2;
    render_param_table(frame, chunks[0], &params[..mid]);
    render_param_table(frame, chunks[1], &params[mid..]);

    // Legend
    let legend = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(50),
        ])
        .split(area);

    let legend_lines = vec![
        Line::from(vec![
            Span::styled("  *", Style::default().fg(Color::Yellow)),
            Span::styled(" = editable via config file  ", Style::default().fg(Color::DarkGray)),
            Span::styled("Config: ", Style::default().fg(Color::DarkGray)),
            Span::styled("~/.polygone/polygone.toml", Style::default().fg(Color::Cyan)),
        ]),
    ];
    let p = Paragraph::new(legend_lines);
    frame.render_widget(p, legend[1]);
}

fn render_param_table(frame: &mut Frame, area: Rect, params: &[Param]) {
    let lines: Vec<Line> = params.iter().map(|p| {
        let key_style = Style::default().fg(Color::Cyan);
        let val_style = if p.editable {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        let edit_mark = if p.editable { "*" } else { " " };
        Line::from(vec![
            Span::styled(format!("  {}{}", edit_mark, p.key), key_style),
        ])
    }).collect();

    let block = Block::default()
        .title(Span::styled(" Parameters ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .style(Style::default().bg(Color::Reset));

    let p = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(p, area);

    // Draw values on top of the block using a second render
    // (ratatui doesn't support aligned columns natively without Table)
    // For a cleaner approach, use the values overlay approach:
    let value_lines: Vec<Line> = params.iter().map(|p| {
        let val_style = if p.editable {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        Line::from(vec![
            Span::raw("  "),
            Span::styled(truncate(&p.value, 28), val_style),
            Span::styled("  ", Style::default().fg(Color::DarkGray)),
        ])
    }).collect();

    // Values in a slightly transparent overlay
    let vp = Paragraph::new(value_lines)
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(vp, area);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max.saturating_sub(1)])
    } else {
        s.to_string()
    }
}
