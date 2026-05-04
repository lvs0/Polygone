//! Application Polygone (CLI + TUI).
//!
//! Fournit l’interface utilisateur et le point d’entrée principal.

mod cli;
mod tui;
mod runtime;
mod http_api;

use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        cli::Commands::Start => runtime::run_node(true)?,
        cli::Commands::Test => runtime::run_tests()?,
    }
    Ok(())
}
