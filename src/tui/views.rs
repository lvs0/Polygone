//! All TUI views for POLYGONE.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::App;
use super::widgets::{
    render_header, render_messages, render_node_grid,
    render_statusbar, render_tabs, section_block,
};
use crate::keys;

// ── View enum ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,
    Keygen,
    Send,
    Receive,
    Node,
    SelfTest,
    Services,
    Params,
    Help,
}

impl View {
    fn tab_index(self) -> usize {
        match self {
            Self::Dashboard => 0,
            Self::Keygen    => 1,
            Self::Send      => 2,
            Self::Receive   => 3,
            Self::Node      => 4,
            Self::SelfTest  => 5,
            Self::Services  => 6,
            Self::Params    => 7,
            Self::Help      => 8,
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
        View::Dashboard => render_dashboard(frame, chunks[2], app),
        View::Keygen    => render_keygen(frame, chunks[2], app),
        View::Send      => render_send(frame, chunks[2], app),
        View::Receive   => render_receive(frame, chunks[2], app),
        View::Node      => render_node(frame, chunks[2], app),
        View::SelfTest  => render_selftest(frame, chunks[2], app),
        View::Services  => render_services(frame, chunks[2], app),
        View::Params    => render_params(frame, chunks[2], app),
        View::Help      => render_help(frame, chunks[2]),
    }

    render_messages(frame, chunks[3], app);
    render_statusbar(frame, chunks[4]);
}

// ── Dashboard ─────────────────────────────────────────────────────────────────

fn render_dashboard(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[0]);

    // Status panel
    let key_dir = keys::default_key_dir();
    let keys_status = if keys::keypair_exists(&key_dir) {
        ("✔ Keypair found", Color::Green)
    } else {
        ("✖ No keypair — press [2] to generate", Color::Red)
    };

    let spinner = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
    let spin = spinner[app.tick as usize % 8].to_string();

    let status_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Status       ", Style::default().fg(Color::DarkGray)),
            Span::styled("● Online", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Version      ", Style::default().fg(Color::DarkGray)),
            Span::styled("v1.0.0", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Keys         ", Style::default().fg(Color::DarkGray)),
            Span::styled(keys_status.0, Style::default().fg(keys_status.1)),
        ]),
        Line::from(vec![
            Span::styled("  Key dir      ", Style::default().fg(Color::DarkGray)),
            Span::styled(key_dir.display().to_string(), Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Sessions     ", Style::default().fg(Color::DarkGray)),
            Span::styled("0 active", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(format!("  Network  {}  ", spin), Style::default().fg(Color::DarkGray)),
            Span::styled("local mode (P2P in v2.0)", Style::default().fg(Color::Yellow)),
        ]),
    ];

    let p = Paragraph::new(status_lines).block(section_block("Status"));
    frame.render_widget(p, left[0]);

    // Crypto stack panel
    let crypto_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  KEM          ", Style::default().fg(Color::DarkGray)),
            Span::styled("ML-KEM-1024 (FIPS 203)", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  Signature    ", Style::default().fg(Color::DarkGray)),
            Span::styled("Ed25519 (ML-DSA ready)", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  Symmetric    ", Style::default().fg(Color::DarkGray)),
            Span::styled("AES-256-GCM", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  KDF          ", Style::default().fg(Color::DarkGray)),
            Span::styled("BLAKE3 (domain-separated)", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  Threshold    ", Style::default().fg(Color::DarkGray)),
            Span::styled("Shamir 4-of-7", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  unsafe code  ", Style::default().fg(Color::DarkGray)),
            Span::styled("FORBIDDEN (#[forbid(unsafe_code)])", Style::default().fg(Color::Green)),
        ]),
    ];

    let p = Paragraph::new(crypto_lines).block(section_block("Crypto Stack"));
    frame.render_widget(p, left[1]);

    // Node grid (right side)
    let active_nodes = 7usize; // In local mode all 7 are conceptually alive
    render_node_grid(frame, chunks[1], active_nodes, app.tick);
}

// ── Keygen ────────────────────────────────────────────────────────────────────

fn render_keygen(frame: &mut Frame, area: Rect, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let key_dir = keys::default_key_dir();
    let exists = keys::keypair_exists(&key_dir);

    let info_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Generates a post-quantum keypair and saves it to disk with "),
            Span::styled("chmod 600", Style::default().fg(Color::Yellow)),
            Span::raw("."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Key directory  ", Style::default().fg(Color::DarkGray)),
            Span::styled(key_dir.display().to_string(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  KEM key        ", Style::default().fg(Color::DarkGray)),
            Span::styled("ML-KEM-1024 — 1568-byte public key", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  Sign key       ", Style::default().fg(Color::DarkGray)),
            Span::styled("Ed25519 — 32-byte public key", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Keypair status ", Style::default().fg(Color::DarkGray)),
            if exists {
                Span::styled("✔ Keypair exists", Style::default().fg(Color::Green))
            } else {
                Span::styled("✖ No keypair yet", Style::default().fg(Color::Red))
            },
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  → Run: polygone keygen",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "    Add --output <path> to use a custom directory.",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    ];

    let p = Paragraph::new(info_lines).block(section_block("Key Generation"));
    frame.render_widget(p, chunks[0]);

    // Show public key preview if it exists
    let preview_lines = if exists {
        if let Ok(pk) = keys::read_kem_pk(&key_dir) {
            let hex = pk.to_hex();
            let preview = &hex[..64];
            vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  KEM public key (first 64 hex chars):", Style::default().fg(Color::DarkGray)),
                ]),
                Line::from(vec![
                    Span::styled(format!("  {preview}…"), Style::default().fg(Color::Cyan)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Share your KEM public key with anyone who wants to send you a message.", 
                        Style::default().fg(Color::DarkGray)),
                ]),
                Line::from(vec![
                    Span::styled("  Keep your secret key offline. It cannot be recovered if lost.",
                        Style::default().fg(Color::Yellow)),
                ]),
            ]
        } else {
            vec![Line::from(vec![Span::styled("  (could not read key file)", Style::default().fg(Color::Red))])]
        }
    } else {
        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  No keypair generated yet.", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::styled("  Run `polygone keygen` to create one.", Style::default().fg(Color::DarkGray)),
            ]),
        ]
    };

    let p = Paragraph::new(preview_lines).block(section_block("Public Key Preview"));
    frame.render_widget(p, chunks[1]);
}

// ── Send ──────────────────────────────────────────────────────────────────────

fn render_send(frame: &mut Frame, area: Rect, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    let usage_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Encrypt a message and distribute it across 7 ephemeral nodes."),
        ]),
        Line::from(vec![
            Span::raw("  Any 4 fragments reconstruct the message. No node sees the full content."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Local demo (no network required):  ", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(
                "    polygone send --peer-pk demo --message \"Hello\"",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  With a real peer public key:  ", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(
                "    polygone send --peer-pk <hex-or-file.pk> --message \"Hello\"",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Read message from stdin:  ", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(
                "    echo \"Hello\" | polygone send --peer-pk <pk> --message -",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let p = Paragraph::new(usage_lines).block(section_block("Send Message"));
    frame.render_widget(p, chunks[0]);

    let flow_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Message flow:", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  plaintext", Style::default().fg(Color::White)),
            Span::styled("  →  AES-256-GCM  →  ", Style::default().fg(Color::DarkGray)),
            Span::styled("ciphertext", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  ciphertext", Style::default().fg(Color::Cyan)),
            Span::styled(" →  Shamir 4-of-7  →  ", Style::default().fg(Color::DarkGray)),
            Span::styled("7 fragments", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("  7 fragments", Style::default().fg(Color::Green)),
            Span::styled(" →  dispatch  →  ", Style::default().fg(Color::DarkGray)),
            Span::styled("7 ephemeral nodes", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("  session key", Style::default().fg(Color::Magenta)),
            Span::styled(" derived from ", Style::default().fg(Color::DarkGray)),
            Span::styled("ML-KEM-1024 shared secret", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Note: P2P network dispatch arrives in v2.0 (libp2p + Kademlia DHT)",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
        ]),
    ];

    let p = Paragraph::new(flow_lines).block(section_block("Protocol Flow"));
    frame.render_widget(p, chunks[1]);
}

// ── Receive ───────────────────────────────────────────────────────────────────

fn render_receive(frame: &mut Frame, area: Rect, _app: &App) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Reconstruct a message from Shamir fragments."),
        ]),
        Line::from(vec![
            Span::raw("  Requires your secret key and the initiator's KEM ciphertext."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Usage:", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(
                "    polygone receive --sk ~/.polygone/keys/kem.sk --ciphertext <hex>",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Or use the demo flow:", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(
                "    polygone send --peer-pk demo --message \"Hello\"",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Requirements:", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ✔ ", Style::default().fg(Color::Green)),
            Span::raw("Your KEM secret key (~/.polygone/keys/kem.sk)"),
        ]),
        Line::from(vec![
            Span::styled("  ✔ ", Style::default().fg(Color::Green)),
            Span::raw("At least 4 of 7 fragments from the initiator"),
        ]),
        Line::from(vec![
            Span::styled("  ✔ ", Style::default().fg(Color::Green)),
            Span::raw("KEM ciphertext from the initiator (sent out-of-band)"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Full P2P receive with automatic fragment collection: v2.0",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            ),
        ]),
    ];

    let p = Paragraph::new(lines).block(section_block("Receive Message"));
    frame.render_widget(p, area);
}

// ── Node ──────────────────────────────────────────────────────────────────────

fn render_node(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let cmd_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Run a relay node and contribute to the network."),
        ]),
        Line::from(vec![
            Span::raw("  You never see the content of messages you relay."),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Start:", Style::default().fg(Color::DarkGray))]),
        Line::from(vec![
            Span::styled(
                "    polygone node start",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "    polygone node start --listen 0.0.0.0:4001 --ram-mb 128",
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Info:", Style::default().fg(Color::DarkGray))]),
        Line::from(vec![
            Span::styled(
                "    polygone node info",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Requirements:", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ✔ ", Style::default().fg(Color::Green)),
            Span::raw("512 MB RAM minimum"),
        ]),
        Line::from(vec![
            Span::styled("  ✔ ", Style::default().fg(Color::Green)),
            Span::raw("Port 4001 open (or --listen to change)"),
        ]),
        Line::from(vec![
            Span::styled("  ○ ", Style::default().fg(Color::DarkGray)),
            Span::raw("P2P transport: libp2p + Kademlia DHT (v2.0)"),
        ]),
    ];

    let p = Paragraph::new(cmd_lines).block(section_block("Relay Node"));
    frame.render_widget(p, chunks[0]);

    render_node_grid(frame, chunks[1], 0, app.tick);
}

// ── Self-test ─────────────────────────────────────────────────────────────────

fn render_selftest(frame: &mut Frame, area: Rect, _app: &App) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Run the built-in self-test suite to verify all crypto primitives."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  polygone self-test",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  Tests:", Style::default().fg(Color::DarkGray))]),
        Line::from(vec![
            Span::styled("  [1/5] ", Style::default().fg(Color::DarkGray)),
            Span::raw("ML-KEM-1024 round-trip (encapsulate → decapsulate)"),
        ]),
        Line::from(vec![
            Span::styled("  [2/5] ", Style::default().fg(Color::DarkGray)),
            Span::raw("AES-256-GCM encrypt → decrypt"),
        ]),
        Line::from(vec![
            Span::styled("  [3/5] ", Style::default().fg(Color::DarkGray)),
            Span::raw("Shamir 4-of-7 — all C(7,4)=35 combinations"),
        ]),
        Line::from(vec![
            Span::styled("  [4/5] ", Style::default().fg(Color::DarkGray)),
            Span::raw("Full session round-trip (Alice → Bob)"),
        ]),
        Line::from(vec![
            Span::styled("  [5/5] ", Style::default().fg(Color::DarkGray)),
            Span::raw("Insufficient fragments → rejected"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  BLAKE3 domain separation verified in unit tests.", 
                Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Ed25519 sign/verify verified in unit tests.", 
                Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let p = Paragraph::new(lines).block(section_block("Self-Test Suite"));
    frame.render_widget(p, area);
}

// ── Help ──────────────────────────────────────────────────────────────────────

fn render_help(frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let cmd_lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled("  COMMANDS", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  polygone keygen", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::styled("    Generate ML-KEM-1024 + Ed25519 keypair, save to disk", Style::default().fg(Color::DarkGray))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  polygone send", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::styled("    --peer-pk <hex|file|demo>  --message <text|->", Style::default().fg(Color::DarkGray))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  polygone receive", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::styled("    --sk <path>  --ciphertext <hex>", Style::default().fg(Color::DarkGray))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  polygone node start|stop|info", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::styled("    --listen <addr>  --ram-mb <n>", Style::default().fg(Color::DarkGray))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  polygone status", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::styled("    Show active sessions, node health, peers", Style::default().fg(Color::DarkGray))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  polygone self-test", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::styled("    Run the full self-test suite", Style::default().fg(Color::DarkGray))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  polygone tui", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![Span::styled("    Launch this TUI dashboard", Style::default().fg(Color::DarkGray))]),
    ];

    let p = Paragraph::new(cmd_lines).block(section_block("Commands"));
    frame.render_widget(p, chunks[0]);

    let nav_lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled("  TUI NAVIGATION", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  1 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Dashboard"),
        ]),
        Line::from(vec![
            Span::styled("  2 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Keygen"),
        ]),
        Line::from(vec![
            Span::styled("  3 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Send"),
        ]),
        Line::from(vec![
            Span::styled("  4 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Receive"),
        ]),
        Line::from(vec![
            Span::styled("  5 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Node"),
        ]),
        Line::from(vec![
            Span::styled("  6 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Self-Test"),
        ]),
        Line::from(vec![
            Span::styled("  ? ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Help (this page)"),
        ]),
        Line::from(vec![
            Span::styled("  q / Esc ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Quit"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  VERBOSITY", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  -v ", Style::default().fg(Color::Yellow)),
            Span::raw("info   "),
            Span::styled("  -vv ", Style::default().fg(Color::Yellow)),
            Span::raw("debug   "),
            Span::styled("  -vvv ", Style::default().fg(Color::Yellow)),
            Span::raw("trace"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled("  LINKS", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  github.com/lvs0/Polygone", Style::default().fg(Color::Blue).add_modifier(Modifier::UNDERLINED)),
        ]),
        Line::from(vec![
            Span::styled("  MIT License — No investors. No token. No telemetry.", 
                Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let p = Paragraph::new(nav_lines).block(section_block("Navigation & Info"));
    frame.render_widget(p, chunks[1]);
}

// ── Services ───────────────────────────────────────────────────────────────────

fn render_services(frame: &mut Frame, area: Rect, app: &App) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let modules: [(&str, &str, &str, Color, bool); 4] = [
        ("Drive",   "▣", "Encrypted distributed file storage. Sharded with Shamir, E2E encrypted.", Color::Yellow, false),
        ("Hide",    "◈", "Traffic obfuscation through ephemeral nodes. No observer can prove a connection.", Color::Yellow, false),
        ("Compute", "⬡", "Lend idle CPU/RAM. Earn credits. Ollama inference sharing.", Color::Green, true),
        ("Petals",  "✿", "Distributed ML inference through peer-to-peer network of contributors.", Color::Yellow, false),
    ];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(8)])
        .split(area);

    // Header
    let header = vec![
        Line::from(vec![
            Span::styled(" Services ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("— Polygone modules and extensions", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Run ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone compute start", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" to lend power. ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone compute status", Style::default().fg(Color::Yellow)),
            Span::styled(" for stats.", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    frame.render_widget(Paragraph::new(header).block(section_block("Services")), chunks[0]);

    // Module cards in 2x2 grid
    let grid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let left_cols = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(grid[0]);
    let right_cols = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(grid[1]);

    'outer: for col_idx in 0..2 {
        for row_idx in 0..2 {
            let idx = col_idx * 2 + row_idx;
            let module_area = if col_idx == 0 { left_cols[row_idx] } else { right_cols[row_idx] };
            if idx >= modules.len() { break 'outer; }
            let (name, icon, desc, color, available) = modules[idx];
            let spinner = ["◐", "◓", "◑", "◒"];
            let spin = spinner[(app.tick / 3) as usize % 4];

            let status_icon = if available { spin } else { "○" };
            let status_color = if available { Color::Green } else { Color::DarkGray };
            let version = if available { "v1.0" } else { "v2.0" };

            let lines = vec![
                Line::from(vec![
                    Span::styled(format!("{} ", icon), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::raw("  "),
                    Span::styled(status_icon, Style::default().fg(status_color)),
                    Span::raw("  "),
                    Span::styled(version, Style::default().fg(color)),
                ]),
                Line::from(vec![
                    Span::styled(format!("  {}", desc), Style::default().fg(Color::DarkGray)),
                ]),
            ];

            let block = Block::default()
                .title(Span::styled(format!(" {} ", name), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(if available { Color::Blue } else { Color::DarkGray }));

            frame.render_widget(Paragraph::new(lines).block(block).wrap(ratatui::widgets::Wrap { trim: true }), module_area);
        }
    }

    // Compute quick panel
    let compute_lines = vec![
        Line::from(vec![
            Span::styled("⬡ Compute", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("  Power Lending Daemon", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Idle threshold  ", Style::default().fg(Color::DarkGray)),
            Span::styled("300s (5 min)", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Max RAM fraction ", Style::default().fg(Color::DarkGray)),
            Span::styled("50%", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Ollama           ", Style::default().fg(Color::DarkGray)),
            Span::styled("enabled", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("  Commands  ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone compute start | stop | status", Style::default().fg(Color::Yellow)),
        ]),
    ];
    let block = Block::default()
        .title(Span::styled(" Polygone-Compute ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue));
    frame.render_widget(Paragraph::new(compute_lines).block(block).wrap(ratatui::widgets::Wrap { trim: true }), chunks[2]);
}

// ── Parameters ─────────────────────────────────────────────────────────────────

fn render_params(frame: &mut Frame, area: Rect, _app: &App) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let params: [(&str, &str, &str); 12] = [
        ("network.listen_addr",   "0.0.0.0:4001",          "P2P listen address (v2.0)"),
        ("network.port",         "4001",                  "Legacy relay port"),
        ("node.ram_mb",          "256",                   "Max RAM per node (MB)"),
        ("node.ttl_sec",         "3600",                  "Ephemeral node TTL (s)"),
        ("node.threshold",       "4",                     "Shamir reconstruction threshold"),
        ("node.fragment_count",  "7",                     "Total fragment / node count"),
        ("compute.idle_threshold","300",                  "Idle before lending (seconds)"),
        ("compute.max_ram_frac", "0.50",                  "Max RAM fraction for lending"),
        ("compute.max_cpu_frac", "80.0",                  "Max CPU % before pausing"),
        ("compute.status_listen","127.0.0.1:4002",        "Compute daemon status endpoint"),
        ("paths.keys",           "~/.polygone/keys",      "Key storage directory"),
        ("paths.config",         "~/.polygone",           "Config directory"),
    ];

    let crypto_params: [(&str, &str, &str); 5] = [
        ("crypto.kem",       "ML-KEM-1024 (FIPS 203)",  "Key encapsulation"),
        ("crypto.sign",      "Ed25519 (ML-DSA ready)",  "Signatures"),
        ("crypto.cipher",   "AES-256-GCM",             "Symmetric encryption"),
        ("crypto.kdf",       "BLAKE3 (domain-separated)","Key derivation"),
        ("crypto.threshold","Shamir 4-of-7",            "Secret sharing"),
    ];

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left: network + node params
    let left_lines: Vec<Line> = params.iter().map(|(key, val, _desc)| {
        Line::from(vec![
            Span::styled(format!("  {:.<26}", key), Style::default().fg(Color::Cyan)),
            Span::styled(*val, Style::default().fg(Color::White)),
        ])
    }).collect();

    let left_block = Block::default()
        .title(Span::styled(" Network & Node ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue));
    frame.render_widget(Paragraph::new(left_lines).block(left_block).wrap(ratatui::widgets::Wrap { trim: true }), chunks[0]);

    // Right: compute + crypto
    let right_lines: Vec<Line> = crypto_params.iter().map(|(key, val, _desc)| {
        Line::from(vec![
            Span::styled(format!("  {:.<26}", key), Style::default().fg(Color::Cyan)),
            Span::styled(*val, Style::default().fg(Color::White)),
        ])
    }).collect();

    let right_block = Block::default()
        .title(Span::styled(" Compute & Crypto ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue));
    frame.render_widget(Paragraph::new(right_lines).block(right_block).wrap(ratatui::widgets::Wrap { trim: true }), chunks[1]);

    // Bottom legend
    let legend = vec![
        Line::from(vec![
            Span::styled(" Edit via ", Style::default().fg(Color::DarkGray)),
            Span::styled("~/.polygone/polygone.toml", Style::default().fg(Color::Yellow)),
        ]),
    ];
    frame.render_widget(Paragraph::new(legend), area);
}
