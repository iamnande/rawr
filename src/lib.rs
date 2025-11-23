use std::sync::Arc;
use globset::{Glob, GlobMatcher};

#[derive(Debug)]
pub struct TrieNode {
    children: Vec<TrieNode>,
    pattern: Arc<GlobMatcher>,
    raw_pattern: Arc<str>,
    terminal: bool,
}

impl TrieNode {
    fn new(pattern: Arc<GlobMatcher>, raw_pattern: Arc<str>) -> Self {
        TrieNode {
            children: Vec::new(),
            pattern,
            raw_pattern,
            terminal: false,
        }
    }

    fn default() -> Self {
        let matcher = Arc::new(Glob::new("*").unwrap().compile_matcher());
        TrieNode::new(matcher, Arc::from("*"))
    }

    fn insert_segment(&mut self, segment_pattern: &str) -> &mut TrieNode {
        // exacto knife
        if let Some(idx) = self.children.iter()
            .position(|c| c.raw_pattern.as_ref() == segment_pattern)
        {
            return &mut self.children[idx];
        }

        // lets build our flair
        let pattern = Arc::new(Glob::new(segment_pattern)
            .unwrap()
            .compile_matcher());
        let raw_pattern = Arc::from(segment_pattern);

        self.children.push(TrieNode::new(pattern, raw_pattern));
        self.children.last_mut().unwrap()
    }

    // this is the real workhorse. like imagine if spirit _did_ break; just
    // luggin trains up mountains and shit? dawg that's obscene.
    fn contains(&self, segments: &[&str]) -> bool {
        // end of the road, bucko
        if segments.is_empty() {
            return self.terminal;
        }

        // shift(@stack) - *sigh*, dawg i miss Perl
        let current = segments[0];
        let remaining = &segments[1..];

        // uhm, like - do you even work here?
        for child in &self.children {
            if child.pattern.is_match(current) && child.contains(remaining) {
                return true;
            }
        }

        // super sus dude, no za for you
        false
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, segments: &[&str]) {
        let mut node = &mut self.root;

        for segment in segments {
            node = node.insert_segment(segment);
        }

        node.terminal = true
    }

    fn contains(&self, segments: &[&str]) -> bool {
        self.root.contains(segments)
    }
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_acm_with_logging() {
        let mut acm = ACM::new();

        acm.deny("networks:*", "VLAN-1");
        acm.allow("networks:GetVLAN", "*");
        acm.allow("networks:UpdateVLAN", "VLAN-20");
        acm.allow("networks:*", "VLAN-70");
        acm.allow("networks:AddVLANTag", "nick/lab/*");
        acm.allow("calendar:Get*", "laura/*");

        let test_cases = vec![
            ("networks:GetVLAN", "VLAN-50", true),
            ("networks:UpdateVLAN", "VLAN-20", true),
            ("networks:UpdateVLAN", "VLAN-30", false),
            ("networks:GetVLAN", "VLAN-1", false),
            ("networks:DeleteVLAN", "VLAN-70", true),
            ("networks:AddVLANTag", "nick/lab/VLAN-70", true),
            ("calendar:GetCalendar", "laura/shared-family-calendar", true),
        ];

        for (action, resource, expected) in test_cases {
            let granted = acm.authorized(action, resource);
            println!(
                "[acm.authorized] action: {action}, resource: {resource} - GRANTED: {granted}, EXPECTED: {expected}"
            );
            assert_eq!(granted, expected);
        }
    }
}
