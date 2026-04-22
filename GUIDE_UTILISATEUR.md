# ⬡ POLYGONE — Guide de l'Utilisateur

> **Version 2.0** — Messagerie privée post-quantique pour tous

---

## 📱 Table des Matières

1. [Installation](#installation)
2. [Premiers Pas](#premiers-pas)
3. [Envoyer un Message](#envoyer-un-message)
4. [Recevoir un Message](#recevoir-un-message)
5. [Interface Graphique](#interface-graphique)
6. [FAQ](#faq)

---

## 🚀 Installation

### Windows

1. Téléchargez `polygone-windows-x64.exe` depuis les [Releases](https://github.com/lvs0/Polygone/releases)
2. Exécutez l'installateur
3. Ouvrez PowerShell et tapez : `polygone --version`

### macOS

```bash
# Avec Homebrew (recommandé)
brew install polygone

# Ou manuellement
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
```

### Linux

```bash
# Debian/Ubuntu
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

# Arch Linux (AUR)
yay -S polygone

# Fedora
sudo dnf install polygone
```

---

## 🔑 Premiers Pas

### Étape 1 : Générer vos clés

Ouvrez un terminal et exécutez :

```bash
polygone keygen
```

Vous obtiendrez :
- Une **clé publique** à partager avec vos contacts
- Une **clé privée** à garder secrète (stockée dans `~/.polygone/keys/`)

### Étape 2 : Partager votre clé publique

Votre clé publique ressemble à ceci :
```
a1b2c3d4e5f6789... (1568 bytes en hexadécimal)
```

**Méthodes de partage :**
- Copiez-collez la dans un message
- Montrez le QR code (dans l'interface graphique)
- Envoyez par email

⚠️ **Important** : Ne partagez JAMAIS votre clé privée !

---

## 📤 Envoyer un Message

### En ligne de commande

```bash
polygone send --peer <CLÉ_PUBLIQUE_DESTINATAIRE> --message "Mon message secret"
```

### Avec l'interface graphique

1. Lancez `polygone-gui`
2. Allez dans l'onglet **"Envoyer"**
3. Collez la clé publique du destinataire
4. Tapez votre message
5. Cliquez sur **"Envoyer de manière sécurisée"**

### Ce qui se passe

```
┌─────────────────────────────────────────────┐
│ Votre message                               │
│   ↓                                         │
│ Chiffrement AES-256-GCM                     │
│   ↓                                         │
│ Fragmentation Shamir (4-of-7)               │
│   ↓                                         │
│ 7 fragments envoyés via le réseau           │
│   ↓                                         │
│ Le destinataire reçoit 4+ fragments         │
│   ↓                                         │
│ Reconstruction et déchiffrement             │
└─────────────────────────────────────────────┘
```

---

## 📥 Recevoir un Message

### Automatiquement (Interface Graphique)

L'application écoute en permanence les messages entrants. Quand un message arrive :

1. Une notification apparaît
2. Le message est automatiquement déchiffré
3. Il s'affiche dans l'onglet **"Messages"**
4. Il disparaît après lecture (éphémère)

### Manuellement (CLI)

```bash
# Démarrer l'écoute
polygone node start

# Les messages reçus s'affichent dans le terminal
```

---

## 💻 Interface Graphique

### Lancer l'interface

```bash
polygone-gui
```

### Onglets disponibles

| Onglet | Fonction |
|---|---|
| **Tableau de bord** | Vue d'ensemble, statut du réseau |
| **Clés** | Générer, exporter, importer des clés |
| **Envoyer** | Composer et envoyer un message |
| **Recevoir** | Voir les messages reçus |

### QR Codes

Dans l'onglet **Clés**, vous pouvez :
- Afficher votre clé publique en QR code
- Scanner la clé publique d'un contact (webcam)

---

## ❓ FAQ

### Q: Comment contacter quelqu'un ?

**R:** Vous avez besoin de sa **clé publique**. Demandez-la-lui par n'importe quel canal (email, SMS, en personne).

### Q: Mes messages sont-ils stockés quelque part ?

**R:** Non. Par défaut, les messages sont éphémères et détruits après lecture. Vous pouvez activer l'historique local (chiffré) dans les paramètres.

### Q: Puis-je utiliser POLYGONE sans Internet ?

**R:** Non. POLYGONE utilise un réseau pair-à-pair qui nécessite une connexion Internet.

### Q: Est-ce gratuit ?

**R:** Oui, totalement. Open-source sous licence MIT.

### Q: Comment ça marche avec un firewall ?

**R:** POLYGONE utilise les ports 4001 (TCP) et 4002 (UDP). Si vous êtes derrière un NAT, le réseau trouve automatiquement des relais.

### Q: Puis-je héberger mon propre serveur ?

**R:** Oui ! Voir le guide [`SCALING_GUIDE.md`](SCALING_GUIDE.md).

---

## 🔒 Bonnes Pratiques de Sécurité

1. **Sauvegardez vos clés** dans un gestionnaire de mots de passe
2. **Vérifiez les empreintes** de clés publiques en personne si possible
3. **Ne laissez pas** votre session ouverte sur un ordinateur partagé
4. **Utilisez l'interface graphique** pour plus de simplicité
5. **Gardez le logiciel à jour** : `polygone update`

---

## 🆘 Support

- 📖 [Documentation complète](docs/)
- 💬 [Discord communautaire](#)
- 🐛 [Signaler un bug](https://github.com/lvs0/Polygone/issues)
- ✉️ Contact : polygone@proton.me

---

<div align="center">

**⬡ POLYGONE v2.0**

*L'information n'existe pas. Elle traverse.*

Projet français · l-vs · collectif Hope (*by Hope*)

</div>
