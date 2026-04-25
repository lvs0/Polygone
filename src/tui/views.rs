//! All TUI views for POLYGONE v2 — inspired by dashboard mockups (PNG 1-4).
//! Tab order: [1] Acceuil | [2] Favoris | [3] Services | [4] Parametres
//! Extended views (Keygen, Send, Node, etc.) remain on number keys 5-9.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::App;
use super::widgets::{
    render_balance_card, render_favorites_grid, render_header,
    render_messages, render_pause_modal,
    render_quick_actions, render_service_list, render_statusbar,
    render_tabs, section_block,
};
use crate::keys;

// ── View enum ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    #[default]
    Dashboard,  // [1] Acceuil
    Favoris,     // [2] Favoris
    Services,    // [3] Services
    Params,      // [4] Parametres
    // Extended views — accessible via 5-9 but not shown in main tab bar
    Keygen,
    Send,
    Receive,
    Node,
    SelfTest,
    Help,
}

impl View {
    /// Maps view → tab bar index (0-3 for the 4 visible tabs).
    fn tab_index(self) -> usize {
        match self {
            Self::Dashboard => 0,
            Self::Favoris   => 1,
            Self::Services  => 2,
            Self::Params    => 3,
            // Extended views — map to closest tab or 0
            Self::Keygen    => 0,
            Self::Send      => 0,
            Self::Receive   => 0,
            Self::Node      => 0,
            Self::SelfTest  => 0,
            Self::Help      => 0,
        }
    }

    fn is_extended(self) -> bool {
        matches!(self, Self::Keygen | Self::Send | Self::Receive | Self::Node | Self::SelfTest | Self::Help)
    }
}

// ── Root render dispatcher ───────────────────────────────────────────────────

/// Root render function — called every frame.
pub fn render_view(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let is_extended = app.current_view.is_extended();

    // Layout:
    //   - header (3 rows)
    //   - tabs (1 row) — hidden for extended views
    //   - content (fill)
    //   - messages (6) — hidden for extended views
    //   - statusbar (1 row) — visible for all
    let (tab_height, msg_height) = if is_extended { (0, 0) } else { (1, 6) };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(tab_height),
            Constraint::Min(10),
            Constraint::Length(msg_height),
            Constraint::Length(1),
        ])
        .split(size);

    render_header(frame, chunks[0]);
    if !is_extended {
        render_tabs(frame, chunks[1], app.current_view.tab_index());
    }

    match app.current_view {
        View::Dashboard => render_dashboard(frame, chunks[2], app),
        View::Favoris   => render_favoris(frame, chunks[2], app),
        View::Services  => render_services(frame, chunks[2], app),
        View::Params    => render_params(frame, chunks[2], app),
        View::Keygen    => render_keygen(frame, chunks[2], app),
        View::Send      => render_send(frame, chunks[2], app),
        View::Receive   => render_receive(frame, chunks[2], app),
        View::Node      => render_node(frame, chunks[2], app),
        View::SelfTest  => render_selftest(frame, chunks[2], app),
        View::Help      => render_help(frame, chunks[2]),
    }

    if !is_extended {
        render_messages(frame, chunks[3], app);
    }
    render_statusbar(frame, chunks[4]);
}

// ── Dashboard (Acceuil) ───────────────────────────────────────────────────────

fn render_dashboard(frame: &mut Frame, area: Rect, app: &App) {
    let show_modal = app.show_pause_modal;

    if show_modal {
        // Center modal overlay
        let modal_h = 18;
        let modal_w = 50;
        let x = (area.width.saturating_sub(modal_w)) / 2;
        let y = (area.height.saturating_sub(modal_h)) / 2;
        let modal_area = Rect::new(x, y, modal_w, modal_h);
        render_pause_modal(frame, modal_area);
        return;
    }

    // Main dashboard: 3 rows
    // Row 1: Quick actions (full width)
    // Row 2: Status left | Balance right | Node grid right-right
    // Row 3: Favoris preview (3 columns)

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),   // quick actions
            Constraint::Min(8),     // middle section
            Constraint::Length(10), // favorites preview
        ])
        .split(area);

    // Row 1: Quick actions bar
    render_quick_actions(frame, Rect::new(rows[0].x, rows[0].y, rows[0].width, 2));

    // Row 2: left (status + node controls) | right (balance + node grid)
    let mid = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    // Left: node status + controls
    let status_lines = vec![
        Line::from(vec![
            Span::styled(" Noeud", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("●", Style::default().fg(Color::Green)),
            Span::styled(" Actifs", Style::default().fg(Color::Green)),
            Span::styled(" depuis 18min", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from("  ──────────────────────────────────"),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Desactiver]", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  [Mettre a jour]", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  [Redemarrer]", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  [Mettre en pause]", Style::default().fg(Color::White)),
        ]),
    ];
    let p_status = Paragraph::new(status_lines)
        .block(section_block("Noeud"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p_status, mid[0]);

    // Right top: balance card
    render_balance_card(frame, mid[1]);

    // Row 3: favorites preview
    render_favorites_grid(frame, rows[2], app);
}

// ── Favoris ───────────────────────────────────────────────────────────────────

fn render_favoris(frame: &mut Frame, area: Rect, _app: &App) {
    // Full 3-column layout
    render_favorites_grid(frame, area, _app);
}

// ── Keygen ──────────────────────────────────────────────────────────────────

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
            Span::styled("  Algorithm:  ", Style::default().fg(Color::DarkGray)),
            Span::styled("ML-KEM-1024 (FIPS 203)", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  Signature:  ", Style::default().fg(Color::DarkGray)),
            Span::styled("ML-DSA-87 (FIPS 204)", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  Location:   ", Style::default().fg(Color::DarkGray)),
            Span::styled(key_dir.display().to_string(), Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(if exists {
            vec![
                Span::styled("  ✔", Style::default().fg(Color::Green)),
                Span::styled(" Keypair already exists", Style::default().fg(Color::Green)),
            ]
        } else {
            vec![
                Span::styled("  ✖", Style::default().fg(Color::Red)),
                Span::styled(" No keypair — press [ENTER] to generate", Style::default().fg(Color::Yellow)),
            ]
        }),
        Line::from(""),
        Line::from(vec![
            Span::styled("  WARNING: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("Keypair is a secret. ", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Anyone with the private key can decrypt your messages.", Style::default().fg(Color::White)),
        ]),
    ];

    let p = Paragraph::new(info_lines)
        .block(section_block("Key Generation"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p, chunks[0]);

    // Visual key icon
    let key_lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled("     .--. ", Style::default().fg(Color::Cyan))]),
        Line::from(vec![Span::styled("     /    \\", Style::default().fg(Color::Cyan))]),
        Line::from(vec![Span::styled("    | KEM |", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled("     \\    /", Style::default().fg(Color::Cyan))]),
        Line::from(vec![Span::styled("      `--", Style::default().fg(Color::Cyan))]),
        Line::from(vec![Span::styled("       ||", Style::default().fg(Color::Cyan))]),
        Line::from(vec![Span::styled("      ===", Style::default().fg(Color::Cyan))]),
        Line::from(vec![Span::styled("   ML-KEM-1024", Style::default().fg(Color::Cyan))]),
        Line::from(""),
    ];
    let p_key = Paragraph::new(key_lines)
        .block(section_block(""))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p_key, chunks[1]);
}

// ── Send ─────────────────────────────────────────────────────────────────────

fn render_send(frame: &mut Frame, area: Rect, _app: &App) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Usage: ", Style::default().fg(Color::Yellow)),
            Span::styled("polygone send <recipient_pubkey> <message>", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  1. Generate your keypair:   ", Style::default().fg(Color::DarkGray)),
            Span::styled("[2] Keygen", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  2. Share your public key with the recipient.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  3. Get the recipient's public key.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  4. Send your message.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Security model: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  The message is split into 7 Shamir fragments.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  4 fragments are sufficient to reconstruct.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Each fragment is AES-256-GCM encrypted.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Fragments auto-expire and are deleted.", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    let p = Paragraph::new(lines)
        .block(section_block("Send Message"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p, area);
}

// ── Receive ─────────────────────────────────────────────────────────────────

fn render_receive(frame: &mut Frame, area: Rect, _app: &App) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Wait for incoming messages using ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone receive", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Usage: ", Style::default().fg(Color::Yellow)),
            Span::styled("polygone receive", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  1. Start the receiver on your node.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  2. Share your public key with the sender.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  3. The sender initiates a session.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  4. The message is reconstructed and decrypted.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Only ", Style::default().fg(Color::DarkGray)),
            Span::styled("4-of-7", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(" fragments are needed.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Your private key is never shared.", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    let p = Paragraph::new(lines)
        .block(section_block("Receive Message"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p, area);
}

// ── Node ────────────────────────────────────────────────────────────────────

fn render_node(frame: &mut Frame, area: Rect, _app: &App) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Control your POLYGONE network node.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Start node:   ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone node start", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  Stop node:    ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone node stop", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  Node status:  ", Style::default().fg(Color::DarkGray)),
            Span::styled("polygone node status", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Node modes:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Active:      ", Style::default().fg(Color::DarkGray)),
            Span::styled("Participates in message routing.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Paused:      ", Style::default().fg(Color::DarkGray)),
            Span::styled("Receives but does not relay.", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Disabled:    ", Style::default().fg(Color::DarkGray)),
            Span::styled("Does not participate in any network activity.", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    let p = Paragraph::new(lines)
        .block(section_block("Node Control"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p, area);
}

// ── Self-Test ────────────────────────────────────────────────────────────────

fn render_selftest(frame: &mut Frame, area: Rect, _app: &App) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Run: ", Style::default().fg(Color::Yellow)),
            Span::styled("polygone self-test", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Tests:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  [1] ML-KEM-1024 round-trip key exchange", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  [2] AES-256-GCM encrypt/decrypt", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  [3] Shamir 4-of-7 (all 35 combinations)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  [4] Full session round-trip (Alice → Bob)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  [5] Insufficient fragments → rejected", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✔ All 5 tests must pass for production use.", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Expected output:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  ⬡ POLYGONE SELF-TEST", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("    [1/5] ML-KEM-1024 round-trip …… PASS ✔", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    [2/5] AES-256-GCM encrypt/decrypt …… PASS ✔", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    [3/5] Shamir 4-of-7 …… PASS ✔", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    [4/5] Full session round-trip …… PASS ✔", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    [5/5] Insufficient fragments → rejected …… PASS ✔", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    ✔ All 5 tests passed. POLYGONE is operational.", Style::default().fg(Color::Green)),
        ]),
    ];
    let p = Paragraph::new(lines)
        .block(section_block("Self-Test"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p, area);
}

// ── Services ─────────────────────────────────────────────────────────────────

fn render_services(frame: &mut Frame, area: Rect, app: &App) {
    // Left: service list | Right: "Add to Favorites" button
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

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

    render_service_list(frame, chunks[0], &services, 0, &app.favorites);

    // Right: empty area with centered "Add to Favorites" button
    let right_lines = vec![
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [ Mettre en Favoris ]",
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let p = Paragraph::new(right_lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p, chunks[1]);
}

// ── Params ───────────────────────────────────────────────────────────────────

fn render_params(frame: &mut Frame, area: Rect, _app: &App) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Configuration file: ", Style::default().fg(Color::DarkGray)),
            Span::styled("~/.config/polygone/", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Key directory:    ", Style::default().fg(Color::DarkGray)),
            Span::styled("~/.local/share/polygone/keys/", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Network mode:     ", Style::default().fg(Color::DarkGray)),
            Span::styled("local (P2P in v2.0)", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("  ML-KEM-1024:     ", Style::default().fg(Color::DarkGray)),
            Span::styled("enabled (FIPS 203)", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("  AES-256-GCM:      ", Style::default().fg(Color::DarkGray)),
            Span::styled("enabled (96-bit nonce)", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("  Shamir 4-of-7:    ", Style::default().fg(Color::DarkGray)),
            Span::styled("enabled (information-theoretic)", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("  BLAKE3:           ", Style::default().fg(Color::DarkGray)),
            Span::styled("enabled (domain-separated)", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Edit config: ", Style::default().fg(Color::Yellow)),
            Span::styled("nano ~/.config/polygone/config.toml", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("  View keys:   ", Style::default().fg(Color::Yellow)),
            Span::styled("ls ~/.local/share/polygone/keys/", Style::default().fg(Color::Cyan)),
        ]),
    ];
    let p = Paragraph::new(lines)
        .block(section_block("Parameters"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p, area);
}

// ── Help ────────────────────────────────────────────────────────────────────

fn render_help(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  POLYGONE v1.0.0 — Keyboard Shortcuts", 
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Navigation:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  1       ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Acceuil (Dashboard)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  2       ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Favoris", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  3       ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Services", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  4       ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Parametres", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  5       ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Generate Keys", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  6       ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Send Message", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  7       ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Receive Message", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  8       ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Node Control", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  9       ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Self-Test", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ? / h   ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Help (this screen)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Controls:", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  ↑ / ↓    ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Navigate / Select", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  ENTER    ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Confirm / Execute", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  q / Esc  ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Quit", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl+C   ", Style::default().fg(Color::Cyan)),
            Span::styled("→ Force Quit", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Documentation: ", Style::default().fg(Color::DarkGray)),
            Span::styled("https://github.com/lvs0/Polygone", Style::default().fg(Color::Cyan)),
        ]),
    ];
    let p = Paragraph::new(lines)
        .block(section_block("Help"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    frame.render_widget(p, area);
}
