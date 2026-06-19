pub mod personalities;
pub mod orchestrator;

pub use personalities::*;
pub use orchestrator::*;

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

    pub fn reason(&self, query: &str) -> Result<String, String> {
        match &self.current {
            Some(personality) => {
                personality.reason(query)
            }
            None => {
                self.orchestrator.reason(query)
            }
        }
    }

    pub fn debate(&self, topic: &str) -> Result<Vec<String>, String> {
        self.orchestrator.debate(topic)
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
}