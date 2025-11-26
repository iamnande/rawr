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

    pub fn enforce(&self, action: &str, resource_path: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_and_enforce() {
        let mut acm = Acm::new();
        acm.allow("action:Get", "resource/path");
        assert!(acm.enforce("action:Get", "resource/path"));
        assert!(!acm.enforce("action:Get", "other/path"));
    }

    #[test]
    fn test_deny_overrides_allow() {
        let mut acm = Acm::new();
        acm.allow("action:*", "resource/*");
        acm.deny("action:Delete", "resource/sensitive");
        assert!(acm.enforce("action:Get", "resource/normal"));
        assert!(!acm.enforce("action:Delete", "resource/sensitive"));
    }
}
