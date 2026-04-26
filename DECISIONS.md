# POLYGONE — Décisions architecturales

> Ce document répond aux questions qui n'ont pas de réponse évidente.
> Il explique les choix faits, pourquoi, et leurs limites honnêtes.
> Il sert de base pour le PIA CNIL et le whitepaper.

---

## D1 — Le problème du bootstrap

**Question :** Comment Alice obtient-elle la clé publique de Bob sans créer un point d'observation central ?

C'est le problème fondamental non résolu de tous les systèmes de communication privée. Trois options ont été évaluées.

**Option A — Échange local (QR code / NFC / copie directe)**

Alice et Bob se retrouvent physiquement ou utilisent un canal existant de confiance pour échanger leurs clés publiques. C'est inattaquable par définition — aucun réseau n'observe l'échange. C'est la solution retenue pour v0.1, documentée dans les instructions de déploiement.

Limites : inutilisable pour des inconnus qui ne se rencontrent pas. Frein à l'adoption à grande échelle.

**Option B — Prekey bundles anonymes (style Signal)**

Un serveur de distribution stocke des prekeys signées. Il ne connaît pas l'identité réelle des utilisateurs — seulement leurs pseudonymes et leurs clés publiques. Les utilisateurs publient leurs prekeys via onion routing (Tor ou POLYGONE lui-même) pour que le serveur ne voie pas leur IP. Le serveur distribue des prekeys sans pouvoir relier un destinataire à un expéditeur.

C'est l'option visée pour v0.2. Elle introduit un serveur — mais ce serveur ne voit ni le graphe social ni les messages.

**Option C — Introduction via tiers de confiance**

Si Alice et Bob ont un contact mutuel C qui utilise POLYGONE, C peut faire l'introduction en utilisant le protocole lui-même. Solution récursive, résout le problème sans serveur, mais nécessite un réseau déjà dense.

**Décision v0.1 :** Option A. Documentée honnêtement comme limitation.
**Décision v0.2 :** Option B, avec le serveur de prekeys accessible uniquement via POLYGONE.

---

## D2 — Modèle d'adversaire

Ce que POLYGONE protège contre, et ce qu'il ne protège pas contre.

**Adversaire supposé :**

- Peut observer tout le trafic réseau entre tous les nœuds (surveillance passive globale)
- Peut enregistrer tout le trafic pour analyse différée (harvest-now-decrypt-later)
- A accès à un ordinateur quantique suffisamment puissant pour casser RSA/ECDH
- Ne contrôle pas la majorité des nœuds du réseau

**Ce que POLYGONE protège :**

- Contenu des messages — via AES-256-GCM avec clé éphémère post-quantique
- Lien entre expéditeur et destinataire — via fragmentation multi-nœuds
- Sessions futures en cas de compromission de clé longue-durée — via forward secrecy ML-KEM
- Détermination de la longueur du message — via padding à 512 bytes (v0.1)

**Ce que POLYGONE ne protège PAS (v0.1) :**

- Métadonnées temporelles : un observateur voit des connexions apparaître. Sans cover traffic, le burst de 7 connexions simultanées est identifiable comme potentiellement POLYGONE. Couvert en v0.3.
- Attaques Sybil à grande échelle (voir D3).
- Compromission de la machine de l'utilisateur (out of scope — POLYGONE protège le transit, pas les endpoints).
- Timing attacks sur les nœuds relay (résistance partielle via jitter en v0.2).

---

## D3 — Résistance Sybil

**Question :** Si un attaquant contrôle 4 nœuds sur 7 dans une session, il peut reconstruire le message. Comment empêcher ça ?

C'est le problème le plus dur du projet. Voici l'analyse honnête.

**La surface d'attaque réelle :**

La topologie d'une session est dérivée du secret partagé — les nœuds sont désignés, pas choisis aléatoirement parmi ceux disponibles. Un attaquant ne peut pas cibler une session spécifique sans connaître le secret partagé, qu'il n'a pas. Il peut seulement opérer en masse : si il contrôle une fraction f des nœuds du réseau total, la probabilité qu'il contrôle k nœuds dans une session donnée suit une distribution hypergéométrique. Pour f = 40% et k ≥ 4 (threshold) sur n = 7, cette probabilité est environ 29%.

**Mécanismes envisagés :**

*Proof of work léger pour les nœuds relay.* Coût à l'entrée qui rend l'opération de milliers de nœuds Sybil coûteuse. Problème : discrimine les petites machines (Raspberry Pi, vieux PC) qu'on veut précisément encourager à rejoindre.

*Réputation basée sur la disponibilité.* Un nœud qui a une longue histoire de sessions réussies reçoit un score plus élevé. L'attaquant doit maintenir ses nœuds actifs longtemps avant de pouvoir les utiliser. Problème : un attaquant patient peut attendre.

*Augmenter le threshold.* Passer de 4-of-7 à 5-of-9 ou 6-of-11 réduit la probabilité d'attaque Sybil réussie. Coût : plus de nœuds nécessaires par session, latence accrue.

*Web of trust.* Les nœuds sont endorsés par d'autres nœuds de confiance. Crée un graphe de confiance. Problème : complexité, et recrée une forme de centralisation.

**Décision v0.1 :** Threshold 4-of-7 documenté avec ses limites. L'adversaire Sybil à grande échelle est **hors du modèle d'adversaire v0.1** — c'est explicite dans la documentation. Augmenter le threshold à 5-of-9 est le premier levier si le réseau grandit.

**Décision v0.2 :** Système de réputation basé sur la disponibilité mesurée, sans PoW.

---

## D4 — Résistance à l'analyse de trafic

**Ce qui est en place (v0.1) :**
- Padding à 512 bytes — tous les messages ont la même taille de fragment
- Jitter sur le TTL des nœuds — les sessions ne durent pas exactement le même temps
- SessionId dérivé du secret partagé — les nœuds relay ne savent pas qui communique

**Ce qui manque (prévu v0.3) :**
- Cover traffic : le réseau génère du trafic synthétique en permanence. Un observateur passif ne peut pas distinguer un vrai échange du bruit. C'est la condition nécessaire pour que la claim "la communication est inobservable" soit vraie dans le modèle d'adversaire passif global.
- Timing jitter sur la livraison des fragments : aujourd'hui les 7 fragments partent dans une fenêtre très courte. Il faut ajouter du délai aléatoire par fragment (0-200ms) pour briser la corrélation temporelle.

**Sur la claim "inobservable" :**

En v0.1, cette claim est vraie dans le modèle théorique. Elle n'est pas encore vraie contre un adversaire qui monitore au niveau des connexions TCP. Ce n'est pas une falsification — c'est une propriété qui s'implémente en couches. Le README le dit. Ce document le dit.

---

## D5 — Conformité RGPD et démarche CNIL

**Données traitées par un nœud relay POLYGONE :**

- Adresses IP des pairs qui se connectent : données personnelles au sens RGPD
- Fragments chiffrés : non-intelligibles, non-reliables à une personne sans le secret partagé

**Qualification juridique des opérateurs de nœuds :**

Un nœud relay fait transiter des fragments chiffrés. Il est probablement qualifiable de **sous-traitant** au sens de l'Article 28 RGPD. Les conditions requises : ne traite les données que selon les instructions du responsable de traitement, ne peut pas accéder au contenu, n'a aucun intérêt propre dans les données.

Ces trois conditions sont satisfaites architecturalement par POLYGONE : les nœuds ne peuvent techniquement pas lire les fragments, ne les stockent pas, et n'ont aucun moyen de relier un fragment à une identité.

**Pour la CNIL — ce qui est en place :**

- Privacy by Design (Article 25 RGPD) : la confidentialité est une propriété architecturale, pas une fonctionnalité ajoutée après
- Minimisation des données (Article 5(1)(c)) : aucune donnée sur les communicants n'est stockée durablement
- Chiffrement (Article 32) : AES-256-GCM + ML-KEM-1024

**À rédiger avant la démarche CNIL :**

- PIA (Privacy Impact Assessment) complet
- CGU pour les opérateurs de nœuds
- Documentation technique accessible aux non-techniciens

**Procédure CNIL recommandée :**

1. Rédiger le PIA selon la méthodologie CNIL (guide disponible sur cnil.fr)
2. Demander une consultation informelle via le formulaire "Innovation et numérique" de la CNIL — ils répondent aux projets innovants
3. Objectif : label "Privacy by Design" (officiel depuis 2019)

---

## D6 — Opération des nœuds à ressources limitées

**Question :** Comment faire fonctionner un nœud sur 512 MB de RAM ou moins ?

L'objectif est que n'importe qui puisse contribuer un nœud — petit VPS, vieux PC, Raspberry Pi.

**Contraintes mémoire d'un nœud relay v0.1 :**

- Binary Rust stripped : ~8-15 MB
- Fragments en transit : 7 fragments × taille max padded (PAD_BLOCK × multiplicateur Shamir) ≈ quelques MB par session simultanée
- libp2p swarm + DHT : ~20-40 MB selon la taille de la routing table

Un nœud qui gère 10 sessions simultanées devrait tenir dans 128 MB. 512 MB est confortable pour une centaine de sessions.

**Configuration recommandée pour les petites machines :**

```toml
# polygone.toml
[node]
max_concurrent_sessions = 20   # ~50 MB pic
session_timeout_secs    = 30   # agressive cleanup
dht_bucket_size         = 16   # routing table plus petite
```

Ces paramètres seront exposés dans le CLI en v0.2.

---

## D7 — Versioning du protocole

**Implémenté en v0.1 :** chaque paquet porte `PROTOCOL_VERSION = 1` dans son header.

Un nœud v0.1 qui reçoit un paquet version 2 renvoie une erreur explicite au lieu de silencieusement mal-interpréter les bytes.

**Règle de compatibilité :**

- Changement de format binaire → bump de version obligatoire
- Ajout de champs optionnels → peut rester v1 si les anciens clients ignorent les champs inconnus
- Changement du protocole de session → bump de version obligatoire

La migration entre versions se fait via négociation au moment du handshake : Alice envoie sa version, Bob répond avec la version commune la plus haute qu'il supporte.

---

## D8 — Ce qui reste comme décision ouverte

| Décision | Statut | Impact |
|---|---|---|
| Bootstrap mécanisme v0.2 | À trancher : prekey bundle vs autre | Critique pour l'adoption |
| Sybil : PoW vs réputation | À trancher avant réseau réel | Critique pour la sécurité |
| Cover traffic : volume et pattern | À modéliser | Critique pour la claim "inobservable" |
| Audit externe Shamir | À planifier | Requis avant prétention de sécurité |
| CGU opérateurs nœuds | À rédiger | Requis avant démarche CNIL |

---

*Ce document est vivant. Chaque décision prise sera datée et motivée.*
*Dernière mise à jour : 2026 — v0.1*
