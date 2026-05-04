use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "polygone")]
#[command(about = "Réseau P2P post‑quantique", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lance le nœud + dashboard Web UI (port 9050)
    Start,
    /// Exécute les tests intégrés locaux
    Test,
}
