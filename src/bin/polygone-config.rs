//! polygone-config — Interactive configuration tool for POLYGONE.
//!
//! A user-friendly CLI configuration interface inspired by OpenCode/OpenClaw.
//! Allows users to configure all aspects of POLYGONE through an interactive menu.
//!
//! Usage: polygone-config

#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::fs;
use std::io::{self, Write};

use clap::Parser;
use dialoguer::{Select, Input, Confirm, FuzzySelect};
use comfy_table::{Table, Cell, ContentArrangement};
use indicatif::{ProgressBar, ProgressStyle};
use tracing_subscriber::{fmt, EnvFilter};

use polygone::keys;

// ── CLI Definition ────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "polygone-config",
    version = env!("CARGO_PKG_VERSION"),
    author = "Lévy <lvs0@proton.me>",
    about = "Interactive configuration tool for POLYGONE",
    long_about = "⬡ POLYGONE CONFIG — Configure your post-quantum messaging experience

This tool provides an interactive interface to configure all aspects of POLYGONE:
  • Key management (generate, backup, restore)
  • Network settings (node configuration, P2P options)
  • Privacy preferences (auto-delete, metadata protection)
  • Display settings (theme, verbosity)
  • Advanced options (custom paths, experimental features)

Navigate with arrow keys, select with Enter."
)]
struct Cli {
    /// Skip welcome screen and go directly to main menu
    #[arg(long)]
    quick: bool,

    /// Configuration file path (default: ~/.polygone/config.toml)
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Verbosity: -v = info, -vv = debug
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

// ── Configuration Structure ───────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct Config {
    /// User profile
    profile: ProfileConfig,
    /// Network settings
    network: NetworkConfig,
    /// Privacy settings
    privacy: PrivacyConfig,
    /// Display settings
    display: DisplayConfig,
    /// Advanced options
    advanced: AdvancedConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ProfileConfig {
    /// Display name (optional, not transmitted)
    display_name: Option<String>,
    /// Path to key directory
    key_dir: PathBuf,
    /// Auto-backup keys on generation
    auto_backup: bool,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            display_name: None,
            key_dir: keys::default_key_dir(),
            auto_backup: true,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct NetworkConfig {
    /// Enable P2P networking
    p2p_enabled: bool,
    /// Listen address for node
    listen_address: String,
    /// Bootstrap nodes
    bootstrap_nodes: Vec<String>,
    /// Max connections
    max_connections: usize,
    /// Enable relay mode
    relay_mode: bool,
    /// RAM limit for relay (MB)
    relay_ram_mb: usize,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            p2p_enabled: false,
            listen_address: "0.0.0.0:4001".to_string(),
            bootstrap_nodes: vec![],
            max_connections: 50,
            relay_mode: false,
            relay_ram_mb: 256,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct PrivacyConfig {
    /// Auto-delete messages after reading
    auto_delete_messages: bool,
    /// Auto-delete session keys after use
    auto_delete_keys: bool,
    /// Hide metadata from logs
    hide_metadata: bool,
    /// Secure memory wiping
    secure_memory: bool,
    /// Disable telemetry (always disabled in POLYGONE)
    disable_telemetry: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            auto_delete_messages: true,
            auto_delete_keys: true,
            hide_metadata: true,
            secure_memory: true,
            disable_telemetry: true,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct DisplayConfig {
    /// Color theme (dark|light|minimal)
    theme: String,
    /// Enable emojis in output
    use_emojis: bool,
    /// Verbosity level (quiet|normal|verbose|debug)
    verbosity: String,
    /// Show progress bars
    show_progress: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            use_emojis: true,
            verbosity: "normal".to_string(),
            show_progress: true,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct AdvancedConfig {
    /// Enable experimental features
    experimental: bool,
    /// Custom data directory
    data_dir: Option<PathBuf>,
    /// Log file path
    log_file: Option<PathBuf>,
    /// Enable debug mode
    debug_mode: bool,
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            experimental: false,
            data_dir: None,
            log_file: None,
            debug_mode: false,
        }
    }
}

// ── Main Entry Point ──────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let filter = match cli.verbose {
        0 => "warn",
        1 => "info",
        _ => "debug",
    };
    fmt()
        .with_env_filter(EnvFilter::new(filter))
        .with_target(false)
        .init();

    // Determine config path
    let config_path = cli.config.unwrap_or_else(|| {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("polygone")
            .join("config.toml")
    });

    println!();
    print_banner();

    if !cli.quick {
        show_welcome();
    }

    // Load or create config
    let mut config = load_config(&config_path)?;

    // Main configuration loop
    loop {
        match show_main_menu() {
            MainMenuOption::KeyManagement => manage_keys(&mut config)?,
            MainMenuOption::NetworkSettings => configure_network(&mut config)?,
            MainMenuOption::PrivacySettings => configure_privacy(&mut config)?,
            MainMenuOption::DisplaySettings => configure_display(&mut config)?,
            MainMenuOption::AdvancedOptions => configure_advanced(&mut config)?,
            MainMenuOption::ViewCurrentConfig => view_config(&config)?,
            MainMenuOption::SaveAndExit => {
                save_config(&config, &config_path)?;
                println!();
                println!("  ✔ Configuration saved to {}", config_path.display());
                break;
            }
            MainMenuOption::ExitWithoutSaving => {
                println!();
                println!("  ℹ Exiting without saving changes.");
                break;
            }
        }
    }

    Ok(())
}

// ── UI Components ─────────────────────────────────────────────────────────────

fn print_banner() {
    println!(r#"
   ╭────────────────────────────────────────────────────────────╮
   │                                                            │
   │     ⬡ POLYGONE CONFIG v{}                              │
   │                                                            │
   │   Configure your post-quantum messaging experience         │
   │                                                            │
   ╰────────────────────────────────────────────────────────────╯
"#, env!("CARGO_PKG_VERSION"));
}

fn show_welcome() {
    println!("  Bienvenue dans l'outil de configuration POLYGONE !");
    println!();
    println!("  Cet outil interactif vous permet de configurer :");
    println!("    • Vos clés cryptographiques post-quantiques");
    println!("    • Les paramètres réseau et nœuds");
    println!("    • Vos préférences de confidentialité");
    println!("    • L'apparence et le comportement");
    println!();
    println!("  Navigation : ↑↓ pour naviguer, Entrée pour sélectionner");
    println!();
    println!("  Appuyez sur Entrée pour continuer...");
    let _ = io::stdin().read_line(&mut String::new());
    println!();
}

#[derive(Clone)]
enum MainMenuOption {
    KeyManagement,
    NetworkSettings,
    PrivacySettings,
    DisplaySettings,
    AdvancedOptions,
    ViewCurrentConfig,
    SaveAndExit,
    ExitWithoutSaving,
}

fn show_main_menu() -> MainMenuOption {
    let options = vec![
        "🔑 Gestion des clés",
        "🌐 Paramètres réseau",
        "🛡️ Confidentialité",
        "🎨 Affichage",
        "⚙️ Options avancées",
        "📄 Voir la configuration actuelle",
        "💾 Sauvegarder et quitter",
        "❌ Quitter sans sauvegarder",
    ];

    let selection = Select::new()
        .with_prompt("Que souhaitez-vous configurer ?")
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => MainMenuOption::KeyManagement,
        1 => MainMenuOption::NetworkSettings,
        2 => MainMenuOption::PrivacySettings,
        3 => MainMenuOption::DisplaySettings,
        4 => MainMenuOption::AdvancedOptions,
        5 => MainMenuOption::ViewCurrentConfig,
        6 => MainMenuOption::SaveAndExit,
        7 => MainMenuOption::ExitWithoutSaving,
        _ => unreachable!(),
    }
}

// ── Key Management ────────────────────────────────────────────────────────────

fn manage_keys(config: &mut Config) -> anyhow::Result<()> {
    loop {
        let options = vec![
            "✨ Générer une nouvelle paire de clés",
            "📂 Changer le répertoire des clés",
            "📋 Afficher les informations de clé",
            "💾 Sauvegarder les clés",
            "↩️ Retour au menu principal",
        ];

        let selection = Select::new()
            .with_prompt("Gestion des clés")
            .items(&options)
            .interact()?;

        match selection {
            0 => generate_keypair(config)?,
            1 => change_key_dir(config)?,
            2 => show_key_info(config)?,
            3 => backup_keys(config)?,
            4 => break,
            _ => {}
        }
    }
    Ok(())
}

fn generate_keypair(config: &Config) -> anyhow::Result<()> {
    println!();
    println!("  ⬡ Génération d'une paire de clés post-quantique");
    println!();

    let confirm = Confirm::new()
        .with_prompt(format!(
            "Les clés seront générées dans : {}",
            config.profile.key_dir.display()
        ))
        .default(true)
        .interact()?;

    if !confirm {
        return Ok(());
    }

    // Check if keys already exist
    if keys::keypair_exists(&config.profile.key_dir) {
        let overwrite = Confirm::new()
            .with_prompt("Des clés existent déjà. Voulez-vous les remplacer ?")
            .default(false)
            .interact()?;
        if !overwrite {
            println!("  ℹ Opération annulée.");
            return Ok(());
        }
    }

    // Show progress
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
        .template("{spinner} {msg}")?);
    pb.set_message("Génération de ML-KEM-1024...");

    let kp = polygone::KeyPair::generate()?;
    pb.set_message("Sauvegarde des clés...");
    keys::write_keypair(&kp, &config.profile.key_dir)?;

    pb.finish_and_clear();

    println!();
    println!("  ✔ Paire de clés générée avec succès !");
    println!();

    // Show key info
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.add_row(vec![
        Cell::new("Fichier").style(comfy_table::Color::Cyan),
        Cell::new("Taille").style(comfy_table::Color::Cyan),
        Cell::new("Statut").style(comfy_table::Color::Cyan),
    ]);
    table.add_row(vec![
        Cell::new("kem.pk"),
        Cell::new(format!("{} B", kp.kem_pk.as_bytes().len())),
        Cell::new("✓ Public - à partager"),
    ]);
    table.add_row(vec![
        Cell::new("sign.pk"),
        Cell::new(format!("{} B", kp.sign_pk.as_bytes().len())),
        Cell::new("✓ Public - à partager"),
    ]);
    table.add_row(vec![
        Cell::new("kem.sk"),
        Cell::new(format!("{} B", kp.kem_sk.to_hex().len() / 2)),
        Cell::new("🔒 Secret - garder privé"),
    ]);
    table.add_row(vec![
        Cell::new("sign.sk"),
        Cell::new(format!("{} B", kp.sign_sk.to_hex().len() / 2)),
        Cell::new("🔒 Secret - garder privé"),
    ]);

    println!("{table}");
    println!();
    println!("  Clé publique KEM (premiers caractères) :");
    println!("    {}…", &kp.kem_pk.to_hex()[..32]);
    println!();

    if config.profile.auto_backup {
        println!("  💡 Conseil : Sauvegardez vos clés secrètes sur un support externe.");
    }

    Ok(())
}

fn change_key_dir(config: &mut Config) -> anyhow::Result<()> {
    let new_path: String = Input::new()
        .with_prompt("Nouveau répertoire pour les clés")
        .default(config.profile.key_dir.to_string_lossy().to_string())
        .interact_text()?;

    config.profile.key_dir = PathBuf::from(new_path);
    println!("  ✔ Répertoire des clés mis à jour : {}", config.profile.key_dir.display());
    Ok(())
}

fn show_key_info(config: &Config) -> anyhow::Result<()> {
    if !keys::keypair_exists(&config.profile.key_dir) {
        println!("  ⚠ Aucune clé trouvée dans {}", config.profile.key_dir.display());
        println!("  → Utilisez 'Générer une nouvelle paire de clés' pour créer des clés.");
        return Ok(());
    }

    let kp = keys::read_keypair(&config.profile.key_dir)?;

    println!();
    println!("  📋 Informations sur les clés");
    println!();

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.add_row(vec![
        Cell::new("Algorithme").style(comfy_table::Color::Cyan),
        Cell::new("Valeur").style(comfy_table::Color::Cyan),
    ]);
    table.add_row(vec![
        Cell::new("KEM"),
        Cell::new("ML-KEM-1024 (FIPS 203)"),
    ]);
    table.add_row(vec![
        Cell::new("Signature"),
        Cell::new("Ed25519"),
    ]);
    table.add_row(vec![
        Cell::new("Répertoire"),
        Cell::new(config.profile.key_dir.display().to_string()),
    ]);

    println!("{table}");
    println!();
    println!("  Clé publique KEM : {}…", &kp.kem_pk.to_hex()[..48]);
    println!("  Clé publique Sign : {}…", &kp.sign_pk.to_hex()[..32]);

    Ok(())
}

fn backup_keys(config: &Config) -> anyhow::Result<()> {
    if !keys::keypair_exists(&config.profile.key_dir) {
        println!("  ⚠ Aucune clé à sauvegarder.");
        return Ok(());
    }

    let backup_dir: String = Input::new()
        .with_prompt("Répertoire de sauvegarde")
        .default("/tmp/polygone-backup".to_string())
        .interact_text()?;

    let backup_path = PathBuf::from(backup_dir);
    fs::create_dir_all(&backup_path)?;

    // Copy key files
    for file in ["kem.pk", "kem.sk", "sign.pk", "sign.sk"] {
        let src = config.profile.key_dir.join(file);
        let dst = backup_path.join(file);
        if src.exists() {
            fs::copy(&src, &dst)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if file.ends_with(".sk") {
                    fs::set_permissions(&dst, fs::Permissions::from_mode(0o600))?;
                }
            }
        }
    }

    println!("  ✔ Clés sauvegardées dans : {}", backup_path.display());
    println!("  🔒 Les fichiers secrets (.sk) ont des permissions restreintes (600).");

    Ok(())
}

// ── Network Configuration ─────────────────────────────────────────────────────

fn configure_network(config: &mut Config) -> anyhow::Result<()> {
    loop {
        let options = vec![
            format!("P2P: {}", if config.network.p2p_enabled { "✓ Activé" } else { "✗ Désactivé" }),
            format!("Adresse d'écoute: {}", config.network.listen_address),
            format!("Mode relais: {}", if config.network.relay_mode { "✓ Activé" } else { "✗ Désactivé" }),
            format!("RAM relais: {} MB", config.network.relay_ram_mb),
            format!("Connexions max: {}", config.network.max_connections),
            "↩️ Retour",
        ];

        let selection = Select::new()
            .with_prompt("Paramètres réseau")
            .items(&options)
            .interact()?;

        match selection {
            0 => {
                config.network.p2p_enabled = !config.network.p2p_enabled;
                println!("  ✔ P2P {}", if config.network.p2p_enabled { "activé" } else {"désactivé"});
            }
            1 => {
                let addr: String = Input::new()
                    .with_prompt("Adresse d'écoute")
                    .default(config.network.listen_address.clone())
                    .interact_text()?;
                config.network.listen_address = addr;
            }
            2 => {
                config.network.relay_mode = !config.network.relay_mode;
                println!("  ✔ Mode relais {}", if config.network.relay_mode { "activé" } else {"désactivé"});
            }
            3 => {
                let ram: usize = Input::new()
                    .with_prompt("RAM allouée au relais (MB)")
                    .default(config.network.relay_ram_mb)
                    .interact_text()?;
                config.network.relay_ram_mb = ram;
            }
            4 => {
                let max: usize = Input::new()
                    .with_prompt("Nombre maximum de connexions")
                    .default(config.network.max_connections)
                    .interact_text()?;
                config.network.max_connections = max;
            }
            5 => break,
            _ => {}
        }
    }
    Ok(())
}

// ── Privacy Configuration ─────────────────────────────────────────────────────

fn configure_privacy(config: &mut Config) -> anyhow::Result<()> {
    loop {
        let options = vec![
            format!("Suppression auto messages: {}", if config.privacy.auto_delete_messages { "✓" } else { "✗" }),
            format!("Suppression auto clés: {}", if config.privacy.auto_delete_keys { "✓" } else { "✗" }),
            format!("Masquer métadonnées: {}", if config.privacy.hide_metadata { "✓" } else { "✗" }),
            format!("Mémoire sécurisée: {}", if config.privacy.secure_memory { "✓" } else { "✗" }),
            "↩️ Retour",
        ];

        let selection = Select::new()
            .with_prompt("Confidentialité")
            .items(&options)
            .interact()?;

        match selection {
            0 => config.privacy.auto_delete_messages = !config.privacy.auto_delete_messages,
            1 => config.privacy.auto_delete_keys = !config.privacy.auto_delete_keys,
            2 => config.privacy.hide_metadata = !config.privacy.hide_metadata,
            3 => config.privacy.secure_memory = !config.privacy.secure_memory,
            4 => break,
            _ => {}
        }
    }
    Ok(())
}

// ── Display Configuration ─────────────────────────────────────────────────────

fn configure_display(config: &mut Config) -> anyhow::Result<()> {
    loop {
        let options = vec![
            format!("Thème: {}", config.display.theme),
            format!("Émojis: {}", if config.display.use_emojis { "✓" } else { "✗" }),
            format!("Verbosité: {}", config.display.verbosity),
            format!("Barres de progression: {}", if config.display.show_progress { "✓" } else { "✗" }),
            "↩️ Retour",
        ];

        let selection = Select::new()
            .with_prompt("Affichage")
            .items(&options)
            .interact()?;

        match selection {
            0 => {
                let themes = vec!["dark", "light", "minimal"];
                let idx = FuzzySelect::new()
                    .items(&themes)
                    .default(themes.iter().position(|&t| t == config.display.theme).unwrap_or(0))
                    .with_prompt("Choisir un thème")
                    .interact()?;
                config.display.theme = themes[idx].to_string();
            }
            1 => config.display.use_emojis = !config.display.use_emojis,
            2 => {
                let levels = vec!["quiet", "normal", "verbose", "debug"];
                let idx = FuzzySelect::new()
                    .items(&levels)
                    .default(levels.iter().position(|&l| l == config.display.verbosity).unwrap_or(1))
                    .with_prompt("Niveau de verbosité")
                    .interact()?;
                config.display.verbosity = levels[idx].to_string();
            }
            3 => config.display.show_progress = !config.display.show_progress,
            4 => break,
            _ => {}
        }
    }
    Ok(())
}

// ── Advanced Configuration ────────────────────────────────────────────────────

fn configure_advanced(config: &mut Config) -> anyhow::Result<()> {
    loop {
        let options = vec![
            format!("Fonctions expérimentales: {}", if config.advanced.experimental { "✓" } else { "✗" }),
            format!("Répertoire données: {:?}", config.advanced.data_dir),
            format!("Fichier de log: {:?}", config.advanced.log_file),
            format!("Mode debug: {}", if config.advanced.debug_mode { "✓" } else { "✗" }),
            "↩️ Retour",
        ];

        let selection = Select::new()
            .with_prompt("Options avancées")
            .items(&options)
            .interact()?;

        match selection {
            0 => config.advanced.experimental = !config.advanced.experimental,
            1 => {
                let path: String = Input::new()
                    .with_prompt("Répertoire de données (vide pour défaut)")
                    .allow_empty(true)
                    .interact_text()?;
                config.advanced.data_dir = if path.is_empty() { None } else { Some(PathBuf::from(path)) };
            }
            2 => {
                let path: String = Input::new()
                    .with_prompt("Fichier de log (vide pour désactiver)")
                    .allow_empty(true)
                    .interact_text()?;
                config.advanced.log_file = if path.is_empty() { None } else { Some(PathBuf::from(path)) };
            }
            3 => config.advanced.debug_mode = !config.advanced.debug_mode,
            4 => break,
            _ => {}
        }
    }
    Ok(())
}

// ── Config View ───────────────────────────────────────────────────────────────

fn view_config(config: &Config) -> anyhow::Result<()> {
    println!();
    println!("  📄 Configuration actuelle");
    println!();

    let json = serde_json::to_string_pretty(config)?;
    println!("{json}");
    println!();

    println!("  Appuyez sur Entrée pour continuer...");
    let _ = io::stdin().read_line(&mut String::new());

    Ok(())
}

// ── Config I/O ────────────────────────────────────────────────────────────────

fn load_config(path: &PathBuf) -> anyhow::Result<Config> {
    if path.exists() {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        println!("  ✓ Configuration chargée depuis {}", path.display());
        Ok(config)
    } else {
        println!("  ℹ Aucune configuration trouvée, création d'une configuration par défaut...");
        Ok(Config::default())
    }
}

fn save_config(config: &Config, path: &PathBuf) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}
