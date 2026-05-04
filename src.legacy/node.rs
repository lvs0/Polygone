//! Node configuration and management.

use std::path::PathBuf;

/// Node configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeConfig {
    /// Whether auto-update is enabled.
    pub auto_update: bool,
    /// Whether performance mode is enabled.
    pub performance_mode: bool,
    /// Whether CPU boost is enabled.
    pub cpu_boost: bool,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            auto_update: true,
            performance_mode: false,
            cpu_boost: false,
        }
    }
}

impl NodeConfig {
    /// Load node configuration from disk, or return defaults.
    pub fn load() -> Self {
        let config_path = config_path();
        match std::fs::read_to_string(&config_path) {
            Ok(contents) => {
                if let Ok(cfg) = toml::from_str::<NodeConfig>(&contents) {
                    return cfg;
                }
            }
            Err(_) => {}
        }
        Self::default()
    }

    /// Save this configuration to disk.
    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = config_path();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(config_path, contents)?;
        Ok(())
    }
}

fn config_path() -> PathBuf {
    let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    p.push("polygone");
    p.push("node.toml");
    p
}
