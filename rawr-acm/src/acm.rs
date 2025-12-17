use smallvec::SmallVec;

use crate::trie::{STACK_CAPACITY, Trie};

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

    pub fn allow(&mut self, action: &str, resource_path: &str) {
        self.allow.insert(
            action
                .split(ACTION_SEPARATOR)
                .chain(resource_path.split(RESOURCE_SEPARATOR)),
        );
    }

    pub fn deny(&mut self, action: &str, resource_path: &str) {
        self.deny.insert(
            action
                .split(ACTION_SEPARATOR)
                .chain(resource_path.split(RESOURCE_SEPARATOR)),
        );
    }

    pub fn enforce(&self, action: &str, resource_path: &str) -> bool {
        // cut my life into pieces - collect once, reuse twice
        let segments: SmallVec<[&str; STACK_CAPACITY]> = action
            .split(ACTION_SEPARATOR)
            .chain(resource_path.split(RESOURCE_SEPARATOR))
            .collect();

        // check the watch list
        if self.deny.contains(&segments) {
            return false;
        }

        // okay, now check "the list"
        self.allow.contains(&segments)
    }

    pub fn enforce_batch(&self, requests: &[(&str, &str)]) -> Vec<bool> {
        let mut results = Vec::with_capacity(requests.len());
        let mut segments: SmallVec<[&str; STACK_CAPACITY]> = SmallVec::new();

        for (action, resource_path) in requests {
            segments.clear();
            segments.extend(
                action
                    .split(ACTION_SEPARATOR)
                    .chain(resource_path.split(RESOURCE_SEPARATOR)),
            );

            let granted = !self.deny.contains(&segments) && self.allow.contains(&segments);
            results.push(granted);
        }

        results
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

    #[test]
    fn test_enforce_batch() {
        let mut acm = Acm::new();
        acm.allow("action:Get", "resource/*");
        acm.deny("action:Delete", "resource/protected");

        let results = acm.enforce_batch(&[
            ("action:Get", "resource/foo"),          // allow match
            ("action:Delete", "resource/bar"),       // no allow
            ("action:Delete", "resource/protected"), // explicit deny
            ("action:Get", "resource/protected"),    // allow match
        ]);

        assert_eq!(results, vec![true, false, false, true]);
    }
}
