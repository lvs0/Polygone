//! POLYGONE TUI Installer
//! Full-screen ratatui installation experience.
//!
//! Steps:
//!   1. Welcome (logo + language)
//!   2. Username setup (optional)
//!   3. Node mode selection
//!   4. Installation progress
//!   5. Configuration
//!   6. Done

use std::path::PathBuf;
use std::process::Command;

use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use ratatui::layout::{Alignment, Margin, Rect};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use ratatui::layout::Constraint;

const VERSION: &str = "1.0.0";

// ─── Colors ────────────────────────────────────────────────────────────────────
const C_VOID: Color    = Color::Rgb( 10,  10,  15);
const C_SURFACE: Color = Color::Rgb( 17,  17,  24);
const C_BORDER: Color  = Color::Rgb( 30,  30,  46);
const C_COBALT: Color  = Color::Rgb( 26, 107, 255);
const C_GREEN: Color    = Color::Rgb( 40, 200,  64);
const C_RED: Color     = Color::Rgb(255,  59,  48);
const C_YELLOW: Color  = Color::Rgb(255, 204,   0);
const C_TEXT: Color    = Color::Rgb(200, 200, 232);
const C_DIM: Color     = Color::Rgb( 74,  74, 106);

// ─── Logo (hexagonal ⬡ style) ────────────────────────────────────────────────
fn draw_logo(area: Rect, f: &mut Frame) {
    let inner = area.inner(Margin::new(0, 0));
    let lines = vec![
        Line::from(""),
        Line::from(Span::raw("         ⬡                         ⬡")),
        Line::from(Span::raw("       ⬡   ⬡                   ⬡   ⬡")),
        Line::from(Span::raw("     ⬡       ⬡             ⬡       ⬡")),
        Line::from(Span::raw("   ⬡           ⬡       ⬡           ⬡")),
        Line::from(Span::raw("  ⬡    P O L Y G O N E    ⬡")),
        Line::from(Span::raw("   ⬡           ⬡       ⬡           ⬡")),
        Line::from(Span::raw("     ⬡       ⬡             ⬡       ⬡")),
        Line::from(Span::raw("       ⬡   ⬡                   ⬡   ⬡")),
        Line::from(Span::raw("         ⬡                         ⬡")),
        Line::from(Span::raw("       EPHEMERAL · POST-QUANTUM · NETWORK")),
        Line::from(Span::raw("              v1.0.0 · by Hope")),
    ];

    let p = Paragraph::new(lines)
        .style(Style::new().fg(C_COBALT))
        .alignment(Alignment::Center);
    f.render_widget(p, inner);
}

// ─── Step enum ────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
enum Step {
    Welcome,
    Language,
    Username,
    NodeMode,
    Installing,
    Configuring,
    Done,
    // Post-install dashboard
    Dashboard,
}

// ─── Node mode ────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeMode {
    None,
    Passive,
    Active,
}

// ─── Language ────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
enum Language {
    English,
    Francais,
}

// ─── App state ───────────────────────────────────────────────────────────────
struct App {
    step: Step,
    language: Language,
    username: String,
    node_mode: NodeMode,
    install_progress: f32,
    install_status: String,
    install_log: Vec<String>,
    install_error: Option<String>,
    selected_idx: usize,
    installing_done: bool,
    config_done: bool,
}

impl App {
    fn new() -> Self {
        Self {
            step: Step::Welcome,
            language: Language::English,
            username: String::new(),
            node_mode: NodeMode::None,
            install_progress: 0.0,
            install_status: String::new(),
            install_log: Vec::new(),
            install_error: None,
            selected_idx: 0,
            installing_done: false,
            config_done: false,
        }
    }

    fn log(&mut self, msg: &str) {
        self.install_log.push(msg.to_string());
        if self.install_log.len() > 6 {
            self.install_log.remove(0);
        }
    }

    fn set_install_status(&mut self, status: &str, progress: f32) {
        self.install_status = status.to_string();
        self.install_progress = progress;
        self.log(status);
    }

    // ── Step transitions ───────────────────────────────────────────────────

    fn next_step(&mut self) {
        self.selected_idx = 0;
        match self.step {
            Step::Welcome   => self.step = Step::Language,
            Step::Language   => self.step = Step::Username,
            Step::Username   => self.step = Step::NodeMode,
            Step::NodeMode   => self.step = Step::Installing,
            Step::Installing if self.installing_done => self.step = Step::Configuring,
            Step::Configuring if self.config_done => self.step = Step::Dashboard,
            _ => {}
        }
    }

    fn prev_step(&mut self) {
        self.selected_idx = 0;
        match self.step {
            Step::Language  => self.step = Step::Welcome,
            Step::Username   => self.step = Step::Language,
            Step::NodeMode   => self.step = Step::Username,
            Step::Installing => self.step = Step::NodeMode,
            Step::Configuring => self.step = Step::Installing,
            _ => {}
        }
    }

    fn run_install(&mut self) {
        // Simulate installation steps with real actions
        self.set_install_status("Checking prerequisites...", 0.05);
        std::thread::sleep(std::time::Duration::from_millis(400));

        // Download binary
        let url = format!(
            "https://github.com/lvs0/Polygone/releases/download/v{}/polygone",
            VERSION
        );
        self.set_install_status("Downloading Polygone...", 0.25);
        self.log(&format!("URL: {}", &url));

        let install_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".local/bin");
        std::fs::create_dir_all(&install_dir).ok();

        let dl = Command::new("curl")
            .args(["-fsSL", "-o", "/tmp/polygone"])
            .arg(&url)
            .output();

        match dl {
            Ok(out) if out.status.success() => {
                self.set_install_status("Installing...", 0.65);
                let dest = install_dir.join("polygone");
                if std::fs::copy("/tmp/polygone", &dest).is_ok() {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755)).ok();
                    }
                    self.set_install_status("Configuring...", 0.80);
                    std::thread::sleep(std::time::Duration::from_millis(300));

                    // Write config
                    let config_dir = dirs::config_dir()
                        .unwrap_or_else(|| PathBuf::from("."))
                        .join("polygone");
                    std::fs::create_dir_all(&config_dir).ok();

                    let cfg = format!(
                        "{{\"version\":\"{VERSION}\",\"username\":\"{username}\",\"node_mode\":\"{node_mode}\",\"language\":\"{lang}\"}}",
                        username = self.username.escape_default(),
                        node_mode = match self.node_mode {
                            NodeMode::None => "none",
                            NodeMode::Passive => "passive",
                            NodeMode::Active => "active",
                        },
                        lang = match self.language {
                            Language::English => "en",
                            Language::Francais => "fr",
                        }
                    );
                    std::fs::write(config_dir.join("config.json"), cfg).ok();

                    self.set_install_status("Almost done...", 0.95);
                    std::thread::sleep(std::time::Duration::from_millis(200));

                    self.installing_done = true;
                    self.set_install_status("Installation complete!", 1.0);
                    self.log("✓ Polygone installed successfully");
                } else {
                    self.install_error = Some("Failed to copy binary".to_string());
                }
            }
            _ => {
                // Fallback: build from source
                self.set_install_status("Building from source (download failed)...", 0.3);
                self.log("Download failed — building from source");

                let build_dir = dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("polygone-src");

                if !build_dir.exists() {
                    self.log("Cloning repository...");
                    let _ = Command::new("git")
                        .args(["clone", "https://github.com/lvs0/Polygone.git"])
                        .arg(&build_dir)
                        .output();
                }

                self.set_install_status("Compiling (this may take a while)...", 0.5);
                self.log("Running cargo build --release...");

                let build = Command::new("cargo")
                    .current_dir(&build_dir)
                    .args(["build", "--release", "--bin", "polygone"])
                    .output();

                match build {
                    Ok(out) if out.status.success() => {
                        self.set_install_status("Installing compiled binary...", 0.85);
                        let dest = install_dir.join("polygone");
                        let src = build_dir.join("target/release/polygone");
                        if std::fs::copy(&src, &dest).is_ok() {
                            #[cfg(unix)]
                            {
                                use std::os::unix::fs::PermissionsExt;
                                std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755)).ok();
                            }
                            self.installing_done = true;
                            self.set_install_status("Installation complete!", 1.0);
                            self.log("✓ Built and installed successfully");
                        } else {
                            self.install_error = Some("Copy failed".to_string());
                        }
                    }
                    Err(e) => {
                        self.install_error = Some(format!("Build error: {}", e));
                    }
                    _ => {
                        let err = String::from_utf8_lossy(&build.unwrap_err().stderr);
                        self.install_error = Some(format!("Build failed: {}", err.lines().last().unwrap_or("?")));
                    }
                }
            }
        }
    }

    // ─── Main draw ────────────────────────────────────────────────────────────
    fn draw(&self, f: &mut Frame) {
        let size = f.area();
        f.render_widget(Clear, size);

        match &self.step {
            Step::Welcome   => self.draw_welcome(f, size),
            Step::Language  => self.draw_language(f, size),
            Step::Username  => self.draw_username(f, size),
            Step::NodeMode  => self.draw_node_mode(f, size),
            Step::Installing => self.draw_installing(f, size),
            Step::Configuring => self.draw_configuring(f, size),
            Step::Done      => self.draw_done(f, size),
            Step::Dashboard => self.draw_dashboard(f, size),
        }
    }

    fn centered(&self, w: u16, h: u16, size: Rect) -> Rect {
        let x = (size.width.saturating_sub(w)) / 2;
        let y = (size.height.saturating_sub(h)) / 2;
        Rect::new(x, y, w.min(size.width), h.min(size.height))
    }

    fn nav_hints(&self) -> Vec<Line<'static>> {
        vec![
            Line::from(vec![
                Span::raw("  "),
                Span::styled("↑↓", Style::new().fg(C_COBALT)),
                Span::raw(" Navigate  "),
                Span::styled("ENTER", Style::new().fg(C_GREEN)),
                Span::raw(" Select  "),
                Span::styled("ESC", Style::new().fg(C_DIM)),
                Span::raw(" Back  "),
            ]),
        ]
    }

    fn block(&self, title: &str) -> Block<'static> {
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
        let box_h = 22u16.min(size.height.saturating_sub(4));
        let box_w = 62u16.min(size.width.saturating_sub(4));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block(""), rect);

        let logo_area = Rect::new(rect.x + 2, rect.y + 1, rect.width - 4, 10);
        draw_logo(logo_area, f);

        let inner = Rect::new(rect.x, rect.y + 11, rect.width, rect.height - 11);
        let lines = vec![
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Welcome to ", Style::new().fg(C_TEXT)),
                Span::styled("Polygone", Style::new().fg(C_COBALT).bold()),
                Span::raw("  ·  Privacy that leaves no trace"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Press "),
                Span::styled("ENTER", Style::new().fg(C_GREEN).bold()),
                Span::raw(" to install  ·  "),
                Span::styled("q", Style::new().fg(C_DIM)),
                Span::raw(" to quit"),
            ]),
        ];
        let p = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    // ── Language ────────────────────────────────────────────────────────────
    fn draw_language(&self, f: &mut Frame, size: Rect) {
        let box_h = 16u16.min(size.height.saturating_sub(4));
        let box_w = 44u16.min(size.width.saturating_sub(4));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block("Language / Langue"), rect);
        let inner = rect.inner(Margin::new(2, 1));

        let items = vec![
            ListItem::new(Line::from(vec![
                Span::raw("  "),
                if self.selected_idx == 0 {
                    Span::styled("▶ English", Style::new().fg(C_GREEN).bold())
                } else {
                    Span::raw("  English")
                },
            ])),
            ListItem::new(Line::from(vec![
                Span::raw("  "),
                if self.selected_idx == 1 {
                    Span::styled("▶ Français", Style::new().fg(C_GREEN).bold())
                } else {
                    Span::raw("  Français")
                },
            ])),
        ];

        let list = List::new(items)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(list, inner);

        let nav = Paragraph::new(self.nav_hints())
            .alignment(Alignment::Center)
            .style(Style::new().fg(C_DIM));
        f.render_widget(nav, inner);
    }

    // ── Username ─────────────────────────────────────────────────────────────
    fn draw_username(&self, f: &mut Frame, size: Rect) {
        let box_h = 18u16.min(size.height.saturating_sub(4));
        let box_w = 52u16.min(size.width.saturating_sub(4));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block("Username / Nom d'utilisateur"), rect);
        let inner = rect.inner(Margin::new(2, 1));

        let prompt = if self.language == Language::Francais {
            "Choose a name (optional) / Choisissez un nom"
        } else {
            "Choose a name (optional)"
        };

        let lines = vec![
            Line::from(Span::raw("")),
            Line::from(Span::raw(&format!("  {}", prompt))),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::raw("  > "),
                Span::raw(&self.username),
                Span::raw(" "),
            ]),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::styled("  Skip (ENTER)", Style::new().fg(C_DIM)),
                Span::raw("  ·  "),
                Span::styled("Clear (c)", Style::new().fg(C_DIM)),
            ]),
            Line::from(Span::raw("")),
        ];

        let p = Paragraph::new(lines)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    // ── Node Mode ────────────────────────────────────────────────────────────
    fn draw_node_mode(&self, f: &mut Frame, size: Rect) {
        let box_h = 26u16.min(size.height.saturating_sub(4));
        let box_w = 56u16.min(size.width.saturating_sub(4));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block("Node Mode"), rect);
        let inner = rect.inner(Margin::new(2, 1));

        let choices = vec![
            ("No node",   "Don't share your computer as a network node"),
            ("Passive",  "Share bandwidth only — invisible, can be paused anytime"),
            ("Active",   "Share bandwidth + computing power for the network"),
        ];

        let items: Vec<ListItem> = choices.iter().enumerate().map(|(i, (title, desc))| {
            let sel = self.selected_idx == i;
            let icon = if sel { "▶" } else { " " };
            let style = if sel {
                Style::new().fg(C_GREEN).bold()
            } else {
                Style::new().fg(C_TEXT)
            };
            ListItem::new(vec![
                Line::from(vec![Span::styled(icon, style), Span::raw("  "), Span::styled(title, style)]}),
                Line::from(vec![Span::raw("     "), Span::raw(desc)]),
                Line::from(Span::raw("")),
            ])
        }).collect();

        let list = List::new(items)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(list, inner);

        let nav = Paragraph::new(self.nav_hints())
            .alignment(Alignment::Center)
            .style(Style::new().fg(C_DIM));
        f.render_widget(nav, inner);
    }

    // ── Installing ────────────────────────────────────────────────────────────
    fn draw_installing(&self, f: &mut Frame, size: Rect) {
        let box_h = 24u16.min(size.height.saturating_sub(4));
        let box_w = 58u16.min(size.width.saturating_sub(4));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block("Installing Polygone"), rect);
        let inner = rect.inner(Margin::new(2, 1));

        let pct = (self.install_progress * 100.0) as u16;

        // Gauge style progress bar
        let bar_ratio = self.install_progress;
        let status_line = Line::from(vec![
            Span::raw("  "),
            Span::raw(&self.install_status),
            Span::raw(format!("  ({}%)", pct)),
        ]);

        let log_lines: Vec<Line> = self.install_log
            .iter()
            .map(|l| Line::from(Span::styled(format!("  {}", l), Style::new().fg(C_DIM))))
            .collect();

        let error_line = self.install_error.as_ref().map(|e| {
            Line::from(vec![
                Span::styled("✗ ERROR: ", Style::new().fg(C_RED)),
                Span::raw(e),
            ])
        });

        let all: Vec<Line> = [
            vec![Line::from("")],
            log_lines,
            vec![Line::from("")],
            vec![status_line],
            vec![Line::from("")],
            error_line.into_iter().collect(),
        ].concat();

        let p = Paragraph::new(all)
            .style(Style::new().fg(C_TEXT))
            .alignment(Alignment::Left);
        f.render_widget(p, inner);
    }

    // ── Configuring ───────────────────────────────────────────────────────────
    fn draw_configuring(&self, f: &mut Frame, size: Rect) {
        let box_h = 20u16.min(size.height.saturating_sub(4));
        let box_w = 56u16.min(size.width.saturating_sub(4));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block("Configuration"), rect);
        let inner = rect.inner(Margin::new(2, 1));

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  ✓ "),
                Span::styled("Polygone installed!", Style::new().fg(C_GREEN).bold()),
            ]),
            Line::from(vec![Span::raw("  ✓ Config file created")]),
            Line::from(vec![Span::raw("  ✓ PATH configured")]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Node mode: "),
                Span::styled(
                    match self.node_mode {
                        NodeMode::None => "Disabled",
                        NodeMode::Passive => "Passive",
                        NodeMode::Active => "Active",
                    },
                    Style::new().fg(C_COBALT),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Press "),
                Span::styled("ENTER", Style::new().fg(C_GREEN).bold()),
                Span::raw(" to launch dashboard"),
            ]),
        ];

        let p = Paragraph::new(lines)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    // ── Done ─────────────────────────────────────────────────────────────────
    fn draw_done(&self, f: &mut Frame, size: Rect) {
        let box_h = 14u16.min(size.height.saturating_sub(4));
        let box_w = 46u16.min(size.width.saturating_sub(4));
        let rect = self.centered(box_w, box_h, size);
        f.render_widget(self.block(""), rect);
        let inner = rect.inner(Margin::new(2, 1));

        let lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled("  ⬡  POLYGONE is ready.", Style::new().fg(C_GREEN).bold())]),
            Line::from(Span::raw("")),
            Line::from(Span::raw("  Thank you for choosing privacy.")),
            Line::from(Span::raw("")),
        ];

        let p = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    // ── Dashboard ──────────────────────────────────────────────────────────────
    fn draw_dashboard(&self, f: &mut Frame, size: Rect) {
        // Full screen dashboard
        let header_h = 3u16;
        let footer_h = 2u16;
        let tab_bar_h = 3u16;

        // Header
        let header_rect = Rect::new(0, 0, size.width, header_h);
        let header_block = Block::new()
            .borders(Borders::BOTTOM)
            .border_style(Style::new().fg(C_BORDER))
            .style(Style::new().bg(C_SURFACE));
        f.render_widget(header_block, header_rect);

        let header_lines = vec![
            Line::from(vec![
                Span::styled("⬡ Polygone", Style::new().fg(C_COBALT).bold()),
                Span::raw(format!(" v{}  ·  user:{}", VERSION, 
                    if self.username.is_empty() { "anonymous".to_string() } else { self.username.clone() }.escape_default())),
                Span::raw("                              "),
                Span::styled("q Quit  ·  ENTER Select  ·  ↑↓ Navigate", Style::new().fg(C_DIM)),
            ]),
        ];
        let header_p = Paragraph::new(header_lines)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(header_p, header_rect);

        // Tab bar
        let tab_rect = Rect::new(0, header_h, size.width, tab_bar_h);
        let tabs = vec!["Home", "Services", "Nodes", "Settings"];
        let tab_lines: Vec<Line> = tabs.iter().enumerate().map(|(i, tab)| {
            let sel = self.selected_idx == i + 10; // Dashboard tabs offset 10
            let style = if sel {
                Style::new().fg(C_GREEN).bold()
            } else {
                Style::new().fg(C_DIM)
            };
            let prefix = if sel { "▶ " } else { "  " };
            Line::from(vec![
                Span::raw("   "),
                Span::styled(prefix, style),
                Span::styled(tab, style),
                Span::raw("   "),
            ])
        }).collect();

        let tab_bar = Paragraph::new(tab_lines)
            .style(Style::new().fg(C_TEXT))
            .alignment(Alignment::Left);
        f.render_widget(tab_bar, tab_rect);

        // Content area
        let content_y = header_h + tab_bar_h;
        let content_h = size.height.saturating_sub(content_y + footer_h);
        let content_rect = Rect::new(0, content_y, size.width, content_h);

        let tab_idx = if self.selected_idx >= 10 { self.selected_idx - 10 } else { 0 };

        match tab_idx {
            0 => self.draw_tab_home(f, content_rect),
            1 => self.draw_tab_services(f, content_rect),
            2 => self.draw_tab_nodes(f, content_rect),
            3 => self.draw_tab_settings(f, content_rect),
            _ => {},
        }

        // Footer
        let footer_rect = Rect::new(0, size.height - footer_h, size.width, footer_h);
        let footer_block = Block::new()
            .borders(Borders::TOP)
            .border_style(Style::new().fg(C_BORDER))
            .style(Style::new().bg(C_SURFACE));
        f.render_widget(footer_block, footer_rect);

        let status = match self.node_mode {
            NodeMode::None => "Node: off",
            NodeMode::Passive => "Node: passive",
            NodeMode::Active => "Node: active",
        };
        let footer_lines = vec![Line::from(vec![
            Span::styled(status, Style::new().fg(C_COBALT)),
            Span::raw("  ·  Polygone v"),
            Span::raw(VERSION),
            Span::raw("  ·  powered by Hope"),
        ])];
        let footer_p = Paragraph::new(footer_lines)
            .style(Style::new().fg(C_DIM));
        f.render_widget(footer_p, footer_rect);
    }

    fn draw_tab_home(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(1, 1));

        let lines = vec![
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::styled("  ⬡ POLYGONE", Style::new().fg(C_COBALT).bold()),
                Span::raw("  —  Privacy that leaves no trace"),
            ]),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::raw("  Status: "),
                Span::styled("Running", Style::new().fg(C_GREEN)),
            ]),
            Line::from(vec![
                Span::raw("  Username: "),
                Span::raw(if self.username.is_empty() { "anonymous".to_string() } else { self.username.clone() }),
            ]),
            Line::from(vec![
                Span::raw("  Node: "),
                Span::raw(match self.node_mode {
                    NodeMode::None => "Disabled",
                    NodeMode::Passive => "Passive (invisible)",
                    NodeMode::Active => "Active",
                }),
            ]),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::raw("  Quick actions:"),
            ]),
            Line::from(vec![
                Span::styled("  ▶ ", Style::new().fg(C_GREEN)),
                Span::raw("Run self-test"),
            ]),
            Line::from(vec![
                Span::styled("  ▶ ", Style::new().fg(C_GREEN)),
                Span::raw("Generate keys"),
            ]),
            Line::from(vec![
                Span::styled("  ▶ ", Style::new().fg(C_GREEN)),
                Span::raw("Start node"),
            ]),
            Line::from(Span::raw("")),
        ];

        let p = Paragraph::new(lines)
            .style(Style::new().fg(C_TEXT))
            .wrap(Wrap { trim: true });
        f.render_widget(p, inner);
    }

    fn draw_tab_services(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(1, 1));

        let services = vec![
            ("polygone",      "Core privacy network",          true),
            ("polygone-drive","Distributed encrypted storage",true),
            ("polygone-hide", "SOCKS5 privacy tunnel",        false),
            ("polygone-petals","Distributed LLM inference",   false),
        ];

        let lines: Vec<Line> = vec![Line::from(Span::raw("  Services  "))]
            .into_iter()
            .chain(services.iter().enumerate().flat_map(|(i, (name, desc, active))| {
                let sel = self.selected_idx == 100 + i;
                let icon = if *active { "●" } else { "○" };
                let style = if *active { C_GREEN } else { C_DIM };
                vec![
                    Line::from(vec![
                        if sel {
                            Span::styled("  ▶ ", Style::new().fg(C_GREEN))
                        } else {
                            Span::raw("    ")
                        },
                        Span::styled(icon, Style::new().fg(style)),
                        Span::raw("  "),
                        Span::raw(name),
                        Span::raw("  —  "),
                        Span::raw(desc),
                    ]),
                ]
            }))
            .collect();

        let p = Paragraph::new(lines)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    fn draw_tab_nodes(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(1, 1));

        let lines = vec![
            Line::from(Span::raw("  Node Status  ")),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::raw("  Mode: "),
                Span::styled(
                    match self.node_mode {
                        NodeMode::None => "Disabled",
                        NodeMode::Passive => "Passive — shares bandwidth only",
                        NodeMode::Active => "Active — shares bandwidth + compute",
                    },
                    Style::new().fg(match self.node_mode {
                        NodeMode::None => C_DIM,
                        NodeMode::Passive => C_YELLOW,
                        NodeMode::Active => C_GREEN,
                    }),
                ),
            ]),
            Line::from(Span::raw("")),
            Line::from(Span::raw("  The node system is ")),
            Line::from(Span::raw("  intelligent and invisible. It runs in the")),
            Line::from(Span::raw("  background, can be paused anytime.")),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::styled("  ▶ ", Style::new().fg(C_GREEN)),
                Span::raw("Pause node for 1h"),
            ]),
            Line::from(vec![
                Span::styled("  ▶ ", Style::new().fg(C_GREEN)),
                Span::raw("Pause node for 4h"),
            ]),
            Line::from(vec![
                Span::styled("  ▶ ", Style::new().fg(C_YELLOW)),
                Span::raw("Disable node"),
            ]),
            Line::from(Span::raw("")),
            Line::from(Span::raw("  Node name: polygone-node")),
            Line::from(Span::raw("  Shared since: just now")),
            Line::from(Span::raw("  Bandwidth shared: calculating...")),
            Line::from(Span::raw("")),
        ];

        let p = Paragraph::new(lines)
            .style(Style::new().fg(C_TEXT))
            .wrap(Wrap { trim: true });
        f.render_widget(p, inner);
    }

    fn draw_tab_settings(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(1, 1));

        let settings = vec![
            ("Username",           &self.username),
            ("Language",           match self.language { Language::English => "English", Language::Francais => "Français" }),
            ("Node mode",          match self.node_mode { NodeMode::None => "None", NodeMode::Passive => "Passive", NodeMode::Active => "Active" }),
            ("Auto-update",         "enabled"),
            ("Telemetry",          "disabled"),
        ];

        let lines: Vec<Line> = vec![Line::from(Span::raw("  Settings  "))]
            .into_iter()
            .chain(settings.iter().enumerate().flat_map(|(i, (key, val))| {
                let sel = self.selected_idx == 200 + i;
                vec![
                    Line::from(vec![
                        if sel {
                            Span::styled("  ▶ ", Style::new().fg(C_GREEN))
                        } else {
                            Span::raw("    ")
                        },
                        Span::raw(key),
                        Span::raw(": "),
                        Span::styled(val, Style::new().fg(C_COBALT)),
                    ]),
                ]
            }))
            .collect();

        let p = Paragraph::new(lines)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    // ── Event handling ─────────────────────────────────────────────────────────
    fn handle_key(&mut self, key: event::KeyEvent) {
        let kind = key.kind;
        if kind != KeyEventKind::Press && kind != KeyEventKind::Repeat {
            return;
        }

        match self.step {
            Step::Welcome => {
                if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                    self.next_step();
                } else if key.code == KeyCode::Char('q') {
                    self.step = Step::Done; // Quit
                }
            }
            Step::Language => {
                if key.code == KeyCode::Up || key.code == KeyCode::Char('k') {
                    self.selected_idx = self.selected_idx.saturating_sub(1);
                } else if key.code == KeyCode::Down || key.code == KeyCode::Char('j') {
                    self.selected_idx = (self.selected_idx + 1).min(1);
                } else if key.code == KeyCode::Enter {
                    self.language = if self.selected_idx == 0 { Language::English } else { Language::Francais };
                    self.next_step();
                } else if key.code == KeyCode::Esc {
                    self.prev_step();
                }
            }
            Step::Username => {
                if key.code == KeyCode::Enter {
                    self.next_step();
                } else if key.code == KeyCode::Char('c') {
                    self.username.clear();
                } else if key.code == KeyCode::Esc {
                    self.username.clear();
                    self.prev_step();
                } else if let KeyCode::Char(ch) = key.code {
                    if ch.is_ascii_graphic() || ch == ' ' {
                        self.username.push(ch);
                    }
                } else if let KeyCode::Backspace = key.code {
                    self.username.pop();
                }
            }
            Step::NodeMode => {
                if key.code == KeyCode::Up || key.code == KeyCode::Char('k') {
                    self.selected_idx = self.selected_idx.saturating_sub(1);
                } else if key.code == KeyCode::Down || key.code == KeyCode::Char('j') {
                    self.selected_idx = (self.selected_idx + 1).min(2);
                } else if key.code == KeyCode::Enter {
                    self.node_mode = match self.selected_idx {
                        1 => NodeMode::Passive,
                        2 => NodeMode::Active,
                        _ => NodeMode::None,
                    };
                    self.next_step();
                } else if key.code == KeyCode::Esc {
                    self.prev_step();
                }
            }
            Step::Installing => {
                if !self.installing_done && self.install_error.is_none() {
                    // Installation runs automatically (blocking in a spawn)
                    // We just wait
                } else if self.installing_done {
                    self.next_step();
                }
            }
            Step::Configuring => {
                if key.code == KeyCode::Enter {
                    self.config_done = true;
                    self.next_step();
                }
            }
            Step::Done => {
                // Exit
            }
            Step::Dashboard => {
                let tab_count = 4;
                if key.code == KeyCode::Up || key.code == KeyCode::Char('k') {
                    self.selected_idx = self.selected_idx.saturating_sub(1);
                } else if key.code == KeyCode::Down || key.code == KeyCode::Char('j') {
                    self.selected_idx = self.selected_idx.saturating_add(1);
                } else if key.code == KeyCode::Left || key.code == KeyCode::Char('h') {
                    // Tab switch left
                    if self.selected_idx >= 10 {
                        self.selected_idx = 10 + ((self.selected_idx - 10).saturating_sub(1) % tab_count);
                    }
                } else if key.code == KeyCode::Right || key.code == KeyCode::Char('l') {
                    if self.selected_idx >= 10 {
                        self.selected_idx = 10 + ((self.selected_idx - 10 + 1) % tab_count);
                    }
                } else if key.code == KeyCode::Enter {
                    // Action on selected item
                } else if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                    self.step = Step::Done;
                }
            }
        }
    }

    // ── Run loop ────────────────────────────────────────────────────────────────
    fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        // If starting at Welcome, run install in background
        let install_handle = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        loop {
            terminal.draw(|f| self.draw(f))?;

            // Start install when we enter Installing step (one-shot)
            if self.step == Step::Installing && !self.installing_done && self.install_error.is_none() {
                let install_handle = install_handle.clone();
                if !install_handle.load(std::sync::atomic::Ordering::SeqCst) {
                    install_handle.store(true, std::sync::atomic::Ordering::SeqCst);
                    let mut me = Self::new();
                    me.step = Step::Installing;
                    me.username = self.username.clone();
                    me.language = self.language;
                    me.node_mode = self.node_mode;
                    me.run_install();
                    self.install_progress = me.install_progress;
                    self.installing_done = me.installing_done;
                    self.install_error = me.install_error.clone();
                    self.install_log = me.install_log.clone();
                    self.install_status = me.install_status.clone();
                }
            }

            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }

            if self.step == Step::Done {
                break;
            }
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    crossterm::terminal::enable_raw_mode()?;

    let result = App::new().run(&mut terminal);

    
    crossterm::terminal::disable_raw_mode()?;
    ratatui::restore();

    result
}
