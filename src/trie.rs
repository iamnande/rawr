use globset::{Glob, GlobMatcher};

const WILDCARD: &str = "*";

#[derive(Debug)]
pub(crate) struct TrieNode {
    children: Vec<TrieNode>,
    pattern: GlobMatcher,
    raw_pattern: String,
    terminal: bool,
}

impl TrieNode {
    pub(crate) fn new(pattern: GlobMatcher, raw_pattern: &str) -> Self {
        TrieNode {
            children: Vec::new(),
            pattern,
            raw_pattern: raw_pattern.to_string(),
            terminal: false,
        }
    }

    pub(crate) fn default() -> Self {
        TrieNode::new(Glob::new(WILDCARD).unwrap().compile_matcher(), WILDCARD)
    }

    pub(crate) fn insert_segment(&mut self, segment_pattern: &str) -> &mut TrieNode {
        // exacto knife
        if let Some(idx) = self
            .children
            .iter()
            .position(|c| c.raw_pattern.as_str() == segment_pattern)
        {
            return &mut self.children[idx];
        }

        // lets build our flair
        let pattern = Glob::new(segment_pattern).unwrap().compile_matcher();
        let raw_pattern = segment_pattern;

        self.children.push(TrieNode::new(pattern, raw_pattern));
        self.children.last_mut().unwrap()
    }

    // this is the real workhorse. like imagine if spirit _did_ break; just
    // luggin trains up mountains and shit? dawg that's obscene.
    pub(crate) fn contains(&self, segments: &[&str]) -> bool {
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
pub(crate) struct Trie {
    root: TrieNode,
}

impl Trie {
    pub(crate) fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub(crate) fn insert(&mut self, segments: &[&str]) {
        let mut node = &mut self.root;

        for segment in segments {
            node = node.insert_segment(segment);
        }

        node.terminal = true
    }

    pub(crate) fn contains(&self, segments: &[&str]) -> bool {
        self.root.contains(segments)
    }
}
