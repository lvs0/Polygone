//! Favorites persistence for Polygone services.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Default favorites file location.
pub fn favorites_path() -> PathBuf {
    let mut path = std::env::current_dir().expect("Failed to get current directory");
    path.push(".config");
    path.push("polygone");
    path.push("favorites.json");
    path
}

/// Load favorites from disk.
pub fn load_favorites() -> HashSet<String> {
    let path = favorites_path();
    if !path.exists() {
        return HashSet::new();
    }
    let content = fs::read_to_string(&path).expect("Failed to read favorites file");
    serde_json::from_str(&content).unwrap_or_else(|_| HashSet::new())
}

/// Save favorites to disk.
pub fn save_favorites(favorites: &HashSet<String>) {
    let path = favorites_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create config directory");
    }
    let content = serde_json::to_string(favorites).expect("Failed to serialize favorites");
    fs::write(&path, content).expect("Failed to write favorites file");
}
