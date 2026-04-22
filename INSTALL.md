# ⬡ POLYGONE — Installation Guide

> **Version 2.0** — Instructions détaillées pour toutes les plateformes

---

## 🎯 Choisissez Votre Plateforme

| OS | Méthode Recommandée | Temps |
|---|---|---|
| **Windows** | Installateur .exe | 2 min |
| **macOS** | Homebrew | 1 min |
| **Linux** | Script d'installation | 3 min |
| **Tous** | Compilation source | 10 min |

---

## 🪟 Windows

### Méthode 1 : Installateur (Recommandé)

1. **Télécharger** le fichier `polygone-installer.exe` depuis [les releases](https://github.com/lvs0/Polygone/releases/latest)

2. **Exécuter** l'installateur en double-cliquant dessus

3. **Suivre** les instructions :
   - Accepter la licence MIT
   - Choisir le dossier d'installation (par défaut : `C:\Program Files\Polygone`)
   - Ajouter au PATH (recommandé)

4. **Vérifier** l'installation :
   ```powershell
   polygone --version
   ```

### Méthode 2 : Winget

```powershell
winget install lvs0.Polygone
```

### Méthode 3 : Scoop

```powershell
scoop bucket add polygone https://github.com/lvs0/scoop-polygone.git
scoop install polygone
```

---

## 🍎 macOS

### Méthode 1 : Homebrew (Recommandé)

```bash
# Installer Homebrew si nécessaire
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Ajouter le tap et installer
brew tap lvs0/polygone
brew install polygone

# Vérifier
polygone --version
```

### Méthode 2 : MacPorts

```bash
sudo port install polygone
```

### Méthode 3 : Manuel

```bash
# Télécharger le binaire
curl -LO https://github.com/lvs0/Polygone/releases/latest/download/polygone-macos-arm64

# Rendre exécutable
chmod +x polygone-macos-arm64

# Déplacer dans le PATH
sudo mv polygone-macos-arm64 /usr/local/bin/polygone

# Vérifier
polygone --version
```

---

## 🐧 Linux

### Debian / Ubuntu / Mint

```bash
# Télécharger et exécuter le script d'installation
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

# OU télécharger le .deb
wget https://github.com/lvs0/Polygone/releases/latest/download/polygone-linux-x64.deb
sudo dpkg -i polygone-linux-x64.deb

# Vérifier
polygone --version
```

### Arch Linux / Manjaro

```bash
# Via AUR (avec yay)
yay -S polygone

# OU compilation AUR
git clone https://aur.archlinux.org/polygone.git
cd polygone
makepkg -si

# Vérifier
polygone --version
```

### Fedora / RHEL

```bash
# Via DNF (si disponible)
sudo dnf install polygone

# OU utiliser le script
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

# Vérifier
polygone --version
```

### NixOS

```nix
# Dans configuration.nix
environment.systemPackages = with pkgs; [ polygone ];
```

```bash
# Ou installation utilisateur
nix-env -iA nixpkgs.polygone
```

---

## 🛠️ Compilation depuis les Sources

### Prérequis Communs

| Outil | Version | Installation |
|---|---|---|
| Rust | nightly | `rustup default nightly` |
| Cargo | latest | Inclus avec Rust |
| Git | any | `apt install git` / `brew install git` |

### Étapes de Compilation

```bash
# 1. Cloner le dépôt
git clone https://github.com/lvs0/Polygone.git
cd Polygone

# 2. Compiler en mode release
cargo build --release

# 3. (Optionnel) Avec interface graphique
cargo build --release --features gui

# 4. Installer
cargo install --path .

# 5. Vérifier
polygone --version
polygone-gui  # Si compilé avec GUI
```

### Flags de Compilation

| Flag | Description |
|---|---|
| `--features cli` | CLI uniquement (défaut) |
| `--features gui` | Interface graphique |
| `--features full` | CLI + GUI + libp2p |
| `--release` | Build optimisé (recommandé) |

---

## ✅ Vérification de l'Installation

Après installation, exécutez ces commandes :

```bash
# 1. Vérifier la version
polygone --version
# Doit afficher : polygone 2.0.0

# 2. Lancer les tests internes
polygone self-test
# Doit afficher : ✔ All X tests passed

# 3. Générer des clés de test
polygone keygen
# Doit créer ~/.polygone/keys/

# 4. Voir le statut
polygone status
# Doit montrer : Keypair ✔ present
```

---

## 🔧 Dépannage

### Problème : "Command not found"

**Solution :** Le binaire n'est pas dans votre PATH.

```bash
# Trouver où polygone est installé
which polygone  # Linux/macOS
where polygone  # Windows

# Ajouter au PATH manuellement
export PATH="$HOME/.local/bin:$PATH"  # Linux/macOS
setx PATH "%PATH%;C:\Program Files\Polygone"  # Windows
```

### Problème : "Permission denied"

**Solution :** Rendre le binaire exécutable.

```bash
chmod +x $(which polygone)
```

### Problème : Library missing (Linux)

**Solution :** Installer les dépendances système.

```bash
# Debian/Ubuntu
sudo apt install libssl-dev pkg-config cmake

# Fedora
sudo dnf install openssl-devel pkg-config cmake

# Arch
sudo pacman -S openssl pkgconf cmake
```

### Problème : Compilation lente

**Solution :** Utiliser plus de cœurs CPU.

```bash
# Éditer ~/.cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Ou simplement
cargo build --release --jobs 4
```

---

## 📞 Support

Si vous rencontrez des problèmes :

1. Consultez la [FAQ](README.md#faq)
2. Vérifiez les [issues existantes](https://github.com/lvs0/Polygone/issues)
3. Créez une nouvelle issue avec :
   - Votre OS et version
   - La méthode d'installation utilisée
   - Les messages d'erreur complets

---

<div align="center">

**⬡ POLYGONE v2.0**

*L'information n'existe pas. Elle traverse.*

[Retour au README](README.md) • [Guide Utilisateur](GUIDE_UTILISATEUR.md)

</div>
