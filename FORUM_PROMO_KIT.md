# POLYGONE — Kit de Lancement Public

## ⬡ Résumé

**POLYGONE** est un réseau de confidentialité post-quantique éphémère construit en Rust pur. Le protocole transforme les messages en états computationnels distribués temporaires — une "vague" qui traverse un DHT global puis s'évapore.

- **8 dépôts** synchronisés sur GitHub
- **ML-KEM-1024** (FIPS 203) + **ML-DSA-87** (FIPS 204)
- **Shamir Secret Sharing** 4-of-7
- **30s TTL** pour tous les fragments
- `forbid(unsafe_code)` + ZeroizeOnDrop

---

## 🚀 Message Hacker News (Show HN)

**Titre suggéré :**
```
Show HN: Polygone – A post-quantum ephemeral network built in Rust
```

**Corps :**
```
Hi HN,

I'm Lévy, 14, and I've been working on Polygone for some time now.

The idea was simple: traditional encryption bets on time (hiding content). 
Polygone bets on existence (hiding communication). By sharding messages 
into computational states across a DHT using Shamir Secret Sharing and 
ML-KEM-1024, I wanted to create a 'vapor' network where data doesn't 
just travel—it drifts and vanishes.

**The ecosystem currently includes:**
- **Drive**: Sharded storage with streaming support.
- **Hide**: A SOCKS5 tunnel for post-quantum anonymity.
- **Petals**: Collaborative LLM inference.
- **Brain**: AI diagnostics of the swarm.

It's in alpha, built entirely in Rust (no unsafe code). I'm looking for 
feedback on the protocol architecture and the sharding logic.

Repo: https://github.com/lvs0/Polygone
Universal Installer: `curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone-CLI/main/install.sh | bash`

I'll be here to answer any technical questions!
```

---

## 🐧 Message Reddit r/rust

**Titre :**
```
I built a post-quantum ephemeral network where the communication itself is unobservable — looking for code review [Rust, ML-KEM-1024, Shamir]
```

**Corps :**
```
Hey everyone,

I've been obsessed with the idea of 'inobservable' communication. 
Standard VPNs and encrypted chats have a centralized metadata problem.

I built **Polygone**, a suite of 7 tools tailored for the post-quantum era. 
No servers, just an ephemeral wave across a DHT.

**Key Tech:**
- **Crates**: libp2p, ratatui, ML-KEM (FIPS 203), ML-DSA.
- **Concept**: Every packet is sharded into 7 fragments. You need 4 to reconstruct.
- **Aesthetic**: I spent a lot of time on the TUIs to make network monitoring 
  as immersive as possible.

Documentation & Installation: https://github.com/lvs0/Polygone-CLI

Would love to get some testers to join the swarm and see how the DHT holds up!

I'm 14, French. Happy to answer questions about the protocol.
```

---

## 🎯 Subreddits Ciblés (par priorité)

| Subreddit | Audience | Angle |
|---|---|---|
| r/rust | Développeurs Rust | Architecture + code review |
| r/privacy | Protection données | "La vague plutôt que le coffre" |
| r/cybersecurity | Sécurité réseau | Post-quantum ephemeral routing |
| r/netsec | Sécurité réseau | Protocol design review |
| r/opensource | Communauté FOSS | MIT, no VC, no ads |
| r/france | Français | "Souveraineté numérique, 14 ans" |

---

## 📝 Conseils de Timing

- **NE PAS** poster un vendredi (trafic Reddit faible le WE)
- **Mardi ou mercredi**, 14h-18h heure FR (matin US = pic de trafic)
- **Commencer par r/rust** (communauté la plus rigoureuse → si ça passe, le reste suit)

---

## 💬 Comment Répondre aux Commentaires

**Si on dit "ça existe déjà"**
→ "Montrez-moi. Je veux lire. Si ça existe, je veux contribuer à ce projet plutôt que dupliquer."

**Si un expert pose une vraie question technique**
→ Répondre honnêtement, même si la réponse est "je sais pas encore".

**Golden rule :** ne pas défendre l'ego. Défendre l'idée.

---

## 🔗 Liens Importants

- **Repo principal** : https://github.com/lvs0/Polygone
- **CLI Installer** : https://github.com/lvs0/Polygone-CLI
- **Drive** : https://github.com/lvs0/Polygone-Drive
- **Hide (VPN)** : https://github.com/lvs0/Polygone-Hide
- **Petals (LLM)** : https://github.com/lvs0/Polygone-Petals
- **Shell (TUI)** : https://github.com/lvs0/Polygone-Shell
- **Server** : https://github.com/lvs0/Polygone-Server
- **Brain (IA)** : https://github.com/lvs0/Polygone-Brain

---

## ✅ Checklist avant de Poster

- [x] Code compile sans erreurs
- [x] Tests passent (13/13)
- [x] CLI self-test passe
- [x] README est à jour
- [x] GitHub Pages activé sur tous les repos
- [ ] Lire les rules des subreddits avant de poster
- [ ] Préparer des screenshots de l'UI
- [ ] Configurer les notifications GitHub pour répondre vite

---

*Dernière mise à jour : 2026-04-12*
*L'information n'existe pas. Elle traverse. ⬡*
