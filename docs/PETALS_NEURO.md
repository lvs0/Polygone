# 🌸 PETALS_NEURO — Protocole d'échange neuronal IA chiffré

> **Polygone-Petals** n'est pas un simple réseau de distribution de modèles.  
> C'est un **cerveau distribué**, où chaque nœud est un neurone artificiel,  
> chaque échange de hidden states est une synapse chiffrée,  
> et l'inférence collective émerge comme une conscience collective.

## Vue d'ensemble

PETALS_NEURO définit le protocole permettant à des nœuds Polygone de collaborer pour exécuter des modèles de langage de taille gigantesque (LLM) en toute sécurité post-quantique.

Au lieu de faire confiance à un serveur central, le modèle est **shardé** (coupé en couches) et distribué sur le réseau. Chaque nœud ne voit que sa propre couche, chiffre ses sorties avec ML-KEM-1024, et les transmet au suivant.

À la fin, l'utilisateur obtient le résultat — sans jamais révéler le modèle complet à aucun nœud individuel.

## Pourquoi "Neuronal" ?

- **Hidden states = potentiel d'action**  
  Chaque tenseur transmis représente l'activation d'une couche de neurones biologiques.
  
- **Chiffrement = myéline**  
  ML-KEM-1024 isole chaque échange, empêchant l'interception ou la corruption.

- **Topologie dynamique = plasticité synaptique**  
  Le réseau peut se reconfigurer : ajouter/retirer des nœuds, changer l'ordre des couches.

- **Émergence = conscience distribuée**  
  Aucune entité ne "sait" tout — mais le collectif produit une réponse cohérente.

## Architecture du protocole

```
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│   Client (User)     │    │   Orchestrator      │    │   Nœud Polygone     │
│   (Demande)         │    │   (Découpage)       │    │   (Couche i)        │
└─────────┬───────────┘    └─────────┬───────────┘    └─────────┬───────────┘
          │                          │                          │
          │ 1. Requête inférence     │                          │
          ├───────────────────────►  │                          │
          │                          │ 2. Orchestration         │
          │                          │    - Découpage modèle    │
          │                          │    - Attribution nœuds   │
          │                          │    - Clés de session     │
          │                          ◄───────────────────────  │
          │ 3. Clés publiques ML-KEM │                          │
          │    (pour chaque nœud)    │                          │
          │                          │ 4. Hidden state entrée   │
          │                          │    (chiffré ML-KEM)     │
          │                          ├───────────────────────►  │
          │                          │    Nœud 1 : Calcule     │
          │                          │    Couche i → i+1       │
          │                          │    Chiffre sortie       │
          │                          │    avec clé nœud 2      │
          │                          │ 5. Transmission         │
          │                          │    (p2p libp2p)         │
          │                          ├───────────────────────►  │
          │                          │    Nœud 2 : Calcule     │
          │                          │    Couche i+1 → i+2     │
          │                          │    ...                  │
          │                          │    ...                  │
          │                          │    ...                  │
          │                          ├───────────────────────►  │
          │                          │    Nœud N : Sortie      │
          │                          │    (logits)             │
          │                          │    Déchiffré par client │
          │ 6. Résultat final        ◄─────────────────────────┘
          │    (texte généré)        │
          └──────────────────────────┘
```

## Spécifications techniques

### 1. Sharding du modèle

Le modèle est partitionné le long de sa profondeur (couches).  
Chaque shard contient :
- Un sous-ensemble consécutif de couches (ex: couches 0-10, 11-20, ...)
- Les poids associés
- La configuration nécessaire (activation function, layer norm, etc.)

Le découpage est décidé par l'orchestrateur en fonction de :
- La capacité de chaque nœud (VRAM, calcul)
- La latence réseau estimée
- La redondance souhaitée (optionnel)

### 2. Chiffrement des hidden states

Entre deux nœuds, les hidden states sont chiffrés avec **ML-KEM-1024** :
- Le nœud récepteur publie sa clé publique ML-KEM-1024 dans le DHT (sous une clé dérivée de son node ID)
- Le nœud émetteur récupère cette clé, effectue l'encapsulation
- Le ciphertext + l'encapsulated key sont transmis
- Le nœud récepteur effectue la décapsulation pour récupérer le shared secret
- Le shared secret est utilisé pour dériver une clé AES-256-GCM (ou ChaCha20-Poly1305) pour le chiffrement réel du tenseur

Pourquoi ML-KEM + symétrique ?
- ML-KEM est coûteux pour de gros tenseurs → on l'utilise uniquement pour échanger une clé symétrique
- AES-GCM fournit authentification + chiffrement rapide

### 3. Transport

Utilise le réseau libp2p existant de Polygone :
- Chaque nœud Polygone-Petals s'annonce avec un protocole `/polygone-petals/1.0.0`
- La découverte se fait via le DHT Kademlia déjà présent
- Les connexions sont chiffrées au niveau transport (TLS 1.3 via noise ou libp2p-tls)
- Supplémentairement, le chiffrement applicatif (ML-KEM + AES) ajoute une couche de sécurité post-quantique

### 4. Gestion de la session

Une session d'inférence est identifiée par un **session ID** aléatoire (256 bits).
- Orchestrator crée le session ID, le partage avec tous les nœuds participants
- Chaque nœud maintient un état lié au session ID (clés ML-KEM reçues, buffers, etc.)
- À la fin, les nœuds nettoient leur état

### 5. Tolérance aux fautes (Byzantine)

- Si un nœud ne répond pas dans un délai (ex: 2s), l'orchestrateur peut :
  - Réessayer avec un autre nœud (si redondance configurée)
  - Retourner une erreur au client
- Les nœuds sont supposés honnêtes mais peuvent être lents ou hors ligne
- Pour résister à des nœuds malveillants, on peut ajouter :
  - Des preuves de calcul (ex: zk-SNARKs sur une couche légère) — future work
  - La redondance (ex: exécuter la même couche sur 2 nœuds et prendre la moyenne) — future work

### 6. Intégration avec Polygone-Core

Polygone-Petals est un **module** Polygone :
- Dépend de `polygone-core` pour le réseau libp2p, le DHT, l'identité
- Ajoute une nouvelle sous-commande CLI : `polygone petals`
- Peut être activé/désactivé dans `services.json`

## Flux d'utilisation

### Pour l'utilisateur final

```bash
# Démarrer un nœud Petals (optionnel, si on veut contribuer du calcul)
polygone petals node start --port 30303

# Demander une inférence
polygone petals run \
  --model "meta-llama/Llama-2-70b-chat-hf" \
  --prompt "Explique la conscience quantique en deux phrases" \
  --max-new-tokens 100
```

L'orchestrateur intégré :
1. Télécharge le modèle (ou utilise le cache local)
2. Le découpe selon les nœuds disponibles
3. Lance l'inférence en pipeline
4. Retourne le texte généré

### Pour le développeur

Ajouter Polygone-Petals comme dépendance :

```toml
[dependencies]
polygone-petals = { git = "https://github.com/lvs0/Polygone-Petals" }
```

Puis dans votre code Rust :

```rust
use polygone_petals::{PetalsClient, InferenceParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PetalsClient::new().await?;
    
    let params = InferenceParams {
        model: "meta-llama/Llama-2-70b-chat-hf".to_string(),
        prompt: "Salut, comment ça va ?".to_string(),
        max_new_tokens: 50,
        ..Default::default()
    };
    
    let response = client.generate(params).await?;
    println!("Réponse : {}", response);
    
    Ok(())
}
```

## Sécurité post-quantique

Toutes les échanges inter-nœuds utilisent :
- **ML-KEM-1024** pour l'échange de clés (résistant aux ordinateurs quantiques de 2030+)
- **AES-256-GCM** ou **ChaCha20-Poly1305** pour le chiffrement des données (supposé résistant)
- **BLAKE3** pour le hachage des clés et des IDs (résistant)

Aucune dépendance à RSA, ECDH, ou autres algorithmes vulnérables.

## Performance

| Composant | Latence estimée (70B modèle, 4 nœuds A100) |
|-----------|--------------------------------------------|
| Découpage modèle | 100ms (one-time) |
| Échange ML-KEM par saut | 50µs |
| Chiffrement AES-GCM par saut | 5µs |
| Calcul couche (FFN + Attention) | 100-200ms par couche |
| Transmission réseau (p2p) | 1-10ms selon géo |
| **Total par token** | ~200-500ms (dépend du nombre de couches sautées) |

Avec 8 nœuds en parallèle (pipeline parallèle), on peut atteindre près du temps d'un seul nœud.

## Roadmap

- v0.1.0 : Pipeline séquentiel basique (LLama 2 7B-70B)
- v0.2.0 : Redondance et tolérance aux fautes
- v0.3.0 : Intégration avec l'orchestrateur de cerveau (polygone-brain)
- v0.4.0 : Pré et post-traitement (tokenizer) distribués
- v1.0.0 : Stable, audit de sécurité, mobile SDK

## Comparaison avec les alternatives

| Solution | Centralisée ? | Post-Quantum ? | Privée ? | Open Source ? |
|----------|---------------|----------------|----------|---------------|
| OpenAI API | Oui | Non | Non | Non |
| Hugging Face Inference | Oui | Non | Semi | Oui |
| Together AI | Oui | Non | Semi | Oui |
| **Polygone-Petals** | **Non** | **Oui** | **Oui** | **Oui** |
| Modalité locale (llama.cpp) | Non | Non | Oui | Oui |

Seul Petals offre **trois** garanties : décentralisation, sécurité post-quantique, et confidentialité.

## Implémentation actuelle

Le code se trouve dans le crate `polygone-petals` :
- `src/lib.rs` : logique d'orchestration
- `src/client.rs` : client CLI et API Rust
- `src/node.rs` : nœud de calcul
- `src/crypto.rs` : wrappers ML-KEM + AES
- `src/network.rs` : intégration libp2p
- `examples/` : démonstrations d'utilisation

## Appel à contribution

Polygone-Petals est un projet ambitieux. Nous recherchons :
- Des cryptographes pour auditer l'usage de ML-KEM
- Des ingénieurs ML pour optimiser le découpage de modèles
- Des développeurs P2P pour améliorer la découverte de nœuds
- Des designers pour créer une interface de monitoring du "cerveau"

Rejoignez-nous sur : https://github.com/lvs0/Polygone-Petals

---

> « L'intelligence n'est pas dans le cerveau individuel, mais dans la synchronisation des neurones. »  
> — PETALS_NEURO Manifesto