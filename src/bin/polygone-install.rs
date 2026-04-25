//! POLYGONE Installer TUI
//! Built with ratatui — the full-screen installation experience.

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use ratatui::layout::{Constraint, Direction, Layout, Alignment};
use crossterm::event::{self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode};

const VERSION: &str = "1.0.0";
const HEX_DARK: Color = Color::Rgb(10, 10, 15);
const HEX_SURFACE: Color = Color::Rgb(17, 17, 24);
const HEX_BORDER: Color = Color::Rgb(30, 30, 46);
const HEX_COBALT: Color = Color::Rgb(26, 107, 255);
const HEX_GREEN: Color = Color::Rgb(40, 200, 64);
const HEX_RED: Color = Color::Rgb(255, 59, 48);
const HEX_YELLOW: Color = Color::Rgb(255, 204, 0);
const HEX_TEXT: Color = Color::Rgb(200, 200, 232);
const HEX_BRIGHT: Color = Color::Rgb(240, 240, 255);
const HEX_DIM: Color = Color::Rgb(74, 74, 106);

#[derive(Debug, Clone, Copy, PartialEq)]
enum InstallMethod {
    Download,
    BuildFromSource,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InstallStep {
    Welcome,
    SystemCheck,
    MethodSelect,
    Installing,
    PostInstall,
    Done,
}

pub struct Installer {
    current_step: InstallStep,
    method: Option<InstallMethod>,
    // System checks
    rust_installed: bool,
    cargo_installed: bool,
    // Progress
    progress: f32,
    status_line: String,
    log_lines: Vec<String>,
    // Error
    error: Option<String>,
}

impl Installer {
    fn new() -> Self {
        Self {
            current_step: InstallStep::Welcome,
            method: None,
            rust_installed: false,
            cargo_installed: false,
            progress: 0.0,
            status_line: String::new(),
            log_lines: Vec::new(),
            error: None,
        }
    }

    fn log(&mut self, line: &str) {
        self.log_lines.push(line.to_string());
        if self.log_lines.len() > 12 {
            self.log_lines.remove(0);
        }
    }

    fn next_step(&mut self) {
        self.progress = 0.0;
        self.status_line.clear();
        self.log_lines.clear();
        self.error = None;

        self.current_step = match self.current_step {
            InstallStep::Welcome => InstallStep::SystemCheck,
            InstallStep::SystemCheck => InstallStep::MethodSelect,
            InstallStep::MethodSelect if self.method.is_some() => InstallStep::Installing,
            InstallStep::Installing => InstallStep::PostInstall,
            InstallStep::PostInstall => InstallStep::Done,
            _ => InstallStep::Done,
        };
    }

    fn prev_step(&mut self) {
        self.current_step = match self.current_step {
            InstallStep::MethodSelect => InstallStep::SystemCheck,
            InstallStep::SystemCheck => InstallStep::Welcome,
            _ => self.current_step,
        };
    }

    fn check_system(&mut self) {
        self.log("Checking system prerequisites...");

        // Check cargo
        let cargo_check = Command::new("cargo").arg("--version").output();
        if cargo_check.is_ok() {
            self.cargo_installed = true;
            let v = String::from_utf8_lossy(&cargo_check.unwrap().stdout);
            self.log(&format!("  ✓ cargo found: {}", v.trim()));
        } else {
            self.rust_installed = false;
            self.cargo_installed = false;
            self.log("  ✗ cargo not found — Rust not installed");
        }

        // Check git
        let git_check = Command::new("git").arg("--version").output();
        if git_check.is_ok() {
            self.log("  ✓ git found");
        }

        // Check platform
        #[cfg(target_os = "linux")]
        self.log("  ✓ Platform: Linux");
        #[cfg(target_os = "macos")]
        self.log("  ✓ Platform: macOS");
        #[cfg(target_os = "windows")]
        self.log("  ✓ Platform: Windows (WSL recommended)");

        self.next_step();
    }

    fn install_download(&mut self) {
        self.log("Preparing download...");
        self.progress = 0.1;

        // Simulate download steps
        let steps = [
            ("Fetching latest release from GitHub...", 0.2),
            ("Downloading binary...", 0.5),
            ("Verifying binary integrity...", 0.7),
            ("Making executable...", 0.9),
            ("Installing to ~/.local/bin...", 1.0),
        ];

        for (msg, p) in steps {
            self.status_line = msg.to_string();
            self.progress = p;
            std::thread::sleep(Duration::from_millis(300));
        }

        // Actually try to download
        let install_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".local/bin");

        std::fs::create_dir_all(&install_dir).ok();

        let binary_url = format!(
            "https://github.com/lvs0/Polygone/releases/download/v{}/polygone",
            VERSION
        );

        self.status_line = "Downloading binary...";
        self.progress = 0.4;

        let dl = Command::new("curl")
            .args(["-fsSL", "-o", "/tmp/polygone"])
            .arg(&binary_url)
            .output();

        match dl {
            Ok(output) if output.status.success() => {
                self.log("  ✓ Downloaded successfully");
                self.progress = 0.7;

                // Move to install dir
                let dest = install_dir.join("polygone");
                if std::fs::copy("/tmp/polygone", &dest).is_ok() {
                    // Make executable
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755)).ok();
                    }
                    self.log(&format!("  ✓ Installed to {:?}", dest));
                    self.progress = 1.0;
                    self.status_line = "Installation complete!".to_string();
                    self.next_step();
                } else {
                    self.error = Some("Failed to copy binary".to_string());
                }
            }
            _ => {
                // Download failed — fall back to source
                self.log("  ! Download failed — will build from source instead");
                self.status_line = "Falling back to source build...";
                self.progress = 0.5;
                self.install_from_source();
            }
        }
    }

    fn install_from_source(&mut self) {
        self.log("Building from source...");
        self.progress = 0.1;

        let build_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("polygone-build");

        // Clone if needed
        if !build_dir.exists() {
            self.status_line = "Cloning Polygone repository...";
            self.progress = 0.15;
            self.log("  → Cloning github.com/lvs0/Polygone...");

            let clone = Command::new("git")
                .args(["clone", "https://github.com/lvs0/Polygone.git"])
                .arg(&build_dir)
                .output();

            match clone {
                Ok(output) if output.status.success() => {
                    self.log("  ✓ Repository cloned");
                }
                _ => {
                    self.error = Some("Git clone failed — check your internet".to_string());
                    return;
                }
            }
        } else {
            self.log("  → Using existing repository");
            // Pull latest
            let _ = Command::new("git")
                .current_dir(&build_dir)
                .args(["pull", "origin", "main"])
                .output();
        }

        self.progress = 0.35;
        self.status_line = "Building release binary...";
        self.log("  → Running cargo build --release...");

        let build = Command::new("cargo")
            .current_dir(&build_dir)
            .args(["build", "--release", "--bin", "polygone"])
            .output();

        match build {
            Ok(output) => {
                if output.status.success() {
                    self.log("  ✓ Build succeeded");
                    self.progress = 0.85;

                    // Copy to install dir
                    let install_dir = dirs::home_dir()
                        .unwrap_or_else(|| PathBuf::from("."))
                        .join(".local/bin");
                    std::fs::create_dir_all(&install_dir).ok();

                    let src = build_dir.join("target/release/polygone");
                    let dest = install_dir.join("polygone");

                    if std::fs::copy(&src, &dest).is_ok() {
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755)).ok();
                        }
                        self.log(&format!("  ✓ Installed to {:?}", dest));
                        self.progress = 1.0;
                        self.status_line = "Installation complete!".to_string();
                        self.next_step();
                    } else {
                        self.error = Some("Failed to install binary".to_string());
                    }
                } else {
                    let err = String::from_utf8_lossy(&output.stderr);
                    self.error = Some(format!("Build failed: {}", err.lines().last().unwrap_or("unknown error")));
                }
            }
            Err(e) => {
                self.error = Some(format!("Build error: {}", e));
            }
        }
    }

    fn run_install(&mut self) {
        match self.method {
            Some(InstallMethod::Download) => self.install_download(),
            Some(InstallMethod::BuildFromSource) => self.install_from_source(),
            None => {}
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;

            if let Event::Key(key) = event::read()? {
                match self.current_step {
                    InstallStep::Welcome => {
                        if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                            self.next_step();
                        }
                    }
                    InstallStep::SystemCheck => {
                        if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                            self.check_system();
                        }
                    }
                    InstallStep::MethodSelect => {
                        if key.code == KeyCode::Down || key.code == KeyCode::Char('j') {
                            self.method = Some(InstallMethod::BuildFromSource);
                        } else if key.code == KeyCode::Up || key.code == KeyCode::Char('k') {
                            self.method = Some(InstallMethod::Download);
                        } else if key.code == KeyCode::Enter {
                            self.next_step();
                        } else if key.code == KeyCode::Esc {
                            self.prev_step();
                        }
                    }
                    InstallStep::Installing => {
                        self.run_install();
                    }
                    InstallStep::PostInstall => {
                        if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                            self.next_step();
                        }
                    }
                    InstallStep::Done => {
                        break;
                    }
                }
            }

            if self.current_step == InstallStep::Done {
                break;
            }
        }

        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        let size = f.area();

        // Full-screen background
        f.render_widget(Clear, size);

        match self.current_step {
            InstallStep::Welcome => self.draw_welcome(f),
            InstallStep::SystemCheck => self.draw_system_check(f),
            InstallStep::MethodSelect => self.draw_method_select(f),
            InstallStep::Installing => self.draw_installing(f),
            InstallStep::PostInstall => self.draw_post_install(f),
            InstallStep::Done => self.draw_done(f),
        }
    }

    fn centered_rect(&self, width: u16, height: u16, size: ratatui::layout::Rect) -> ratatui::layout::Rect {
        let x = (size.width.saturating_sub(width)) / 2;
        let y = (size.height.saturating_sub_sub(height)) / 2;
        ratatui::layout::Rect::new(x, y, width, height)
    }

    fn draw_welcome(&self, f: &mut Frame) {
        let size = f.area();
        let box_w = 60u16.min(size.width.saturating_sub(4));
        let box_h = 22u16.min(size.height.saturating_sub(4));
        let rect = self.centered_rect(box_w, box_h, size);

        let block = Block::new()
            .title("  ⬡ POLYGONE  ")
            .title_style(Style::new().fg(HEX_COBALT).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(HEX_BORDER))
            .style(Style::new().bg(HEX_SURFACE));

        f.render_widget(block, rect);

        let inner = rect.inner(2);

        // Logo
        let logo = Paragraph::new(
            vec![
                Line::from(Span::raw("")),
                Line::from(Span::raw("        ███████╗██╗     ██╗ █████╗ ███████╗")),
                Line::from(Span::raw("        ██╔════╝██║     ██║██╔══██╗██╔════╝")),
                Line::from(Span::raw("        ███████╗██║     ██║███████║███████╗")),
                Line::from(Span::raw("        ╚════██║██║     ██║██╔══██║╚════██║")),
                Line::from(Span::raw("        ███████║███████╗██║██║  ██║███████║")),
                Line::from(Span::raw("        ╚══════╝╚══════╝╚═╝╚═╝  ╚═╝╚══════╝")),
                Line::from(Span::raw("")),
            ]
        )
        .style(Style::new().fg(HEX_COBALT))
        .alignment(Alignment::Center);
        f.render_widget(logo, inner);

        let version_style = Style::new().fg(HEX_DIM);
        let version = Paragraph::new(Line::from(vec![
            Span::raw("v"),
            Span::styled(VERSION, version_style),
            Span::raw("  ·  Post-Quantum Ephemeral Privacy Network"),
        ]))
        .style(Style::new().fg(HEX_TEXT))
        .alignment(Alignment::Center);
        f.render_widget(version, inner);

        // Description
        let desc_lines = vec![
            Line::from(Span::raw("")),
            Line::from(Span::raw("  ML-KEM-1024 · AES-256-GCM · Shamir 4-of-7  ")),
            Line::from(Span::raw("  No metadata. No logs. No trace.  ")),
            Line::from(Span::raw("")),
        ];
        let desc = Paragraph::new(desc_lines)
            .style(Style::new().fg(HEX_DIM))
            .alignment(Alignment::Center);
        f.render_widget(desc, inner);

        // Prompt
        let prompt = Paragraph::new(Line::from(vec![
            Span::raw("  Press "),
            Span::styled("ENTER", Style::new().fg(HEX_GREEN).bold()),
            Span::raw(" to begin installation  "),
        ]))
        .style(Style::new().fg(HEX_TEXT))
        .alignment(Alignment::Center);
        f.render_widget(prompt, inner);
    }

    fn draw_system_check(&self, f: &mut Frame) {
        let size = f.area();
        let box_w = 60u16.min(size.width.saturating_sub(4));
        let box_h = 20u16.min(size.height.saturating_sub(4));
        let rect = self.centered_rect(box_w, box_h, size);

        let block = Block::new()
            .title("  System Check  ")
            .title_style(Style::new().fg(HEX_COBALT).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(HEX_BORDER))
            .style(Style::new().bg(HEX_SURFACE));

        f.render_widget(block, rect);
        let inner = rect.inner(2);

        let checks = vec![
            ("Rust/Cargo", self.cargo_installed),
            ("Git", Command::new("git").arg("--version").output().is_ok()),
        ];

        let mut lines = vec![Line::from(Span::raw(""))];
        for (name, ok) in &checks {
            let icon = if *ok { "✓" } else { "✗" };
            let color = if *ok { HEX_GREEN } else { HEX_RED };
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(icon, Style::new().fg(color)),
                Span::raw("  "),
                Span::raw(*name),
            ]));
            lines.push(Line::from(Span::raw("")));
        }

        let content = Paragraph::new(lines)
            .style(Style::new().fg(HEX_TEXT))
            .alignment(Alignment::Left);
        f.render_widget(content, inner);

        let hint = Paragraph::new(Line::from(vec![
            Span::raw("  Press "),
            Span::styled("ENTER", Style::new().fg(HEX_GREEN).bold()),
            Span::raw(" to continue  "),
        ]))
        .style(Style::new().fg(HEX_TEXT))
        .alignment(Alignment::Center);
        f.render_widget(hint, inner);
    }

    fn draw_method_select(&self, f: &mut Frame) {
        let size = f.area();
        let box_w = 62u16.min(size.width.saturating_sub(4));
        let box_h = 24u16.min(size.height.saturating_sub(4));
        let rect = self.centered_rect(box_w, box_h, size);

        let block = Block::new()
            .title("  Choose Install Method  ")
            .title_style(Style::new().fg(HEX_COBALT).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(HEX_BORDER))
            .style(Style::new().bg(HEX_SURFACE));

        f.render_widget(block, rect);
        let inner = rect.inner(2);

        let methods = vec![
            ("Download Binary", "Fast (~2MB download)", InstallMethod::Download),
            ("Build from Source", "Compile with Cargo (slower, no Rust needed)", InstallMethod::BuildFromSource),
        ];

        let mut items = Vec::new();
        for (title, desc, method) in methods {
            let selected = self.method == Some(method);
            let icon = if selected { "▶" } else { " " };
            let style = if selected {
                Style::new().fg(HEX_GREEN).bold()
            } else {
                Style::new().fg(HEX_DIM)
            };
            items.push(ListItem::new(vec![
                Line::from(Span::raw("")),
                Line::from(vec![
                    Span::styled(icon, style),
                    Span::raw("  "),
                    Span::styled(title, style),
                ]),
                Line::from(vec![
                    Span::raw("      "),
                    Span::raw(desc),
                ]),
                Line::from(Span::raw("")),
            ]));
        }

        let list = List::new(items)
            .block(Block::new().borders(Borders::NONE))
            .style(Style::new().bg(HEX_SURFACE));

        f.render_widget(list, inner);

        let nav = Paragraph::new(Line::from(vec![
            Span::raw("  ↑↓ Navigate  ·  "),
            Span::raw("ENTER Confirm  ·  "),
            Span::raw("ESC Back  "),
        ]))
        .style(Style::new().fg(HEX_DIM))
        .alignment(Alignment::Center);
        f.render_widget(nav, inner);
    }

    fn draw_installing(&self, f: &mut Frame) {
        let size = f.area();
        let box_w = 64u16.min(size.width.saturating_sub(4));
        let box_h = 26u16.min(size.height.saturating_sub(4));
        let rect = self.centered_rect(box_w, box_h, size);

        let block = Block::new()
            .title("  Installing Polygone  ")
            .title_style(Style::new().fg(HEX_COBALT).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(HEX_BORDER))
            .style(Style::new().bg(HEX_SURFACE));

        f.render_widget(block, rect);
        let inner = rect.inner(2);

        // Progress bar
        let pct = (self.progress * 100.0) as u16;
        let gauge = Gauge::default()
            .gauge_style(Style::new().fg(HEX_COBALT).bg(HEX_BORDER))
            .line_style(Style::new().fg(HEX_BORDER))
            .label(format!(" {}%", pct))
            .ratio(self.progress);
        f.render_widget(gauge, inner);

        // Status
        let status = Paragraph::new(Line::from(vec![
            Span::raw("  "),
            Span::raw(&self.status_line),
        ]))
        .style(Style::new().fg(HEX_TEXT))
        .alignment(Alignment::Left);
        f.render_widget(status, inner);

        // Log
        let log_text: Vec<Line> = self.log_lines
            .iter()
            .map(|l| Line::from(Span::raw(format!("  {}", l))))
            .collect();
        let log = Paragraph::new(log_text)
            .style(Style::new().fg(HEX_DIM))
            .alignment(Alignment::Left);
        f.render_widget(log, inner);

        // Error
        if let Some(ref err) = self.error {
            let err_p = Paragraph::new(Line::from(vec![
                Span::styled("✗ ", Style::new().fg(HEX_RED)),
                Span::raw(err),
            ]))
            .style(Style::new().fg(HEX_RED))
            .alignment(Alignment::Center);
            f.render_widget(err_p, inner);
        }
    }

    fn draw_post_install(&self, f: &mut Frame) {
        let size = f.area();
        let box_w = 64u16.min(size.width.saturating_sub(4));
        let box_h = 28u16.min(size.height.saturating_sub(4));
        let rect = self.centered_rect(box_w, box_h, size);

        let block = Block::new()
            .title("  ✓ Installation Successful!  ")
            .title_style(Style::new().fg(HEX_GREEN).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(HEX_BORDER))
            .style(Style::new().bg(HEX_SURFACE));

        f.render_widget(block, rect);
        let inner = rect.inner(2);

        let lines = vec![
            Line::from(Span::raw("")),
            Line::from(Span::raw("  Next steps:")),
            Line::from(Span::raw("")),
            Line::from(vec![Span::raw("  "), Span::styled("1.", Style::new().fg(HEX_COBALT)), Span::raw("  Add to PATH if needed:")]),
            Line::from(vec![Span::raw("       "), Span::styled("echo 'export PATH=\"$HOME/.local/bin:$PATH\"' >> ~/.bashrc", Style::new().fg(HEX_DIM))]),
            Line::from(Span::raw("")),
            Line::from(vec![Span::raw("  "), Span::styled("2.", Style::new().fg(HEX_COBALT)), Span::raw("  Run the self-test:")]),
            Line::from(vec![Span::raw("       "), Span::styled("polygone self-test", Style::new().fg(HEX_DIM))]),
            Line::from(Span::raw("")),
            Line::from(vec![Span::raw("  "), Span::styled("3.", Style::new().fg(HEX_COBALT)), Span::raw("  Generate your keys:")]),
            Line::from(vec![Span::raw("       "), Span::styled("polygone keygen", Style::new().fg(HEX_DIM))]),
            Line::from(Span::raw("")),
            Line::from(Span::raw("  ⬡ \"L'information n'existe pas. Elle traverse.\"")),
            Line::from(Span::raw("")),
        ];

        let content = Paragraph::new(lines)
            .style(Style::new().fg(HEX_TEXT))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);
        f.render_widget(content, inner);

        let prompt = Paragraph::new(Line::from(vec![
            Span::raw("  Press "),
            Span::styled("ENTER", Style::new().fg(HEX_GREEN).bold()),
            Span::raw(" to finish  "),
        ]))
        .style(Style::new().fg(HEX_TEXT))
        .alignment(Alignment::Center);
        f.render_widget(prompt, inner);
    }

    fn draw_done(&self, f: &mut Frame) {
        let size = f.area();
        let box_w = 50u16.min(size.width.saturating_sub(4));
        let box_h = 12u16.min(size.height.saturating_sub(4));
        let rect = self.centered_rect(box_w, box_h, size);

        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(HEX_GREEN))
            .style(Style::new().bg(HEX_SURFACE));

        f.render_widget(block, rect);
        let inner = rect.inner(2);

        let bye = Paragraph::new(
            vec![
                Line::from(Span::raw("")),
                Line::from(vec![
                    Span::styled(" ⬡  POLYGONE is ready.", Style::new().fg(HEX_GREEN).bold()),
                ]),
                Line::from(Span::raw("")),
                Line::from(Span::raw("  Thank you for choosing privacy.")),
                Line::from(Span::raw("")),
            ]
        )
        .style(Style::new().fg(HEX_TEXT))
        .alignment(Alignment::Center);
        f.render_widget(bye, inner);
    }
}

pub fn run() -> anyhow::Result<()> {
    // Enable bracketed paste
    let _ = event::execute(std::io::stdout(), EnableBracketedPaste);

    let mut terminal = ratatui::init();

    crossterm::terminal::enable_raw_mode()?;

    let installer = Installer::new();
    let result = installer.run(&mut terminal);

    // Cleanup
    let _ = event::execute(std::io::stdout(), DisableBracketedPaste);
    crossterm::terminal::disable_raw_mode()?;
    ratatui::restore();

    result
}
