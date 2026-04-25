//! POLYGONE Installer — Full-screen TUI
//!
//! Flow:
//!   Welcome → (if installed: Update/Uninstall/Reinstall/Launch) → Install → Configure
//!   → Dashboard
//!
//! Navigation: ↑↓ navigate · ENTER select · ESC back · q quit

use std::path::PathBuf;
use std::process::Command;

use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

// ─── Colors ─────────────────────────────────────────────────────────────────
const C_VOID:    Color = Color::Rgb(10, 10, 15);
const C_SURFACE: Color = Color::Rgb(17, 17, 24);
const C_BORDER:  Color = Color::Rgb(30, 30, 46);
const C_COBALT:  Color = Color::Rgb(26, 107, 255);
const C_GREEN:   Color = Color::Rgb(40, 200, 64);
const C_RED:     Color = Color::Rgb(255, 59, 48);
const C_YELLOW:  Color = Color::Rgb(255, 204, 0);
const C_TEXT:    Color = Color::Rgb(200, 200, 232);
const C_DIM:     Color = Color::Rgb(74, 74, 106);

// ─── Install state ────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
enum InstallState {
    // Pre-install
    Welcome,
    AlreadyInstalled,   // detected existing install
    Installing,
    // Post-install config
    Configure,
    // Dashboard
    Dashboard,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeChoice {
    None,
    Passive,
    Active,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Lang { EN, FR }

#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuAction {
    None,
    Update,
    Reinstall,
    Uninstall,
    Launch,
}

// ─── Step data ───────────────────────────────────────────────────────────────
struct Config {
    lang: Lang,
    username: String,
    node: NodeChoice,
    install_dir: PathBuf,
    config_dir: PathBuf,
}

impl Config {
    fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("polygone");
        let install_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".local/bin");
        Self {
            lang: Lang::EN,
            username: String::new(),
            node: NodeChoice::None,
            install_dir,
            config_dir,
        }
    }

    fn load(&mut self) {
        let cfg_file = self.config_dir.join("config.json");
        if let Ok(data) = std::fs::read_to_string(&cfg_file) {
            if let Ok(v) = json::parse(&data) {
                if let Some(u) = v.get("username").and_then(|x| x.as_str()) {
                    self.username = u.to_string();
                }
                if let Some(n) = v.get("node_mode").and_then(|x| x.as_str()) {
                    self.node = match n {
                        "passive" => NodeChoice::Passive,
                        "active" => NodeChoice::Active,
                        _ => NodeChoice::None,
                    };
                }
                if let Some(l) = v.get("language").and_then(|x| x.as_str()) {
                    self.lang = match l { "fr" => Lang::FR, _ => Lang::EN };
                }
            }
        }
    }

    fn save(&self) {
        std::fs::create_dir_all(&self.config_dir).ok();
        let node_str = match self.node {
            NodeChoice::None => "none",
            NodeChoice::Passive => "passive",
            NodeChoice::Active => "active",
        };
        let lang_str = match self.lang { Lang::FR => "fr", Lang::EN => "en" };
        let data = json::json!({
            "version": VERSION,
            "username": self.username,
            "node_mode": node_str,
            "language": lang_str,
        });
        std::fs::write(self.config_dir.join("config.json"), data.to_string()).ok();
    }

    fn t(&self, en: &str, fr: &str) -> &str {
        match self.lang { Lang::FR => fr, Lang::EN => en }
    }
}

// ─── App ──────────────────────────────────────────────────────────────────────
struct App {
    state: InstallState,
    config: Config,
    menu_idx: usize,
    menu_actions: Vec<MenuAction>,
    installing: bool,
    install_pct: f32,
    install_status: String,
    install_log: Vec<String>,
    install_error: Option<String>,
    dash_tab: usize,
    dash_item: usize,
    uninstall_confirm: bool,
}

impl App {
    fn new() -> Self {
        let mut config = Config::new();
        config.load();
        Self {
            state: InstallState::Welcome,
            config,
            menu_idx: 0,
            menu_actions: vec![],
            installing: false,
            install_pct: 0.0,
            install_status: String::new(),
            install_log: Vec::new(),
            install_error: None,
            dash_tab: 0,
            dash_item: 0,
            uninstall_confirm: false,
        }
    }

    fn binary_path(&self) -> PathBuf {
        self.config.install_dir.join("polygone")
    }

    fn is_installed(&self) -> bool {
        self.binary_path().exists()
    }

    fn log(&mut self, msg: &str) {
        self.install_log.push(msg.to_string());
        if self.install_log.len() > 8 { self.install_log.remove(0); }
    }

    fn set_status(&mut self, msg: &str, pct: f32) {
        self.install_status = msg.to_string();
        self.install_pct = pct;
        self.log(msg);
    }

    fn run_install(&mut self) {
        let install_dir = &self.config.install_dir;
        let url = format!(
            "https://github.com/lvs0/Polygone/releases/download/v{}/polygone",
            VERSION
        );

        self.set_status(self.config.t("Downloading Polygone...", "Téléchargement de Polygone..."), 0.15);

        std::fs::create_dir_all(install_dir).ok();
        let dest = self.binary_path();

        // Try download first
        let dl = Command::new("curl")
            .args(["-fsSL", "-o", "/tmp/polygone"])
            .arg(&url)
            .output();

        match dl {
            Ok(out) if out.status.success() => {
                self.set_status(self.config.t("Installing...", "Installation..."), 0.6);
                if std::fs::copy("/tmp/polygone", &dest).is_ok() {
                    #[cfg(unix)] {
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755)).ok();
                    }
                    self.set_status(self.config.t("Configuring...", "Configuration..."), 0.85);
                    self.config.save();
                    self.set_status(self.config.t("Done!", "Terminé!"), 1.0);
                    self.state = InstallState::Configure;
                } else {
                    self.install_error = Some(self.config.t("Copy failed", "Échec de la copie").to_string());
                }
            }
            _ => {
                // Build from source
                self.set_status(self.config.t("Building from source...", "Compilation depuis les sources..."), 0.2);
                let build_dir = dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("polygone-src");

                if !build_dir.exists() {
                    self.log(&format!("Cloning..."));
                    let _ = Command::new("git")
                        .args(["clone", "https://github.com/lvs0/Polygone.git"])
                        .arg(&build_dir)
                        .output();
                }

                self.set_status(self.config.t("Compiling (may take a while)...", "Compilation (peut prendre du temps)..."), 0.5);
                self.log("cargo build --release");

                let build = Command::new("cargo")
                    .current_dir(&build_dir)
                    .args(["build", "--release", "--bin", "polygone"])
                    .output();

                match build {
                    Ok(out) if out.status.success() => {
                        self.set_status(self.config.t("Installing compiled binary...", "Installation du binaire compilé..."), 0.85);
                        let src = build_dir.join("target/release/polygone");
                        if std::fs::copy(&src, &dest).is_ok() {
                            #[cfg(unix)] {
                                use std::os::unix::fs::PermissionsExt;
                                std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755)).ok();
                            }
                            self.config.save();
                            self.set_status(self.config.t("Done!", "Terminé!"), 1.0);
                            self.state = InstallState::Configure;
                        } else {
                            self.install_error = Some(self.config.t("Copy failed", "Échec de la copie").to_string());
                        }
                    }
                    _ => {
                        self.install_error = Some(self.config.t("Build failed. Install Rust: curl https://sh.rustup.rs | sh", "Compilation échouée. Installe Rust: curl https://sh.rustup.rs | sh").to_string());
                    }
                }
            }
        }
    }

    fn do_uninstall(&mut self) {
        let bin_path = self.binary_path();
        if bin_path.exists() {
            std::fs::remove_file(&bin_path).ok();
        }
        // Remove config
        std::fs::remove_file(self.config.config_dir.join("config.json")).ok();
        self.log("Uninstalled Polygone");
        self.state = InstallState::Done;
    }

    // ─── Main draw ────────────────────────────────────────────────────────────
    fn draw(&self, f: &mut Frame) {
        let size = f.area();
        f.render_widget(Clear, size);
        match self.state {
            InstallState::Welcome => self.draw_welcome(f, size),
            InstallState::AlreadyInstalled => self.draw_already_installed(f, size),
            InstallState::Installing => self.draw_installing(f, size),
            InstallState::Configure => self.draw_configure(f, size),
            InstallState::Dashboard => self.draw_dashboard(f, size),
            InstallState::Done => self.draw_done(f, size),
        }
    }

    fn centered(&self, w: u16, h: u16, size: Rect) -> Rect {
        let x = (size.width.saturating_sub(w)) / 2;
        let y = (size.height.saturating_sub(h)) / 2;
        Rect::new(x, y, w.min(size.width), h.min(size.height))
    }

    fn nav_raw(&self) -> Vec<Line> {
        vec![Line::from(vec![
            Span::raw("  "),
            Span::styled("↑↓", Style::new().fg(C_COBALT)),
            Span::raw(" navigate  "),
            Span::styled("ENTER", Style::new().fg(C_GREEN)),
            Span::raw(" select  "),
            Span::styled("ESC", Style::new().fg(C_DIM)),
            Span::raw(" back  "),
            Span::styled("q", Style::new().fg(C_DIM)),
            Span::raw(" quit"),
        ])]
    }

    fn block(&self, title: &str) -> Block {
        Block::new()
            .title(format!("  {}  ", title))
            .title_style(Style::new().fg(C_COBALT).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(C_BORDER))
            .style(Style::new().bg(C_SURFACE))
    }

    // ── Welcome ──────────────────────────────────────────────────────────────
    fn draw_welcome(&self, f: &mut Frame, size: Rect) {
        let box_w = 56u16.min(size.width.saturating_sub(6));
        let box_h = 18u16.min(size.height.saturating_sub(6));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block(""), rect);

        // Logo area (top half)
        let logo_rect = Rect::new(rect.x + 2, rect.y + 1, rect.width - 4, 7);
        let logo_lines = vec![
            Line::from(""),
            Line::from(Span::raw("       ⬡                         ⬡")),
            Line::from(Span::raw("     ⬡   ⬡                   ⬡   ⬡")),
            Line::from(Span::raw("   ⬡       ⬡               ⬡       ⬡")),
            Line::from(Span::raw("  ⬡    P O L Y G O N E    ⬡")),
            Line::from(Span::raw("   ⬡       ⬡               ⬡       ⬡")),
            Line::from(Span::raw("     ⬡   ⬡                   ⬡   ⬡")),
            Line::from(Span::raw("       ⬡                         ⬡")),
        ];
        let logo_p = Paragraph::new(logo_lines)
            .alignment(Alignment::Center)
            .style(Style::new().fg(C_COBALT));
        f.render_widget(logo_p, logo_rect);

        // Welcome text
        let inner_y = rect.y + 8;
        let text_rect = Rect::new(rect.x, inner_y, rect.width, rect.height - 8);
        let t = self.config.t;
        let lines = vec![
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Welcome to ", Style::new().fg(C_TEXT)),
                Span::styled("Polygone", Style::new().fg(C_COBALT).bold()),
            ]),
            Line::from(Span::raw("  Privacy that leaves no trace.")),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("ENTER", Style::new().fg(C_GREEN).bold()),
                Span::raw(" "),
                Span::raw(t("Install Polygone", "Installer Polygone")),
                Span::raw("  ·  "),
                Span::styled("q", Style::new().fg(C_DIM)),
                Span::raw(" quit"),
            ]),
        ];
        let p = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(p, text_rect);
    }

    // ── Already installed ────────────────────────────────────────────────────
    fn draw_already_installed(&self, f: &mut Frame, size: Rect) {
        let box_w = 52u16.min(size.width.saturating_sub(6));
        let box_h = (4 + self.menu_actions.len() as u16 * 3).min(size.height.saturating_sub(6));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block(self.config.t("Polygone is already installed", "Polygone est déjà installé")), rect);
        let inner = rect.inner(Margin::new(2, 1));

        let items: Vec<ListItem> = self.menu_actions.iter().enumerate().map(|(i, action)| {
            let sel = self.menu_idx == i;
            let icon = if sel { "▶" } else { " " };
            let style = if sel { Style::new().fg(C_GREEN).bold() } else { Style::new().fg(C_TEXT) };
            let label = match action {
                MenuAction::Update => self.config.t("Update to latest version", "Mettre à jour"),
                MenuAction::Reinstall => self.config.t("Reinstall Polygone", "Réinstaller Polygone"),
                MenuAction::Uninstall => self.config.t("Uninstall Polygone", "Désinstaller Polygone"),
                MenuAction::Launch => self.config.t("Launch Polygone", "Lancer Polygone"),
                MenuAction::None => "",
            };
            ListItem::new(vec![
                Line::from(vec![Span::styled(icon, style), Span::raw("  "), Span::styled(label, style)]),
                Line::from(Span::raw("")),
            ])
        }).collect();

        let list = List::new(items).style(Style::new().fg(C_TEXT));
        f.render_widget(list, inner);

        let nav = Paragraph::new(self.nav_raw())
            .alignment(Alignment::Center)
            .style(Style::new().fg(C_DIM));
        f.render_widget(nav, inner);
    }

    // ── Installing ──────────────────────────────────────────────────────────
    fn draw_installing(&self, f: &mut Frame, size: Rect) {
        let box_w = 58u16.min(size.width.saturating_sub(6));
        let box_h = 22u16.min(size.height.saturating_sub(6));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block(self.config.t("Installing Polygone", "Installation de Polygone")), rect);
        let inner = rect.inner(Margin::new(2, 1));

        let pct = (self.install_pct * 100.0) as u16;
        let status_line = Line::from(vec![
            Span::raw("  "),
            Span::raw(&self.install_status),
            Span::raw(format!("  {}%", pct)),
        ]);

        let log_lines: Vec<Line> = self.install_log.iter()
            .map(|l| Line::from(Span::styled(format!("  {}", l), Style::new().fg(C_DIM))))
            .collect();

        let error_line = self.install_error.as_ref().map(|e| {
            Line::from(vec![Span::styled("✗ ERROR: ", Style::new().fg(C_RED)), Span::raw(e)])
        });

        let all: Vec<Line> = [
            vec![Line::from("")],
            log_lines,
            vec![Line::from("")],
            vec![status_line],
            error_line.into_iter().collect(),
        ].concat();

        let p = Paragraph::new(all).style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    // ── Configure ───────────────────────────────────────────────────────────
    fn draw_configure(&self, f: &mut Frame, size: Rect) {
        let box_w = 52u16.min(size.width.saturating_sub(6));
        let box_h = 22u16.min(size.height.saturating_sub(6));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block(self.config.t("Configure Polygone", "Configurer Polygone")), rect);
        let inner = rect.inner(Margin::new(2, 1));

        let t = self.config.t;
        let lang_label = match self.config.lang { Lang::FR => "Français", Lang::EN => "English" };
        let node_label = match self.config.node {
            NodeChoice::None => t("Disabled", "Désactivé"),
            NodeChoice::Passive => t("Passive — invisible", "Passif — invisible"),
            NodeChoice::Active => t("Active — shared power", "Actif — puissance partagée"),
        };

        let lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled("  ✓ Polygone installed!", Style::new().fg(C_GREEN).bold())]),
            Line::from(Span::raw("")),
            Line::from(vec![Span::raw("  Language:  "), Span::styled(lang_label, Style::new().fg(C_COBALT))]),
            Line::from(vec![Span::raw("  Username:   "), Span::raw(if self.config.username.is_empty() { "anonymous".to_string() } else { self.config.username.clone() })]),
            Line::from(vec![Span::raw("  Node:       "), Span::raw(node_label)]),
            Line::from(Span::raw("")),
            Line::from(vec![Span::raw("  "), Span::styled("ENTER", Style::new().fg(C_GREEN).bold()), Span::raw(format!("  {}", t("Launch dashboard", "Ouvrir le dashboard")))]),
            Line::from(Span::raw("")),
        ];

        let p = Paragraph::new(lines).style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    // ── Dashboard ────────────────────────────────────────────────────────────
    fn draw_dashboard(&self, f: &mut Frame, size: Rect) {
        let tab_h = 3u16;
        let footer_h = 2u16;
        let header_h = 3u16;

        // Header
        let header_rect = Rect::new(0, 0, size.width, header_h);
        let hblock = Block::new()
            .borders(Borders::BOTTOM)
            .border_style(Style::new().fg(C_BORDER))
            .style(Style::new().bg(C_SURFACE));
        f.render_widget(hblock, header_rect);

        let user_label = if self.config.username.is_empty() { "anonymous".to_string() } else { self.config.username.clone() };
        let header_lines = vec![Line::from(vec![
            Span::styled("⬡ Polygone", Style::new().fg(C_COBALT).bold()),
            Span::raw(format!("  v{}  ·  {}", VERSION, user_label.escape_default())),
        ])];
        f.render_widget(Paragraph::new(header_lines).style(Style::new().fg(C_TEXT)), header_rect);

        // Tab bar
        let tab_rect = Rect::new(0, header_h, size.width, tab_h);
        let tabs = ["Home", "Services", "Nodes", "Settings"];
        let tab_lines: Vec<Line> = tabs.iter().enumerate().map(|(i, tab)| {
            let sel = self.dash_tab == i;
            let style = if sel { Style::new().fg(C_GREEN).bold() } else { Style::new().fg(C_DIM) };
            let prefix = if sel { "▶ " } else { "  " };
            Line::from(vec![
                Span::raw("   "),
                Span::styled(prefix, style),
                Span::styled(tab, style),
            ])
        }).collect();
        f.render_widget(Paragraph::new(tab_lines).style(Style::new().fg(C_TEXT)), tab_rect);

        // Content
        let content_y = header_h + tab_h;
        let content_h = size.height.saturating_sub(content_y + footer_h);
        let content_rect = Rect::new(0, content_y, size.width, content_h);

        match self.dash_tab {
            0 => self.draw_tab_home(f, content_rect),
            1 => self.draw_tab_services(f, content_rect),
            2 => self.draw_tab_nodes(f, content_rect),
            3 => self.draw_tab_settings(f, content_rect),
            _ => {}
        }

        // Footer
        let footer_rect = Rect::new(0, size.height - footer_h, size.width, footer_h);
        let fblock = Block::new()
            .borders(Borders::TOP)
            .border_style(Style::new().fg(C_BORDER))
            .style(Style::new().bg(C_SURFACE));
        f.render_widget(fblock, footer_rect);

        let node_status = match self.config.node {
            NodeChoice::None => "Node: off",
            NodeChoice::Passive => "Node: passive",
            NodeChoice::Active => "Node: active",
        };
        let footer_lines = vec![Line::from(vec![
            Span::styled(node_status, Style::new().fg(C_COBALT)),
            Span::raw("  ·  Polygone v"),
            Span::raw(VERSION),
            Span::raw("  ·  Hope"),
        ])];
        f.render_widget(Paragraph::new(footer_lines).style(Style::new().fg(C_DIM)), footer_rect);
    }

    fn draw_tab_home(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(1, 1));
        let t = self.config.t;
        let lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled("  ⬡ POLYGONE", Style::new().fg(C_COBALT).bold()), Span::raw("  —  Privacy that leaves no trace.")]),
            Line::from(""),
            Line::from(vec![Span::raw("  Status: "), Span::styled("Running", Style::new().fg(C_GREEN))]),
            Line::from(vec![Span::raw("  User:   "), Span::raw(if self.config.username.is_empty() { "anonymous".to_string() } else { self.config.username.clone() })]),
            Line::from(vec![Span::raw("  Node:   "), Span::raw(match self.config.node {
                NodeChoice::None => t("Disabled", "Désactivé"),
                NodeChoice::Passive => t("Passive (invisible)", "Passif (invisible)"),
                NodeChoice::Active => t("Active (sharing power)", "Actif (puissance partagée)"),
            })]),
            Line::from(Span::raw("")),
            Line::from(Span::raw("  Quick actions:")),
            Line::from(Span::raw("")),
            Line::from(vec![Span::styled(if self.dash_item == 0 { "  ▶ " } else { "    " }, Style::new().fg(C_GREEN)), Span::raw(t("Run self-test", "Test crypto"))]),
            Line::from(vec![Span::styled(if self.dash_item == 1 { "  ▶ " } else { "    " }, Style::new().fg(C_GREEN)), Span::raw(t("Generate keys", "Générer clés"))]),
            Line::from(vec![Span::styled(if self.dash_item == 2 { "  ▶ " } else { "    " }, Style::new().fg(C_GREEN)), Span::raw(t("Send a message", "Envoyer un message"))]),
            Line::from(Span::raw("")),
        ];
        let p = Paragraph::new(lines).style(Style::new().fg(C_TEXT)).wrap(Wrap { trim: true });
        f.render_widget(p, inner);
    }

    fn draw_tab_services(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(1, 1));
        let services = [
            ("polygone",       "Core network",           true),
            ("polygone-drive", "Encrypted storage",     false),
            ("polygone-hide",  "Privacy tunnel",         false),
            ("polygone-petals","LLM inference",         false),
        ];
        let lines: Vec<Line> = vec![Line::from(Span::raw("  Services  "))]
            .into_iter()
            .chain(services.iter().enumerate().flat_map(|(i, (name, desc, on))| {
                let icon = if *on { "●" } else { "○" };
                let col = if *on { C_GREEN } else { C_DIM };
                vec![
                    Line::from(vec![
                        if self.dash_item == i { Span::styled("  ▶ ", Style::new().fg(C_GREEN)) } else { Span::raw("    ") },
                        Span::styled(icon, Style::new().fg(col)),
                        Span::raw("  "),
                        Span::raw(name),
                        Span::raw("  —  "),
                        Span::raw(*desc),
                    ]),
                ]
            }))
            .collect();
        let p = Paragraph::new(lines).style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    fn draw_tab_nodes(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(1, 1));
        let t = self.config.t;
        let node_lines = match self.config.node {
            NodeChoice::None => vec![
                Line::from(Span::raw("  Node is disabled.")),
                Line::from(Span::raw("")),
                Line::from(vec![Span::raw("  "), Span::styled("Enable passive node?", Style::new().fg(C_COBALT))]),
                Line::from(Span::raw("  Passive: share bandwidth only. Invisible. Pausable.")),
                Line::from(Span::raw("")),
            ],
            NodeChoice::Passive | NodeChoice::Active => vec![
                Line::from(vec![Span::raw("  Node: "), Span::styled(match self.config.node {
                    NodeChoice::Passive => "Passive",
                    NodeChoice::Active => "Active",
                    _ => "",
                }, Style::new().fg(C_GREEN))]),
                Line::from(Span::raw("")),
                Line::from(Span::raw("  The node is intelligent and invisible.")),
                Line::from(Span::raw("  It shares bandwidth in the background.")),
                Line::from(Span::raw("  You can pause it anytime.")),
                Line::from(Span::raw("")),
                Line::from(vec![Span::styled(if self.dash_item == 0 { "  ▶ " } else { "    " }, Style::new().fg(C_YELLOW)), Span::raw(t("Pause for 1 hour", "Pause 1h"))]),
                Line::from(vec![Span::styled(if self.dash_item == 1 { "  ▶ " } else { "    " }, Style::new().fg(C_YELLOW)), Span::raw(t("Pause for 4 hours", "Pause 4h"))]),
                Line::from(vec![Span::styled(if self.dash_item == 2 { "  ▶ " } else { "    " }, Style::new().fg(C_RED)), Span::raw(t("Disable node", "Désactiver le noeud"))]),
                Line::from(Span::raw("")),
            ],
        };

        let all: Vec<Line> = [vec![Line::from(Span::raw("  Nodes  "))], node_lines].concat();
        let p = Paragraph::new(all).style(Style::new().fg(C_TEXT)).wrap(Wrap { trim: true });
        f.render_widget(p, inner);
    }

    fn draw_tab_settings(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(1, 1));
        let t = self.config.t;
        let settings = [
            (t("Username", "Nom d'utilisateur"), if self.config.username.is_empty() { "anonymous".to_string() } else { self.config.username.clone() }),
            (t("Language", "Langue"), match self.config.lang { Lang::FR => "Français", Lang::EN => "English" }),
            (t("Node mode", "Mode noeud"), match self.config.node {
                NodeChoice::None => t("Disabled", "Désactivé"),
                NodeChoice::Passive => t("Passive", "Passif"),
                NodeChoice::Active => t("Active", "Actif"),
            }),
            ("Version", VERSION.to_string()),
        ];

        let lines: Vec<Line> = vec![Line::from(Span::raw("  Settings  "))]
            .into_iter()
            .chain(settings.iter().enumerate().flat_map(|(i, (key, val))| {
                vec![
                    Line::from(vec![
                        if self.dash_item == i { Span::styled("  ▶ ", Style::new().fg(C_GREEN)) } else { Span::raw("    ") },
                        Span::raw(key),
                        Span::raw(": "),
                        Span::styled(val, Style::new().fg(C_COBALT)),
                    ]),
                ]
            }))
            .collect();

        let p = Paragraph::new(lines).style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    // ── Done ────────────────────────────────────────────────────────────────
    fn draw_done(&self, f: &mut Frame, size: Rect) {
        let box_w = 46u16.min(size.width.saturating_sub(6));
        let box_h = 12u16.min(size.height.saturating_sub(6));
        let rect = self.centered(box_w, box_h, size);
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(C_BORDER))
            .style(Style::new().bg(C_SURFACE));
        f.render_widget(block, rect);
        let inner = rect.inner(2);
        let lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled("  ⬡  Goodbye.", Style::new().fg(C_COBALT).bold())]),
            Line::from(Span::raw("")),
            Line::from(Span::raw("  Thank you for choosing privacy.")),
            Line::from(Span::raw("")),
        ];
        let p = Paragraph::new(lines).alignment(Alignment::Center).style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    // ── Event handling ───────────────────────────────────────────────────────
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        if key.kind != KeyEventKind::Press && key.kind != KeyEventKind::Repeat { return; }

        match self.state {
            // ── Welcome ──
            InstallState::Welcome => {
                if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                    if self.is_installed() {
                        self.menu_actions = vec![MenuAction::Update, MenuAction::Reinstall, MenuAction::Uninstall, MenuAction::Launch];
                        self.menu_idx = 0;
                        self.state = InstallState::AlreadyInstalled;
                    } else {
                        self.state = InstallState::Installing;
                    }
                } else if key.code == KeyCode::Char('q') {
                    self.state = InstallState::Done;
                }
            }

            // ── Already installed menu ──
            InstallState::AlreadyInstalled => {
                if key.code == KeyCode::Up || key.code == KeyCode::Char('k') {
                    self.menu_idx = self.menu_idx.saturating_sub(1);
                } else if key.code == KeyCode::Down || key.code == KeyCode::Char('j') {
                    self.menu_idx = (self.menu_idx + 1).min(self.menu_actions.len().saturating_sub(1));
                } else if key.code == KeyCode::Enter {
                    match self.menu_actions.get(self.menu_idx) {
                        Some(MenuAction::Update | MenuAction::Reinstall) => {
                            self.state = InstallState::Installing;
                        }
                        Some(MenuAction::Uninstall) => {
                            self.do_uninstall();
                        }
                        Some(MenuAction::Launch) => {
                            self.state = InstallState::Dashboard;
                        }
                        _ => {}
                    }
                } else if key.code == KeyCode::Esc {
                    self.state = InstallState::Welcome;
                }
            }

            // ── Installing (runs automatically) ──
            InstallState::Installing => {
                if self.install_error.is_some() && key.code == KeyCode::Enter {
                    self.state = InstallState::Done;
                }
            }

            // ── Configure ──
            InstallState::Configure => {
                if key.code == KeyCode::Enter {
                    self.state = InstallState::Dashboard;
                }
            }

            // ── Dashboard ──
            InstallState::Dashboard => {
                let tab_count = 4;
                let item_counts = [3, 4, 3, 4];

                if key.code == KeyCode::Left || key.code == KeyCode::Char('h') {
                    self.dash_tab = (self.dash_tab + tab_count - 1) % tab_count;
                    self.dash_item = 0;
                } else if key.code == KeyCode::Right || key.code == KeyCode::Char('l') {
                    self.dash_tab = (self.dash_tab + 1) % tab_count;
                    self.dash_item = 0;
                } else if key.code == KeyCode::Up || key.code == KeyCode::Char('k') {
                    self.dash_item = self.dash_item.saturating_sub(1);
                } else if key.code == KeyCode::Down || key.code == KeyCode::Char('j') {
                    let max = *item_counts.get(self.dash_tab).unwrap_or(&1);
                    self.dash_item = (self.dash_item + 1).min(max.saturating_sub(1));
                } else if key.code == KeyCode::Enter {
                    // Action on selected item
                } else if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                    self.state = InstallState::Done;
                }
            }

            // ── Done ──
            InstallState::Done => {}
        }
    }

    // ── Run loop ─────────────────────────────────────────────────────────────
    fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        // Run install asynchronously-like (blocking call, but that's ok for now)
        loop {
            terminal.draw(|f| self.draw(f))?;

            if self.state == InstallState::Installing && !self.installing {
                self.installing = true;
                self.run_install();
            }

            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }

            if self.state == InstallState::Done { break; }
        }
        Ok(())
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────
fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    crossterm::terminal::enable_raw_mode()?;
    let result = App::new().run(&mut terminal);
    crossterm::terminal::disable_raw_mode()?;
    ratatui::restore();
    result
}
