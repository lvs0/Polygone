use crate::personalities::Personality;

/// Orchestrator for distributed reasoning (debate between personalities)
pub struct ReasoningOrchestrator {
    personalities: Vec<Personality>,
}

impl ReasoningOrchestrator {
    pub fn new() -> Self {
        let mut orchestrator = ReasoningOrchestrator { personalities: Vec::new() };
        // Initialize with a diverse set
        let names = ["einstein", "feynman", "turing", "lovelace", "newton", "curie", "darwin", "satoshi"];
        for name in names {
            if let Ok(p) = Personality::from_name(name) {
                orchestrator.personalities.push(p);
            }
        }
        orchestrator
    }

    /// Run a debate: each personality gives their take
    pub fn debate(&self, topic: &str) -> Vec<String> {
        self.personalities
            .iter()
            .map(|p| format!("[{}] {}", p.name, p.reason(topic)))
            .collect()
    }

    /// General reasoning: pick the most relevant personality (simple heuristic)
    pub fn reason(&self, query: &str) -> String {
        // For now, just cycle through or pick first
        if let Some(p) = self.personalities.first() {
            format!("[Orchestrator] {}", p.reason(query))
        } else {
            format!("No personalities available")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_debate() {
        let orchestrator = ReasoningOrchestrator::new();
        let debate = orchestrator.debate("What is the nature of reality?");
        assert!(!debate.is_empty());
        // Should have entries from multiple personalities
        let combined = debate.join("\n");
        assert!(combined.contains("[Einstein]"));
        assert!(combined.contains("[Feynman]"));
    }

    #[test]
    fn test_orchestrator_reason() {
        let orchestrator = ReasoningOrchestrator::new();
        let reasoning = orchestrator.reason("Test query");
        assert!(reasoning.contains("[Orchestrator]"));
    }
}