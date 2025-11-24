use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub effect: Effect,
    pub actions: Vec<String>,
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub policies: Vec<Policy>,
}

impl Role {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Role {
            name: name.into(),
            description: description.into(),
            policies: Vec::new(),
        }
    }

    pub fn with_policy(mut self, policy: Policy) -> Self {
        self.policies.push(policy);
        self
    }
}

impl Policy {
    pub fn allow(actions: Vec<String>, resources: Vec<String>) -> Self {
        Policy {
            effect: Effect::Allow,
            actions,
            resources,
        }
    }

    pub fn deny(actions: Vec<String>, resources: Vec<String>) -> Self {
        Policy {
            effect: Effect::Deny,
            actions,
            resources,
        }
    }
}
