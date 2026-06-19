

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
            "schopenhauer" => Ok(Personality {
                name: "Schopenhauer",
                style: ReasoningStyle::Aphoristic,
                knowledge_domains: vec!["philosophy", "ethics", "aesthetics", "will", "pessimism"],
            }),
            "rousseau" => Ok(Personality {
                name: "Rousseau",
                style: ReasoningStyle::Narrative,
                knowledge_domains: vec!["political philosophy", "education", "social contract", "nature", "freedom"],
            }),
            "musk" => Ok(Personality {
                name: "Musk",
                style: ReasoningStyle::Experimental,
                knowledge_domains: vec!["engineering", "physics", "business", "space", "AI", "renewable energy"],
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
        format!("[Analytical] {} — {}", self.name, query)
    }

    fn visual(&self, query: &str) -> String {
        format!("[Visual] {} — {}", self.name, query)
    }

    fn experimental(&self, query: &str) -> String {
        format!("[Experimental] {} — {}", self.name, query)
    }

    fn narrative(&self, query: &str) -> String {
        format!("[Narrative] {} — {}", self.name, query)
    }

    fn aphoristic(&self, query: &str) -> String {
        format!("[Aphoristic] {} — {}", self.name, query)
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
        assert!(reasoning.contains("Einstein"));

        let feynman = Personality::from_name("feynman").unwrap();
        let reasoning = feynman.reason("How do magnets work?");
        assert!(reasoning.contains("Feynman"));
    }
}