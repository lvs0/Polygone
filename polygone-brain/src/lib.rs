mod orchestrator;
mod personalities;

use polygone_crypto::{
    kem::{decapsulate, encapsulate, generate_kem_key_pair, PublicKey, SecretKey},
    symmetric::{decrypt, encrypt},
};
use polygone_common::SessionKey;

pub use orchestrator::ReasoningOrchestrator;
pub use personalities::Personality;

/// Brain entry point
pub struct PolygoneBrain {
    /// Active personality
    current: Option<Personality>,
    /// Orchestrator for distributed reasoning
    orchestrator: ReasoningOrchestrator,
}

impl PolygoneBrain {
    pub fn new() -> Self {
        Self {
            current: None,
            orchestrator: ReasoningOrchestrator::new(),
        }
    }

    pub fn activate_personality(&mut self, name: &str) -> Result<(), String> {
        let personality = Personality::from_name(name)?;
        self.current = Some(personality);
        Ok(())
    }

    /// Reason about a query, using active personality or orchestrator
    pub fn reason(&self, query: &str) -> String {
        match &self.current {
            Some(personality) => personality.reason(query),
            None => self.orchestrator.reason(query),
        }
    }

    /// Run a multi-personality debate on a topic
    pub fn debate(&self, topic: &str) -> Vec<String> {
        self.orchestrator.debate(topic)
    }

    /// Securely exchange a thought between personalities using post-quantum cryptography
    /// 
    /// # Arguments
    /// * `sender_name` - Name of the sending personality
    /// * `recipient_name` - Name of the receiving personality
    /// * `thought` - The thought to be securely transmitted
    /// 
    /// # Returns
    /// * `Ok((ciphertext, kem_ciphertext, nonce))` - The encrypted thought, KEM encapsulation, and AES nonce
    /// * `Err(String)` - Error message if personalities not found or crypto operation fails
    pub fn exchange_secure_thought(
        &self,
        sender_name: &str,
        recipient_name: &str,
        thought: &str,
    ) -> Result<(Vec<u8>, Vec<u8>, [u8; 12]), String> {
        // Look up the personalities to verify they exist
        let _sender = Personality::from_name(sender_name)
            .map_err(|_| format!("Sender personality '{}' not found", sender_name))?;
        let _recipient = Personality::from_name(recipient_name)
            .map_err(|_| format!("Recipient personality '{}' not found", recipient_name))?;
        
        // Generate a KEM key pair for the recipient (in practice, these would be pre-shared)
        let (recipient_pk, recipient_sk) = generate_kem_key_pair();
        
        // Encapsulate a shared secret using the recipient's public key
        let (kem_ciphertext, shared_secret) = encapsulate(&recipient_pk);
        
        // Convert the shared secret to a SessionKey for symmetric encryption
        let session_key = SessionKey::new(shared_secret.as_slice().to_vec().try_into().unwrap());
        
        // Encrypt the thought using AES-256-GCM
        let (ciphertext, nonce) = encrypt(&session_key, thought.as_bytes(), &[]).map_err(|_| 
            "Encryption failed".to_string()
        )?;
        
        Ok((ciphertext, kem_ciphertext, nonce))
    }
}

impl Default for PolygoneBrain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brain_new() {
        let brain = PolygoneBrain::new();
        assert!(brain.current.is_none());
    }

    #[test]
    fn test_activate_einstein() {
        let mut brain = PolygoneBrain::new();
        assert!(brain.activate_personality("einstein").is_ok());
        assert!(brain.current.is_some());
    }

    #[test]
    fn test_reason_with_personality() {
        let mut brain = PolygoneBrain::new();
        brain.activate_personality("feynman").unwrap();
        let result = brain.reason("Explain quantum entanglement simply");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_reason_without_personality() {
        let brain = PolygoneBrain::new();
        let result = brain.reason("What is distributed systems?");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_debate() {
        let brain = PolygoneBrain::new();
        let debate = brain.debate("What is the nature of consciousness?");
        assert!(!debate.is_empty());
    }

    #[test]
    fn test_activate_schopenhauer() {
        let mut brain = PolygoneBrain::new();
        assert!(brain.activate_personality("schopenhauer").is_ok());
        assert!(brain.current.is_some());
        let result = brain.reason("What is the meaning of life?");
        assert!(result.contains("Schopenhauer"));
        assert!(result.contains("[Aphoristic]"));
    }

    #[test]
    fn test_activate_rousseau() {
        let mut brain = PolygoneBrain::new();
        assert!(brain.activate_personality("rousseau").is_ok());
        assert!(brain.current.is_some());
        let result = brain.reason("How should society be organized?");
        assert!(result.contains("Rousseau"));
        assert!(result.contains("[Narrative]"));
    }

    #[test]
    fn test_activate_musk() {
        let mut brain = PolygoneBrain::new();
        assert!(brain.activate_personality("musk").is_ok());
        assert!(brain.current.is_some());
        let result = brain.reason("What is the future of transportation?");
        assert!(result.contains("Musk"));
        assert!(result.contains("[Experimental]"));
    }
}