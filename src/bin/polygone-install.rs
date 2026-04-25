//! POLYGONE TUI Installer
//! Full-screen ratatui installation experience.

use std::path::PathBuf;
use std::process::Command;
// use std::time::Duration; // unused for now

use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Alignment, Margin, Rect};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    DefaultTerminal, Frame,
};

const VERSION: &str = "1.0.0";
const C_BG: Color = Color::Rgb(10, 10, 15);
const C_SURFACE: Color = Color::Rgb(17, 17, 24);
const C_BORDER: Color = Color::Rgb(30, 30, 46);
const C_COBALT: Color = Color::Rgb(26, 107, 255);
const C_GREEN: Color = Color::Rgb(40, 200, 64);
const C_RED: Color = Color::Rgb(255, 59, 48);
const C_TEXT: Color = Color::Rgb(200, 200, 232);
const C_DIM: Color = Color::Rgb(74, 74, 106);

enum Step {
    Welcome,
    Install,
    Done,
}

struct App {
    step: Step,
    progress: f32,
    status: String,
    log: Vec<String>,
    error: Option<String>,
    done: bool,
}

impl App {
    fn new() -> Self {
        Self {
            step: Step::Welcome,
            progress: 0.0,
            status: String::new(),
            log: Vec::new(),
            error: None,
            done: false,
        }
    }

    fn log(&mut self, msg: &str) {
        self.log.push(msg.to_string());
        if self.log.len() > 8 {
            self.log.remove(0);
        }
    }

    fn install(&mut self) {
        self.step = Step::Install;
        self.progress = 0.1;
        self.status = "Starting installation...".into();
        self.log("Preparing installation...");

        let install_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".local/bin");
        std::fs::create_dir_all(&install_dir).ok();
        self.progress = 0.2;

        // Try download
        let url = format!(
            "https://github.com/lvs0/Polygone/releases/download/v{}/polygone",
            VERSION
        );
        self.status = "Downloading binary...".into();
        self.log(&format!("Downloading: {}", &url[..40.min(url.len())]));

        let dl = Command::new("curl")
            .args(["-fsSL", "-o", "/tmp/polygone"])
            .arg(&url)
            .output();

        match dl {
            Ok(out) if out.status.success() => {
                self.progress = 0.6;
                self.log("Download complete");
                self.status = "Installing to ~/.local/bin...".into();

                let dest = install_dir.join("polygone");
                if std::fs::copy("/tmp/polygone", &dest).is_ok() {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))
                            .ok();
                    }
                    self.progress = 1.0;
                    self.status = "Done!".into();
                    self.log(&format!("Installed: {:?}", dest));
                    self.step = Step::Done;
                } else {
                    self.error = Some("Failed to copy binary".into());
                }
            }
            _ => {
                self.log("Download failed — building from source");
                self.status = "Building from source...".into();
                self.progress = 0.3;
                self.build_from_source(&install_dir);
            }
        }
    }

    fn build_from_source(&mut self, install_dir: &PathBuf) {
        let build_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("polygone-src");

        if !build_dir.exists() {
            self.status = "Cloning repository...".into();
            self.log("Cloning github.com/lvs0/Polygone...");
            let _ = Command::new("git")
                .args(["clone", "https://github.com/lvs0/Polygone.git"])
                .arg(&build_dir)
                .output();
        }
        self.progress = 0.5;

        self.status = "Building (cargo build --release)...".into();
        self.log("Running: cargo build --release");

        let build = Command::new("cargo")
            .current_dir(&build_dir)
            .args(["build", "--release", "--bin", "polygone"])
            .output();

        match build {
            Ok(out) if out.status.success() => {
                self.progress = 0.85;
                self.log("Build successful");
                self.status = "Installing...".into();

                let src = build_dir.join("target/release/polygone");
                let dest = install_dir.join("polygone");

                if std::fs::copy(&src, &dest).is_ok() {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))
                            .ok();
                    }
                    self.progress = 1.0;
                    self.status = "Done!".into();
                    self.log(&format!("Installed: {:?}", dest));
                    self.step = Step::Done;
                } else {
                    self.error = Some("Copy failed".into());
                }
            }
            Ok(out) => {
                let err = String::from_utf8_lossy(&out.stderr);
                self.error = Some(format!(
                    "Build failed: {}",
                    err.lines().last().unwrap_or("?")
                ));
            }
            Err(e) => {
                self.error = Some(format!("Build error: {}", e));
            }
        }
    }

    fn draw(&self, f: &mut Frame) {
        let size = f.area();
        f.render_widget(Clear, size);

        let w = 56u16.min(size.width.saturating_sub(4));
        let h = match self.step {
            Step::Welcome => 18u16,
            Step::Install => 22u16,
            Step::Done => 14u16,
        }
        .min(size.height.saturating_sub(4));

        let x = (size.width.saturating_sub(w)) / 2;
        let y = (size.height.saturating_sub(h)) / 2;
        let rect = Rect::new(x, y, w, h);

        let block = Block::new()
            .title("  ⬡ POLYGONE Installer  ")
            .title_style(Style::new().fg(C_COBALT).bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(C_BORDER))
            .style(Style::new().bg(C_SURFACE));

        f.render_widget(block, rect);

        match &self.step {
            Step::Welcome => self.draw_welcome(f, rect),
            Step::Install => self.draw_install(f, rect),
            Step::Done => self.draw_done(f, rect),
        }
    }

    fn draw_welcome(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(2, 2));

        let lines = vec![
            Line::from(""),
            Line::from(Span::raw("       ███████╗██╗     ██╗ █████╗ ███████╗")),
            Line::from(Span::raw("       ██╔════╝██║     ██║██╔══██╗██╔════╝")),
            Line::from(Span::raw("       ███████╗██║     ██║███████║███████╗")),
            Line::from(Span::raw("       ╚════██║██║     ██║██╔══██║╚════██║")),
            Line::from(Span::raw("       ███████║███████╗██║██║  ██║███████║")),
            Line::from(Span::raw("       ╚══════╝╚══════╝╚═╝╚═╝  ╚═╝╚══════╝")),
            Line::from(""),
            Line::from(vec![
                Span::raw("  v"),
                Span::styled(VERSION, Style::new().fg(C_DIM)),
                Span::raw("  ·  Post-Quantum Privacy Network"),
            ]),
            Line::from(""),
            Line::from(vec![Span::raw(
                "  ML-KEM-1024  ·  AES-256-GCM  ·  Shamir 4-of-7",
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Press "),
                Span::styled("ENTER", Style::new().fg(C_GREEN).bold()),
                Span::raw(" to install  ·  "),
                Span::raw("q to quit"),
            ]),
        ];

        let p = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .style(Style::new().fg(C_TEXT));
        f.render_widget(p, inner);
    }

    fn draw_install(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(2, 2));

        // Progress bar
        let pct = (self.progress * 100.0) as u16;
        let bar_w = (inner.width as f32 * self.progress) as u16;

        let pad_len = (inner.width as usize).saturating_sub(8_usize.saturating_add(bar_w as usize));
        let bar_pad = " ".repeat(pad_len);
        let bar_line = Line::from(vec![
            Span::raw("  ["),
            Span::styled("=".repeat(bar_w as usize), Style::new().fg(C_COBALT)),
            Span::raw(bar_pad),
            Span::raw("] "),
            Span::raw(format!("{}%", pct)),
        ]);

        let status_line = Line::from(vec![Span::raw("  "), Span::raw(&self.status)]);

        let log_lines: Vec<Line> = self
            .log
            .iter()
            .map(|l| Line::from(Span::styled(format!("  {}", l), Style::new().fg(C_DIM))))
            .collect();

        let error_line = self.error.as_ref().map(|e| {
            Line::from(vec![
                Span::styled("✗ ", Style::new().fg(C_RED)),
                Span::raw(e),
            ])
        });

        let all_lines = [
            vec![Line::from("")],
            vec![bar_line],
            vec![status_line],
            vec![Line::from("")],
            log_lines,
            self.error
                .as_ref()
                .map(|_| Line::from(""))
                .into_iter()
                .collect(),
            error_line.into_iter().collect(),
        ]
        .concat();

        let p = Paragraph::new(all_lines)
            .style(Style::new().fg(C_TEXT))
            .alignment(Alignment::Left);
        f.render_widget(p, inner);
    }

    fn draw_done(&self, f: &mut Frame, rect: Rect) {
        let inner = rect.inner(Margin::new(2, 2));

        let lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "  ✓  INSTALLED",
                Style::new().fg(C_GREEN).bold(),
            )]),
            Line::from(""),
            Line::from("  Run these commands:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("polygone self-test", Style::new().fg(C_COBALT)),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("polygone keygen", Style::new().fg(C_COBALT)),
            ]),
            Line::from(""),
            Line::from("  ⬡ L'information n'existe pas. Elle traverse."),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Press "),
                Span::styled("ENTER", Style::new().fg(C_GREEN).bold()),
                Span::raw(" to exit"),
            ]),
        ];

        let p = Paragraph::new(lines)
            .style(Style::new().fg(C_TEXT))
            .alignment(Alignment::Center);
        f.render_widget(p, inner);
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;

            if let Event::Key(key) = event::read()? {
                match self.step {
                    Step::Welcome => {
                        if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                            self.install();
                        } else if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                            break;
                        }
                    }
                    Step::Install => {
                        // Installation happens async-like in steps
                        // Just wait
                    }
                    Step::Done => {
                        if key.code == KeyCode::Enter
                            || key.code == KeyCode::Char('q')
                            || key.code == KeyCode::Esc
                        {
                            break;
                        }
                    }
                }
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
