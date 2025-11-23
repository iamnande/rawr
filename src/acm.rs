use crate::trie::Trie;
use crate::role::{Role, Effect};

#[derive(Debug)]
pub struct ACM {
    allow: Trie,
    deny: Trie,
}

impl ACM {
    pub fn new() -> Self {
        ACM {
            allow: Trie::new(),
            deny: Trie::new(),
        }
    }

    fn split_action(action: &str) -> Vec<&str> {
        action.split(':').collect()
    }

    fn split_resource_path(resource_path: &str) -> Vec<&str> {
        resource_path.split('/').collect()
    }

    pub fn allow(&mut self, action: &str, resource_path: &str) {
        let mut segments = Self::split_action(action);
        segments.extend(Self::split_resource_path(resource_path));
        self.allow.insert(&segments);
    }

    pub fn deny(&mut self, action: &str, resource_path: &str) {
        let mut segments = Self::split_action(action);
        segments.extend(Self::split_resource_path(resource_path));
        self.deny.insert(&segments);
    }

    pub fn authorized(&self, action: &str, resource_path: &str) -> bool {
        // cut my life into pieces
        let mut segments = Self::split_action(action);
        segments.extend(Self::split_resource_path(resource_path));

        // check the watch list
        if self.deny.contains(&segments) {
            return false;
        }

        // okay, now check "the list"
        self.allow.contains(&segments)
    }

    // TODO(nick): find a new home for this. PIP?
    // this here be PDP land.
    pub fn apply_role(&mut self, role: &Role) {
        for policy in &role.policies {
            for action in &policy.actions {
                for resource in &policy.resources {
                    match policy.effect {
                        Effect::Allow => self.allow(action, resource),
                        Effect::Deny => self.deny(action, resource),
                    }
                }
            }
        }
    }
}
