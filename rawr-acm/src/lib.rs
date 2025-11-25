use rawr_pap::{Effect, Role};
mod trie;

use trie::Trie;

const ACTION_SEPARATOR: &str = ":";
const RESOURCE_SEPARATOR: &str = "/";

#[derive(Debug)]
pub struct Acm {
    allow: Trie,
    deny: Trie,
}

impl Default for Acm {
    fn default() -> Self {
        Self::new()
    }
}

impl Acm {
    pub fn new() -> Self {
        Acm {
            allow: Trie::new(),
            deny: Trie::new(),
        }
    }

    pub fn split_action(action: &str) -> Vec<&str> {
        action.split(ACTION_SEPARATOR).collect()
    }

    pub fn split_resource_path(resource_path: &str) -> Vec<&str> {
        resource_path.split(RESOURCE_SEPARATOR).collect()
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
}
