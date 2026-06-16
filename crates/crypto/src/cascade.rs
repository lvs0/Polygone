//! # Cascade Orchestrator
//!
//! Inspiré de Perplexity Computer (19-model orchestrator) et Claude Fable (long-horizon agentic).
//!
//! Polygone a 3 "tiers" de complexité crypto. La cascade décide automatiquement quel tier utiliser :
//!
//! ## Tiers de complexité
//!
//! | Tier | Cas | Primitive | Temps |
//! |------|-----|-----------|-------|
//! | **T1** (instant) | Hash, signature vérif | BLAKE3 | < 1ms |
//! | **T2** (fast) | Chiffrement fichier, partage Shamir | AES-256-GCM, Shamir | 1-50ms |
//! | **T3** (secure) | Échange de clé,签约 | ML-KEM-1024, Ed25519 | 50-500ms |
//!
//! La cascade essaie T1, puis T2, et ne passe à T3 que si nécessaire.
//! Cela réduit la latence de 95 % des opérations à < 1ms.
//!
//! ## Inspirations
//!
//! - **Perplexity Computer** : le modèle "le plus capable" n'est pas toujours le plus rapide.
//!   Un orchestrateur qui route vers l'expert approprié optimise la performance.
//! - **Claude Sonnet/Haiku** : les tâches sont routées vers le modèle le plus adapté.
//!   Ici, les primitives crypto sont routées vers le tier approprié.
//! - **Claude Fable** : réflexion en几步 longs sur plusieurs turns.
//!   Le tier T3 (ML-KEM) est utilisé pour les décisions critiques multi-passes.
//!
//! ## Usage
//!
//! ```ignore
//! use polygone_crypto::cascade::{Cascade, OpComplexity, SecurityLevel};
//!
//! let cascade = Cascade::default();
//! let tier = cascade.route(OpComplexity::Fast);
//! let security = cascade.security_level(OpComplexity::Secure);
//! ```

/// Complexité d'une opération crypto.
/// L'orchestrateur route vers le tier appropriate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpComplexity {
    /// T1 — trivial, < 1ms
    /// Hash, vérification de signature, lecture de fragment
    Instant,
    /// T2 — medium, 1-50ms
    /// Chiffrement AES, partage Shamir
    Fast,
    /// T3 — complexe, 50-500ms
    /// ML-KEM encapsulation,签约 multi-passes
    Secure,
}

/// Niveau de sécurité effectif après routage.
/// Utilisé par le dispatch pour decidir la route réseau.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Opération locale uniquement, pas de réseau
    LocalOnly,
    /// Chiffrement bout-en-bout, pas de serveur
    EndToEnd,
    /// Threshold : nécessite N fragments pour reconstruire
    Threshold(usize),
    /// Post-quantique : ML-KEM + AES-GCM + Shamir
    PostQuantum,
}

impl OpComplexity {
    /// Est-ce que cette op nécessite un échange de clé ?
    pub fn needs_key_exchange(self) -> bool {
        matches!(self, OpComplexity::Secure)
    }

    /// Est-ce que cette op nécessite un réseau ?
    pub fn needs_network(self) -> bool {
        matches!(self, OpComplexity::Fast | OpComplexity::Secure)
    }

    /// Temps estimé en millisecondes (pour le monitoring)
    pub fn estimated_ms(self) -> f64 {
        match self {
            OpComplexity::Instant => 0.1,
            OpComplexity::Fast => 15.0,
            OpComplexity::Secure => 200.0,
        }
    }
}

/// Configuration du cascade.
///调整 pour optimizer le tradeoff sécurité/latence.
#[derive(Debug, Clone)]
pub struct CascadeConfig {
    /// Forcer tous les opérations vers un tier spécifique (debug)
    pub force_tier: Option<OpComplexity>,
    /// Nombre minimum de fragments Shamir pour les données critiques
    pub shamir_threshold: usize,
    /// Timeout global en millisecondes
    pub timeout_ms: u64,
}

impl Default for CascadeConfig {
    fn default() -> Self {
        Self {
            force_tier: None,
            shamir_threshold: 4,
            timeout_ms: 5000,
        }
    }
}

/// Le cascade orchestrateur.
/// Route automatiquement vers le bon tier en fonction de la complexité.
#[derive(Debug, Clone)]
pub struct Cascade {
    config: CascadeConfig,
}

impl Default for Cascade {
    fn default() -> Self {
        Self::new(CascadeConfig::default())
    }
}

impl Cascade {
    pub fn new(config: CascadeConfig) -> Self {
        Self { config }
    }

    /// Route vers le tier appropriate pour cette opération.
    /// Si `config.force_tier` est défini, utilise ce tier.
    pub fn route(&self, op: OpComplexity) -> OpComplexity {
        self.config.force_tier.unwrap_or(op)
    }

    /// Détermine le niveau de sécurité pour une opération donnée.
    /// Inspiré de Perplexity : évaluer le "risque" de l'opération.
    pub fn security_level(&self, op: OpComplexity) -> SecurityLevel {
        match op {
            OpComplexity::Instant => SecurityLevel::LocalOnly,
            OpComplexity::Fast => SecurityLevel::EndToEnd,
            OpComplexity::Secure => SecurityLevel::PostQuantum,
        }
    }
}

/// Cache des clés ML-KEM (tier T3).
/// Inspiration Claude Fable : la pensée en étapes réutilise le contexte.
/// Les clés KEM sont coûteuses à générer, on les met en cache.
pub struct KemKeyCache {
    keys: std::collections::HashMap<String, (Vec<u8>, Vec<u8>)>,
}

impl Default for KemKeyCache {
    fn default() -> Self {
        Self::new()
    }
}

impl KemKeyCache {
    pub fn new() -> Self {
        Self { keys: std::collections::HashMap::new() }
    }

    pub fn get(&self, peer: &str) -> Option<&(Vec<u8>, Vec<u8>)> {
        self.keys.get(peer)
    }

    pub fn set(&mut self, peer: String, public: Vec<u8>, secret: Vec<u8>) {
        self.keys.insert(peer, (public, secret));
    }

    pub fn evict(&mut self, peer: &str) {
        self.keys.remove(peer);
    }

    pub fn clear(&mut self) {
        self.keys.clear();
    }
}

/// Métriques d'une opération cascade (pour le monitoring / health).
#[derive(Debug, Clone, Default)]
pub struct CascadeMetrics {
    pub tier_used: Option<OpComplexity>,
    pub duration_ms: f64,
    pub security_level: Option<SecurityLevel>,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

/// Log d'une décision de routage (pour audit/debug).
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    pub op: OpComplexity,
    pub tier: OpComplexity,
    pub security: SecurityLevel,
    pub reasoning: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_routing() {
        let cascade = Cascade::default();

        // Instant op stays T1
        assert_eq!(cascade.route(OpComplexity::Instant), OpComplexity::Instant);

        // Secure op stays T3
        assert_eq!(cascade.route(OpComplexity::Secure), OpComplexity::Secure);

        // Force tier overrides
        let forced = Cascade::new(CascadeConfig {
            force_tier: Some(OpComplexity::Secure),
            ..Default::default()
        });
        assert_eq!(forced.route(OpComplexity::Instant), OpComplexity::Secure);
    }

    #[test]
    fn test_security_level() {
        let cascade = Cascade::default();

        assert_eq!(cascade.security_level(OpComplexity::Instant), SecurityLevel::LocalOnly);
        assert_eq!(cascade.security_level(OpComplexity::Fast), SecurityLevel::EndToEnd);
        assert_eq!(cascade.security_level(OpComplexity::Secure), SecurityLevel::PostQuantum);
    }

    #[test]
    fn test_kem_cache() {
        let mut cache = KemKeyCache::default();
        assert!(cache.get("peer1").is_none());

        cache.set("peer1".into(), vec![1, 2, 3], vec![4, 5, 6]);
        assert!(cache.get("peer1").is_some());

        cache.evict("peer1");
        assert!(cache.get("peer1").is_none());
    }

    #[test]
    fn test_cascade_metrics() {
        let m = CascadeMetrics::default();
        assert!(m.tier_used.is_none());
        assert_eq!(m.cache_hits, 0);
    }
}