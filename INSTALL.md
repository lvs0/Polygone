# ⬡ POLYGONE — Guide d'Installation Multiplateforme

## 📋 Table des matières

1. [Installation Rapide](#installation-rapide)
2. [Linux (Fedora, Ubuntu, Debian, Arch)](#linux)
3. [macOS](#macos)
4. [Windows](#windows)
5. [Compilation depuis les sources](#compilation-depuis-les-sources)
6. [Vérification](#vérification)

---

## 🚀 Installation Rapide

### Script automatique (Recommandé)

```bash
# Télécharge et exécute le script d'installation
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
```

Le script détecte automatiquement votre système et installe les dépendances nécessaires.

---

## 🐧 Linux

### Fedora

```bash
# 1. Installer les dépendances
sudo dnf install -y rust cargo openssl-devel pkg-config make git curl

# 2. Compiler et installer
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release

# 3. Copier le binaire
sudo cp target/release/polygone /usr/local/bin/
```

### Ubuntu/Debian

```bash
# 1. Installer les dépendances
sudo apt-get update
sudo apt-get install -y rustc cargo libssl-dev pkg-config make git curl

# 2. Compiler et installer
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release

# 3. Copier le binaire
sudo cp target/release/polygone /usr/local/bin/
```

### Arch Linux

```bash
# 1. Installer les dépendances
sudo pacman -Sy rust cargo openssl pkg-config make git curl

# 2. Compiler et installer
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release

# 3. Copier le binaire
sudo cp target/release/polygone /usr/local/bin/
```

---

## 🍎 macOS

### Avec Homebrew (Recommandé)

```bash
# 1. Installer Homebrew si nécessaire
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. Installer les dépendances
brew install rust openssl pkg-config make git curl

# 3. Compiler et installer
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release

# 4. Copier le binaire
cp target/release/polygone /usr/local/bin/
# Ou pour Apple Silicon:
cp target/release/polygone /opt/homebrew/bin/
```

---

## 🪟 Windows

### Option 1: WSL (Windows Subsystem for Linux)

1. Installez WSL2 et une distribution Linux (Ubuntu recommandé)
2. Suivez les instructions Linux ci-dessus dans WSL

### Option 2: Natif avec PowerShell

```powershell
# 1. Installer Rust (depuis PowerShell admin)
winget install Rustlang.Rust.GNU
# OU téléchargez depuis https://rustup.rs/

# 2. Cloner le dépôt
git clone https://github.com/lvs0/Polygone.git
cd Polygone

# 3. Compiler
cargo build --release

# 4. Le binaire est dans target\release\polygone.exe
```

### Option 3: Télécharger le binaire pré-compilé

Rendez-vous sur la page [Releases](https://github.com/lvs0/Polygone/releases) et téléchargez:
- `polygone-windows-x86_64.zip` pour Windows 64-bit

Dézippez et placez `polygone.exe` dans un dossier de votre PATH.

---

## 🛠️ Compilation depuis les sources

### Prérequis communs

- **Rust** >= 1.75 ([installer](https://rustup.rs/))
- **Git**
- **OpenSSL** (libssl-dev ou openssl-devel)
- **pkg-config**
- **make**

### Étapes

```bash
# 1. Cloner le dépôt
git clone https://github.com/lvs0/Polygone.git
cd Polygone

# 2. Compiler en mode release (optimisé)
cargo build --release

# 3. (Optionnel) Lancer les tests
cargo test

# 4. (Optionnel) Installer globalement
sudo cp target/release/polygone /usr/local/bin/
```

---

## ✅ Vérification

Après installation, vérifiez que POLYGONE fonctionne:

```bash
# Afficher la version
polygone --version

# Lancer l'interface TUI
polygone tui

# Générer vos clés
polygone keygen

# Lancer les tests intégrés
polygone self-test
```

---

## 🎯 Utilisation de base

```bash
# Interface graphique terminal
polygone tui

# Générer une paire de clés post-quantique
polygone keygen

# Envoyer un message chiffré
polygone send --peer-pk <clé_publique> --message "Bonjour"

# Recevoir un message
polygone receive --sk ~/.polygone/keys/kem.sk --ciphertext <hex>

# Démarrer un nœud relayeur
polygone node start

# Voir l'état du réseau
polygone status
```

---

## 📞 Support

- **Documentation complète**: [docs/](docs/)
- **Issues GitHub**: https://github.com/lvs0/Polygone/issues
- **Discussions**: https://github.com/lvs0/Polygone/discussions

---

> ⬡ *"L'information n'existe pas. Elle traverse."*
