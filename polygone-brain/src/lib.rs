pub mod orchestrator;
pub mod personalities;

pub use personalities::Personality;
pub use orchestrator::ReasoningOrchestrator;

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
}