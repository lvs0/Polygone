//! Key persistence: read/write keypairs from disk with secure permissions.
//!
//! Key files are stored as JSON in `~/.polygone/keys/` with chmod 600.
//! The directory is created if it doesn't exist, with chmod 700.

use std::fs;
use std::path::{Path, PathBuf};


use crate::crypto::{KeyPair, kem, sign};
use crate::{PolygoneError, Result};

// ── File layout ───────────────────────────────────────────────────────────────
//
//  ~/.polygone/
//  └── keys/
//      ├── kem.pk      — KEM public key (hex, share freely)
//      ├── sign.pk     — Sign public key (hex, share freely)
//      ├── kem.sk      — KEM secret key (hex, KEEP PRIVATE)
//      └── sign.sk     — Sign secret key (hex, KEEP PRIVATE)

const DIR_PERMISSIONS:  u32 = 0o700;
const FILE_PERMISSIONS: u32 = 0o600;

/// Resolve the default key directory: `~/.polygone/keys`.
pub fn default_key_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".polygone")
        .join("keys")
}

// ── Write ─────────────────────────────────────────────────────────────────────

/// Write a full keypair to `dir`, creating it with secure permissions.
pub fn write_keypair(kp: &KeyPair, dir: &Path) -> Result<()> {
    ensure_dir(dir)?;

    write_file(dir.join("kem.pk"),  &kp.kem_pk.to_hex())?;
    write_file(dir.join("sign.pk"), &kp.sign_pk.to_hex())?;
    write_file(dir.join("kem.sk"),  &kp.kem_sk.to_hex())?;
    write_file(dir.join("sign.sk"), &kp.sign_sk.to_hex())?;

    Ok(())
}

// ── Read ──────────────────────────────────────────────────────────────────────

/// Read a full keypair from `dir`.
pub fn read_keypair(dir: &Path) -> Result<KeyPair> {
    let kem_pk_hex  = read_file(dir.join("kem.pk"))?;
    let kem_sk_hex  = read_file(dir.join("kem.sk"))?;
    let sign_pk_hex = read_file(dir.join("sign.pk"))?;
    let sign_sk_hex = read_file(dir.join("sign.sk"))?;

    Ok(KeyPair {
        kem_pk:  kem::KemPublicKey::from_hex(&kem_pk_hex)?,
        kem_sk:  kem::KemSecretKey::from_hex(&kem_sk_hex)?,
        sign_pk: sign::SignPublicKey::from_hex(&sign_pk_hex)?,
        sign_sk: sign::SignSecretKey::from_hex(&sign_sk_hex)?,
    })
}

/// Read only the KEM public key from `dir` (e.g. to display or share).
pub fn read_kem_pk(dir: &Path) -> Result<kem::KemPublicKey> {
    let hex = read_file(dir.join("kem.pk"))?;
    kem::KemPublicKey::from_hex(&hex)
}

/// Check whether a keypair exists at `dir`.
pub fn keypair_exists(dir: &Path) -> bool {
    dir.join("kem.pk").exists()
        && dir.join("kem.sk").exists()
        && dir.join("sign.pk").exists()
        && dir.join("sign.sk").exists()
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn ensure_dir(dir: &Path) -> Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)
            .map_err(|e| PolygoneError::KeyFile(format!("cannot create {}: {e}", dir.display())))?;
    }
    set_dir_permissions(dir)?;
    Ok(())
}

fn write_file(path: PathBuf, content: &str) -> Result<()> {
    fs::write(&path, content)
        .map_err(|e| PolygoneError::KeyFile(format!("write {}: {e}", path.display())))?;
    set_file_permissions(&path)?;
    Ok(())
}

fn read_file(path: PathBuf) -> Result<String> {
    fs::read_to_string(&path)
        .map_err(|e| PolygoneError::KeyFile(
            format!("cannot read {}: {e} — run `polygone keygen` first", path.display())
        ))
        .map(|s| s.trim().to_string())
}

#[cfg(unix)]
fn set_dir_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(DIR_PERMISSIONS))
        .map_err(|e| PolygoneError::KeyFile(format!("chmod {}: {e}", path.display())))
}

#[cfg(not(unix))]
fn set_dir_permissions(_path: &Path) -> Result<()> { Ok(()) }

#[cfg(unix)]
fn set_file_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(FILE_PERMISSIONS))
        .map_err(|e| PolygoneError::KeyFile(format!("chmod {}: {e}", path.display())))
}

#[cfg(not(unix))]
fn set_file_permissions(_path: &Path) -> Result<()> { Ok(()) }
