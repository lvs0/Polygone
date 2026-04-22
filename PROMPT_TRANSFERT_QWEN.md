# 📘 PROMPT DE TRANSFERT — ÉCOSYSTÈME POLYGONE v2.1.0

## 🎯 CONTEXTE DU PROJET

**POLYGONE** est une messagerie privée post-quantique grand public développée en Rust. Le projet vise à rendre la cryptographie post-quantique accessible à tous, avec une philosophie "l'information n'existe pas, elle traverse".

**État actuel** : Version 2.1.0 en cours de finalisation. La CLI principale est fonctionnelle, l'outil de configuration interactif (`polygone-config`) a été créé, et l'architecture est prête pour le déploiement multi-plateforme.

---

## 📁 STRUCTURE DU DÉPÔT PRINCIPAL (Polygone)

```
/workspace/
├── Cargo.toml                    # v2.1.0 avec features: cli, gui, p2p, full
├── src/
│   ├── main.rs                   # CLI principale (commands: keygen, send, receive, node, status, self-test, tui, config)
│   ├── lib.rs                    # Bibliothèque core
│   ├── bin/
│   │   ├── polygone-gui.rs       # Interface graphique (iced)
│   │   └── polygone-config.rs    # ✅ NOUVEAU : Outil de config interactif type OpenClaw
│   ├── crypto/                   # Cryptographie post-quantique
│   │   ├── kem.rs                # ML-KEM-1024 (FIPS 203)
│   │   ├── sign.rs               # Ed25519 / ML-DSA
│   │   ├── symmetric.rs          # AES-256-GCM
│   │   ├── shamir.rs             # Secret sharing 4-of-7
│   │   └── secure_types.rs       # Types zeroize
│   ├── network/                  # Réseau P2P
│   │   ├── p2p.rs                # libp2p + Kademlia DHT
│   │   ├── node.rs               # Noeud relais
│   │   └── topology.rs           # Topologie dérivée de clé
│   ├── protocol/                 # Protocole de session
│   ├── tui/                      # Terminal UI (ratatui)
│   ├── server/                   # Server infrastructure
│   └── drive/                    # Stockage distribué
├── docs/
│   ├── ARCHITECTURE.md
│   ├── PROTOCOL.md
│   └── SECURITY.md
├── .github/workflows/
│   ├── ci.yml                    # Tests CI
│   ├── deploy.yml                # Déploiement
│   └── release.yml               # Builds multi-OS
└── [Documentation FR]
    ├── README.md                 # ✅ Réécrit pour grand public
    ├── GUIDE_UTILISATEUR.md
    ├── INSTALL.md
    └── CHANGELOG.md
```

---

## 🔧 COMPOSANTS IMPLÉMENTÉS

### 1. CLI Principale (`src/main.rs`)
**Commandes disponibles :**
- `polygone keygen` — Génère paire de clés ML-KEM-1024 + Ed25519
- `polygone send --peer-pk <key> --message <msg>` — Envoie chiffré
- `polygone receive --sk <path> --ciphertext <hex>` — Réception
- `polygone node start|stop|info` — Gestion des noeuds relais
- `polygone status` — État du réseau
- `polygone self-test` — Tests cryptographiques
- `polygone tui` — Dashboard terminal interactif
- `polygone config` — ✅ NOUVEAU : Lance l'outil de configuration

**Qualité visée :** Expérience utilisateur type OpenCode/OpenClaw
- Messages clairs avec emojis
- Barres de progression (indicatif)
- Tables formatées (comfy-table)
- Prompts interactifs (dialoguer)

### 2. Outil de Configuration (`src/bin/polygone-config.rs`)
**Fonctionnalités :**
- ✅ Menu interactif avec navigation fléchée
- ✅ Gestion des clés (génération, backup, affichage)
- ✅ Configuration réseau (P2P, adresse d'écoute, relais)
- ✅ Paramètres de confidentialité (auto-delete, metadata)
- ✅ Préférences d'affichage (thème, verbosité, emojis)
- ✅ Options avancées (expérimental, debug, logs)
- ✅ Sauvegarde TOML dans `~/.polygone/config.toml`
- ✅ Progress bars stylisées
- ✅ Tables de présentation des clés

**Dépendances ajoutées :**
```toml
indicatif = "0.17"      # Progress bars
dialoguer = "0.11"      # Prompts interactifs
comfy-table = "7"       # Tables ASCII
toml = "0.8"            # Parsing config
hostname = "0.4"        # Info système
whoami = "1"            # User info
ratatui = "0.29"        # TUI framework
crossterm = "0.28"      # Terminal backend
```

### 3. TUI Dashboard (`src/tui/`)
**Vues implémentées :**
- Dashboard (status, crypto stack, node grid animé)
- Keygen (infos + preview clé publique)
- Send (workflow de fragmentation)
- Receive (instructions de reconstruction)
- Node (gestion des noeuds)
- SelfTest (résultats des tests)
- Help (aide contextuelle)

**Raccourcis globaux :**
- `1-6` : Navigation entre vues
- `q/Esc` : Quitter
- `Ctrl+C` : Force quit
- `?/h` : Aide

### 4. GUI Desktop (`src/bin/polygone-gui.rs`)
**Framework :** Iced 0.13
**Onglets :**
- Tableau de bord
- Gestion des clés
- Envoyer message
- Recevoir message
- QR Code pour échange de clés

---

## 🔐 STACK CRYPTOGRAPHIQUE

| Composant | Technologie | Statut | Notes |
|-----------|-------------|--------|-------|
| KEM | ML-KEM-1024 (FIPS 203) | ✅ Prod | Résistant quantiques |
| Signature | Ed25519 | ✅ Prod | ML-DSA ready via trait |
| Chiffrement | AES-256-GCM | ✅ Prod | Authenticated encryption |
| KDF | BLAKE3 | ✅ Prod | Domain-separated |
| Secret Sharing | Shamir 4-of-7 | ✅ Prod | Threshold cryptography |
| Zeroize | zeroize + derive | ✅ Prod | Nettoyage mémoire |

**Sécurité :**
- `#![forbid(unsafe_code)]` sur tous les bins
- Permissions 600 sur les clés secrètes
- Session keys auto-détruites après usage
- Pas de telemetry, pas de tracking

---

## 🌐 ARCHITECTURE RÉSEAU

### Modèle "Vague" (Wave Architecture)

```
┌─────────────────────────────────────────────────────────────┐
│  1. Alice & Bob échangent clé ML-KEM-1024 (hors-bande)     │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  2. 7 noeuds éphémères dérivés déterministement de la clé  │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  3. Message → AES-256-GCM → Shamir 4-of-7 → 7 fragments    │
│     Aucun noeud ne voit plus d'un fragment                  │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  4. Bob reconstruit avec ≥4 fragments → déchiffre          │
│     Tous les fragments et clés sont détruits                │
│     L'échange n'a jamais eu lieu                            │
└─────────────────────────────────────────────────────────────┘
```

### Implémentation P2P (libp2p)
**Fichier :** `src/network/p2p.rs`

**Composants :**
- ✅ Swarm TCP + WebSocket
- ✅ Noise (handshake crypté)
- ✅ Yamux (multiplexing)
- ✅ Kademlia DHT (découverte + stockage fragments)
- ✅ Identify (échange infos noeuds)
- ✅ DNS resolution

**Statut :** Intégré mais nécessite activation via feature `--features p2p`

**À améliorer :**
- [ ] Persistance identité noeud (`~/.polygone/identity.key`)
- [ ] Bootstrap nodes configurables
- [ ] NAT traversal (hole punching)
- [ ] QUIC transport (optionnel)

---

## 🚀 ECOSYSTÈME DES DÉPÔTS

L'écosystème Polygone comprend plusieurs dépôts GitHub commençant par `Polygone-`. Voici comment les traiter :

### Dépôts Connus/Probables
```
Polygone          (core)        — ✅ Ce dépôt principal
Polygone-GUI      (desktop)     — Interface graphique standalone
Polygone-Mobile   (mobile)      — Apps iOS/Android (Flutter/React Native?)
Polygone-Web      (web)         — WebAssembly + service worker
Polygone-Server   (infra)       — Infrastructure de bootstrap/nodes
Polygone-Docs     (docs)        — Documentation technique
Polygone-Crypto   (crypto)      — Bibliothèque crypto pure (FFI bindings)
Polygone-CLI      (cli)         — CLI alternative (shell scripts?)
Polygone-Tools    (tools)       — Utilitaires (key migration, backup)
```

### Stratégie d'Harmonisation

Pour CHAQUE dépôt `Polygone-*`, appliquer :

1. **Même charte graphique**
   - Logo ⬡ POLYGONE cohérent
   - Couleurs : sombre (#0a0a0f), cyan (#00ffff), vert (#00ff00)
   - Typographie : Inter ou JetBrains Mono

2. **Même structure de documentation**
   ```
   README.md          # Présentation + installation rapide
   CONTRIBUTING.md    # Guide contributeurs
   LICENSE            # MIT
   CHANGELOG.md       # Historique versions
   docs/
     ARCHITECTURE.md
     SECURITY.md
   ```

3. **Mêmes workflows CI/CD**
   - Tests automatiques sur PR
   - Builds multi-plateformes
   - Release GitHub automatique sur tag

4. **Mêmes standards de code**
   - Rust : `#![forbid(unsafe_code)]` si possible
   - Tests unitaires obligatoires
   - Benchmarks pour crypto
   - Zeroize pour données sensibles

---

## ✅ CORRECTIONS ET AMÉLIORATIONS À APPLIQUER

### 1. Sécurité Réseau (CRITIQUE)

**Problème identifié :** Le réseau doit cacher TOUTE interaction, même pour l'admin.

**Solutions à implémenter :**

```rust
// Dans src/network/p2p.rs

/// Mode furtif : aucun log de métadonnées
pub struct StealthConfig {
    pub disable_peer_logging: bool,      // Ne pas logger les PeerIds
    pub disable_traffic_analysis: bool,  // Padding constant des messages
    pub disable_admin_access: bool,      // Même l'admin ne voit rien
    pub ephemeral_identity: bool,        // Nouvelle identité par session
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            disable_peer_logging: true,
            disable_traffic_analysis: true,
            disable_admin_access: true,  // ← CRITIQUE
            ephemeral_identity: true,
        }
    }
}
```

**Actions :**
- [ ] Supprimer TOUS les `println!` qui révèlent des PeerIds ou adresses IP
- [ ] Ajouter du padding constant à TOUS les messages (même taille)
- [ ] Rendre impossible l'accès admin aux logs de communication
- [ ] Identités éphémères par session (pas de persistance)
- [ ] Chiffrement de bout en bout MÊME pour les logs internes

### 2. Qualité Interface CLI (Style OpenClaw)

**Objectif :** Avoir la même qualité UX que les outils modernes (opencode, openclaw)

**Améliorations à ajouter :**

```rust
// Dans src/main.rs — cmd_keygen

use indicatif::{ProgressBar, ProgressStyle};
use comfy_table::{Table, Cell, ContentArrangement};

async fn cmd_keygen(output: Option<PathBuf>, force: bool) -> anyhow::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
        .template("{spinner} {msg}")?);
    
    pb.set_message("Génération de ML-KEM-1024...");
    let kp = KeyPair::generate()?;
    
    pb.set_message("Sauvegarde sécurisée...");
    keys::write_keypair(&kp, &dir)?;
    
    pb.finish_and_clear();
    
    // Affichage tableau style OpenClaw
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.add_row(vec![
        Cell::new("Fichier").style(comfy_table::Color::Cyan),
        Cell::new("Taille").style(comfy_table::Color::Cyan),
        Cell::new("Usage").style(comfy_table::Color::Cyan),
    ]);
    // ... etc
}
```

**Checklist UX :**
- [ ] Progress bars sur TOUTES les opérations longues (>100ms)
- [ ] Tables pour TOUS les affichages structurés
- [ ] Codes couleur cohérents (cyan=info, vert=succès, rouge=erreur, jaune=warning)
- [ ] Messages d'erreur actionnables ("Hint: run `polygone keygen` first")
- [ ] Sortie silencieuse par défaut, verbose avec `-v/-vv`
- [ ] Support vrai dark/light theme (détection auto)

### 3. Simplification Humain-Machine

**Principes :**
1. **Zéro configuration manuelle** par défaut
2. **Auto-détection** intelligente (clés, réseau, ports)
3. **Messages en français** pour l'utilisateur final
4. **QR Codes** pour échanges mobiles
5. **Mode demo** pour tester sans setup

**Implémentations :**

```bash
# Premier lancement ultra-simple
polygone init
# → Génère clés, configure réseau, affiche QR code

# Envoi en 1 commande
echo "Salut" | polygone send @contact
# → Résout @contact depuis carnet d'adresses

# Réception auto
polygone listen
# → Écoute en fond, notifications desktop
```

**À créer :**
- [ ] `polygone init` — Setup wizard en 1 commande
- [ ] Carnet d'adresses (`~/.polygone/contacts.toml`)
- [ ] Alias pour contacts (`@alice` → clé publique)
- [ ] Notifications desktop (notify-rust)
- [ ] Mode écoute background (`polygone daemon`)

### 4. Tests et Validation

**Couverture actuelle :**
- ✅ Crypto self-test (`polygone self-test`)
- ❌ Tests d'intégration réseau
- ❌ Tests de charge (benchmarks)
- ❌ Fuzzing des parsers

**À ajouter :**

```rust
// tests/integration_test.rs

#[tokio::test]
async fn test_full_protocol_roundtrip() {
    // Alice et Bob sur loopback
    // Échange complet avec vrais sockets
    // Vérification zéro trace après dissolution
}

#[test]
fn test_zeroize_after_use() {
    // Vérifie que les clés sont écrasées mémoire
    // Utilisation de memory-safe assertions
}
```

**Benchmarks :**
```bash
cargo bench --bench crypto_bench
# Doit afficher :
# - ML-KEM encapsulation : X µs
# - AES-256-GCM encrypt : Y ns/octet
# - Shamir split/merge : Z µs
```

---

## 📝 INSTRUCTIONS POUR LA PROCHAINE SESSION QWEN

### Mission Globale

**Objectif :** Prendre CHAQUE dépôt `Polygone-*` et lui appliquer le même traitement qualitatif que le dépôt principal.

### Méthodologie Par Dépôt

#### Étape 1 : Audit Initial
```markdown
1. Cloner le dépôt
2. Lire README.md et comprendre le but
3. Identifier les écarts vs standards Polygone
4. Lister les dépendances obsolètes
5. Vérifier la présence des fichiers requis (LICENSE, CONTRIBUTING, etc.)
```

#### Étape 2 : Mise à Niveau
```markdown
1. Mettre à jour Cargo.toml (Rust 2021, deps récentes)
2. Ajouter `#![forbid(unsafe_code)]` si applicable
3. Implémenter zeroize pour données sensibles
4. Ajouter tests unitaires manquants
5. Configurer CI/CD (workflows GitHub)
```

#### Étape 3 : Amélioration UX
```markdown
1. Ajouter indicatif pour progress bars
2. Ajouter comfy-table pour affichages
3. Ajouter dialoguer pour prompts interactifs
4. Uniformiser messages (français, emojis)
5. Tester sur Linux/macOS/Windows
```

#### Étape 4 : Documentation
```markdown
1. Réécrire README.md (structure standard)
2. Ajouter GUIDE_UTILISATEUR.md si outil complexe
3. Créer INSTALL.md avec méthodes multiples
4. Mettre à jour CHANGELOG.md
5. Traduire commentaires code en français si pertinent
```

#### Étape 5 : Sécurité
```markdown
1. Audit des logs (supprimer métadonnées)
2. Vérifier permissions fichiers sensibles
3. Tester zeroize avec valgrind/memcheck
4. Review des erreurs (pas de leak d'infos)
5. Validation modèle de menace
```

### Template de Communication Inter-Dev

Quand tu travailles sur un dépôt et que tu dois passer la main :

```markdown
## 🔄 POINT D'ÉTAPE — [NOM_DU_DEPOT]

### ✅ Fait
- [Liste des tâches accomplies]

### 🚧 En Cours
- [Tâches en progression]

### ⚠️ Problèmes Rencontrés
- [Description + tentative de solution]

### 📋 Prochaines Étapes
1. [Action 1]
2. [Action 2]

### 📁 Fichiers Modifiés
- `chemin/vers/fichier.rs` — [Résumé changement]

### 🔗 Références
- Lien vers issue GitHub
- Lien vers doc technique
```

---

## 🎨 STANDARDS DE QUALITÉ

### Code Rust

```rust
// ✅ BIEN
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use zeroize::Zeroize;

pub struct SessionKey {
    bytes: [u8; 32],
}

impl Drop for SessionKey {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

// ❌ MAL
pub struct SessionKey {
    bytes: [u8; 32],  // Jamais écrasé !
}
```

### Messages Utilisateur

```rust
// ✅ BIEN
println!("  ✔ Clés générées avec succès !");
println!("     Répertoire : {}", dir.display());
println!("     Hint: Partagez kem.pk avec vos contacts.");

// ❌ MAL
println!("Keys generated in {:?}", dir);
```

### Gestion Erreurs

```rust
// ✅ BIEN
let kp = keys::read_keypair(&dir)
    .map_err(|e| anyhow::anyhow!(
        "{e}\n  Hint: Run `polygone keygen` to create a keypair."
    ))?;

// ❌ MAL
let kp = keys::read_keypair(&dir).unwrap();
```

---

## 📊 MÉTRIQUES DE SUCCÈS

Pour chaque dépôt traité, vérifier :

| Métrique | Cible | Mesure |
|----------|-------|--------|
| Coverage tests | >80% | `cargo tarpaulin` |
| Build time | <5 min | CI logs |
| Binary size | <10 MB | `ls -lh target/release/polygone*` |
| Startup time | <100 ms | `time polygone --version` |
| Memory safety | 0 unsafe | `grep -r "unsafe" src/` |
| Docs completeness | 100% | Checklist fichiers |

---

## 🆘 ESCALATION & SUPPORT

Si blocage technique :

1. **Consulter** `docs/ARCHITECTURE.md` et `docs/SECURITY.md`
2. **Vérifier** issues GitHub existantes
3. **Tester** avec `RUST_LOG=debug` pour logs détaillés
4. **Isoler** le problème dans un test minimal
5. **Documenter** toute découverte dans ce fichier

---

## 📞 CONTACT & PHILOSOPHIE

**Développeur principal :** Lévy <lvs0@proton.me>  
**Collectif :** Hope Collective  
**License :** MIT — No investors. No token. No telemetry.

**Philosophie :**
> "L'information n'existe pas. Elle traverse."

Chaque ligne de code doit refléter cette philosophie :
- **Éphémère** : Rien ne persiste inutilement
- **Invisible** : Aucune métadonnée exploitable
- **Accessible** : Compréhensible par tous
- **Robuste** : Résistant aux attaques quantiques

---

## ✨ CHECKLIST FINALE PAR DÉPÔT

Avant de considérer un dépôt comme "terminé" :

- [ ] Cargo.toml à jour (deps, features, profiles)
- [ ] `#![forbid(unsafe_code)]` si possible
- [ ] Tests unitaires (>80% coverage)
- [ ] Benchmarks pour crypto
- [ ] CI/CD configuré (tests + release)
- [ ] README.md complet (FR + EN)
- [ ] GUIDE_UTILISATEUR.md si pertinent
- [ ] INSTALL.md avec méthodes multiples
- [ ] CHANGELOG.md tenu à jour
- [ ] LICENSE MIT présente
- [ ] CONTRIBUTING.md clair
- [ ] Security policy (SECURITY.md)
- [ ] Code comments en français cohérent
- [ ] Messages utilisateur en français + emojis
- [ ] Progress bars sur opérations longues
- [ ] Tables pour données structurées
- [ ] Zeroize sur toutes données sensibles
- [ ] Permissions 600 sur fichiers secrets
- [ ] Logs audités (pas de metadata leak)
- [ ] Build testé sur Linux/macOS/Windows
- [ ] Binary size optimisé (LTO, strip)
- [ ] Startup time <100ms
- [ ] Documentation API (rustdoc)
- [ ] Exemples d'usage dans docs/
- [ ] QR codes pour configs mobiles (si pertinent)
- [ ] Mode demo/test fonctionnel
- [ ] Feature flags pour optionsnelles
- [ ] Backward compatibility assurée
- [ ] Migration guide si breaking changes

---

**FIN DU PROMPT DE TRANSFERT**

*Ce document doit être transmis intégralement à chaque nouveau développeur ou session IA travaillant sur l'écosystème Polygone.*

**Version :** 1.0  
**Date :** 2024  
**Dernière mise à jour :** Session v2.1.0 — Ajout polygone-config, harmonisation UX
