# Polygone — Plan d'Exécution v1.1.0

> Basé sur : `polygone_msh_specification.pdf` (spécification officielle)
> + `moi_entree.txt` (vision originelle de Lévy)
> + `polygone-vision.md` (synthèse existante)
> Mis à jour : 2026-06-14 — Phase 1 déclarée complète.

## Rappel de la Vision

Polygone n'est pas un outil de chiffrement. C'est un **écosystème de souveraineté numérique**.
- Zero-trust, zero-knowledge, post-quantique
- Chaque utilisateur est son propre nœud
- Protocole `msh` (3 lettres, comme http, ssh, tcp)
- Commande unique `polygone` → TUI arrow-key → tout fonctionne
- Pas de cloud centralisé. Jamais.

## État Actuel (2026-06-14)

| Module | État | Notes |
|--------|------|-------|
| Crypto (AES, Shamir, BLAKE3) | ✅ Solide | 49+ tests, 0 warning |
| KEM (ML-KEM-1024) | ✅ Opérationnel | pqcrypto-mlkem 0.1.1, 7 tests, roundtrip vérifié |
| Network (libp2p) | ⚠️ Stub | P2PNode existant, PolygoneBehaviour vide |
| TUI | ⚠️ Visuel uniquement | Arrow-key functional, pas encore de données live |
| Msg | ❌ N'existe pas | Module complet à créer |
| Hide | ❌ N'existe pas | Proxy SOCKS5 à créer |
| Drive | ❌ N'existe pas | Stockage distribué + UI web à créer |
| Mesh | ❌ N'existe pas | mDNS + Bluetooth à créer |
| Brain | ⚠️ Partiel | CLI brain fonctionnelle, pas encore de modèle IA |
| Tests unitaires | ✅ Passants | 69/69 workspace, 0 failed |
| Tests intégration | ✅ Passants | 8/8 (integration.rs: 7, transfer.rs: 1) |
| Build release | ✅ OK | 2m17s, opt-level z, LTO |

**Avancement v0.1.0 : ~25%**

## Plan en 5 Phases (selon spec)

### Phase 1 — Consolidation Core & Workspace ✅ TERMINÉE
1. ✅ Workspace Cargo multi-crate fonctionnel
2. ✅ Intégrer `pqcrypto-mlkem` (vraie impl, FIPS 203, NIST 2024)
3. ✅ Types centralisés dans `common` (Packet, SessionKey, NodeId)
4. ✅ Dépendances circulaires éliminées
5. ✅ Tests unitaires crypto validés (69/69)
6. ✅ Tests d'intégration fixés (8/8)

### Phase 2 — TUI Maître ← **PROCHAINE**
1. 🔲 Connecter le dashboard aux données réelles (node_id, uptime, session key)
2. 🔲 4 onglets : Accueil, Favoris, Services, Paramètres
3. 🔲 Favoris : liste services filtrés, toggle
4. 🔲 Services : pause (10min→4h), activate/désactiver modules
5. 🔲 Paramètres : config réseau, ports, MAJ auto
6. 🔲 Pas de polling CPU — refresh sur événement ou touche [R]

### Phase 3 — Services P2P + Drive
1. 🔲 Activation libp2p + DHT Kademlia + PolygoneBehaviour
2. 🔲 Module Msg : messagerie E2E éphémère
3. 🔲 Module Hide : proxy SOCKS5
4. 🔲 Module Drive : stockage distribué + UI web locale
5. 🔲 Liens éphémères (expiration 24h)
6. 🔲 Streaming multimédia

### Phase 4 — Mesh de Proximité
1. 🔲 Découverte mDNS via Wi-Fi
2. 🔲 Liaison Bluetooth
3. 🔲 Distribution de charge intelligente
4. 🔲 Fragmentation automatique des tâches lourdes

### Phase 5 — Brain (IA Locale)
1. 🔲 Intégration modèle quantifié local
2. 🔲 Fallback automatique (Petals → Ollama → Groq)
3. 🔲 Interconnexion avec Mesh pour calcul distribué

## Priorité Immédiate

**Phase 2** = rendre le produit montrant.
Un binaire `polygone` qui compile, lance une TUI, et montre l'état réel du système.
Pas de networking fantôme. Pas de stubs.

## Règles de Constance

1. LOCAL d'abord — pas de Docker/Render tant que le binaire local ne marche pas
2. Un module à la fois — pas 5 en parallèle
3. Constance > vitesse
4. Chaque phase = build + tests + commit avant de passer à la suivante
5. Lévy teste, pas moi qui décide si c'est bon