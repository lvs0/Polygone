use std::collections::HashMap;

/// A simulated personality with a distinct reasoning style
#[derive(Debug, Clone)]
pub struct Personality {
    pub name: &'static str,
    pub style: ReasoningStyle,
    pub knowledge_domains: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub enum ReasoningStyle {
    /// Logical, mathematical, first-principles
    Analytical,
    /// Creative, visual, thought experiments
    Visual,
    /// Practical, experimental, hands-on
    Experimental,
    /// Historical, contextual, narrative
    Narrative,
    /// Concise, aphoristic, wisdom
    Aphoristic,
}

impl Personality {
    /// Create a personality from name
    pub fn from_name(name: &str) -> Result<Self, String> {
        match name.to_lowercase().as_str() {
            "einstein" => Ok(Personality {
                name: "Einstein",
                style: ReasoningStyle::Visual,
                knowledge_domains: vec!["physics", "relativity", "quantum"],
            }),
            "feynman" => Ok(Personality {
                name: "Feynman",
                style: ReasoningStyle::Experimental,
                knowledge_domains: vec!["physics", "quantum electrodynamics", "teaching"],
            }),
            "turing" => Ok(Personality {
                name: "Turing",
                style: ReasoningStyle::Analytical,
                knowledge_domains: vec!["computer science", "cryptography", "AI"],
            }),
            "lovelace" => Ok(Personality {
                name: "Lovelace",
                style: ReasoningStyle::Narrative,
                knowledge_domains: vec!["mathematics", "computing", "poetry"],
            }),
            "newton" => Ok(Personality {
                name: "Newton",
                style: ReasoningStyle::Analytical,
                knowledge_domains: vec!["physics", "mathematics", "optics"],
            }),
            "curie" => Ok(Personality {
                name: "Curie",
                style: ReasoningStyle::Experimental,
                knowledge_domains: vec!["radioactivity", "chemistry", "physics"],
            }),
            "darwin" => Ok(Personality {
                name: "Darwin",
                style: ReasoningStyle::Narrative,
                knowledge_domains: vec!["biology", "evolution", "naturalism"],
            }),
            "satoshi" => Ok(Personality {
                name: "Satoshi",
                style: ReasoningStyle::Aphoristic,
                knowledge_domains: vec!["cryptography", "economics", "distributed systems"],
            }),
            _ => Err(format!("Unknown personality: {}", name)),
        }
    }

    /// Apply reasoning style to a query
    pub fn reason(&self, query: &str) -> String {
        match self.style {
            ReasoningStyle::Analytical => self.analytical(query),
            ReasoningStyle::Visual => self.visual(query),
            ReasoningStyle::Experimental => self.experimental(query),
            ReasoningStyle::Narrative => self.narrative(query),
            ReasoningStyle::Aphoristic => self.aphoristic(query),
        }
    }

    fn analytical(&self, query: &str) -> String {
        format!("Analyzing [{}] through first principles: {}", self.name, query)
    }

    fn visual(&self, query: &str) -> String {
        format!("Visualizing [{}]: Imagine {}, what do you see?", self.name, query)
    }

    fn experimental(&self, query: &str) -> String {
        format!("Testing [{}] in the lab: Let's try {} and observe the results", self.name, query)
    }

    fn narrative(&self, query: &str) -> String {
        format!("The story of [{}]: Once upon a time, {} unfolded like this...", self.name, query)
    }

    fn aphoristic(&self, query: &str) -> String {
        format!("Wisdom of [{}]: {} — remember that.", self.name, query)
    }
}

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
    fn test_personality_creation() {
        let einstein = Personality::from_name("einstein").unwrap();
        assert_eq!(einstein.name, "Einstein");
        assert!(matches!(einstein.style, ReasoningStyle::Visual));

        let feynman = Personality::from_name("feynman").unwrap();
        assert_eq!(feynman.name, "Feynman");
        assert!(matches!(feynman.style, ReasoningStyle::Experimental));
    }

    #[test]
    fn test_unknown_personality() {
        assert!(Personality::from_name("unknown").is_err());
    }

    #[test]
    fn test_reason_styles() {
        let einstein = Personality::from_name("einstein").unwrap();
        let reasoning = einstein.reason("Explain E=mc^2");
        assert!(reasoning.contains("Visualizing"));
        assert!(reasoning.contains("Einstein"));

        let feynman = Personality::from_name("feynman").unwrap();
        let reasoning = feynman.reason("How do magnets work?");
        assert!(reasoning.contains("Testing"));
        assert!(reasoning.contains("Feynman"));
    }

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