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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_creation() {
        let role = Role::new("TestRole", "Description");
        assert_eq!(role.name, "TestRole");
        assert_eq!(role.description, "Description");
        assert_eq!(role.policies.len(), 0);
    }

    #[test]
    fn test_role_with_allow_policy() {
        let role = Role::new("TestRole", "Description").with_policy(Policy::allow(
            vec!["action".to_string()],
            vec!["resource".to_string()],
        ));
        assert_eq!(role.policies.len(), 1);
        assert!(matches!(role.policies[0].effect, Effect::Allow));
    }

    #[test]
    fn test_role_with_deny_policy() {
        let role = Role::new("TestRole", "Description").with_policy(Policy::deny(
            vec!["action".to_string()],
            vec!["resource".to_string()],
        ));
        assert_eq!(role.policies.len(), 1);
        assert!(matches!(role.policies[0].effect, Effect::Deny));
    }
}
