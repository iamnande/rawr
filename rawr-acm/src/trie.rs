use globset::{Glob, GlobMatcher};

const WILDCARD: &str = "*";
const ROOT_PATTERN: &str = ".";

#[derive(Debug)]
pub(crate) struct TrieNode {
    literal_children: Vec<(String, TrieNode)>,
    glob_children: Vec<TrieNode>,
    pattern: Option<GlobMatcher>,
    raw_pattern: String,
    terminal: bool,
}

impl TrieNode {
    pub(crate) fn new(pattern: GlobMatcher, raw_pattern: &str) -> Self {
        TrieNode {
            literal_children: Vec::new(),
            glob_children: Vec::new(),
            pattern: Some(pattern),
            raw_pattern: raw_pattern.to_string(),
            terminal: false,
        }
    }

    pub(crate) fn default() -> Self {
        TrieNode {
            literal_children: Vec::new(),
            glob_children: Vec::new(),
            pattern: None,
            raw_pattern: ROOT_PATTERN.to_string(),
            terminal: false,
        }
    }

    pub(crate) fn insert_segment(&mut self, segment_pattern: &str) -> &mut TrieNode {
        if !segment_pattern.contains(WILDCARD) {
            match self
                .literal_children
                .binary_search_by(|(key, _)| key.as_str().cmp(segment_pattern))
            {
                Ok(idx) => &mut self.literal_children[idx].1,
                Err(idx) => {
                    let pattern = Glob::new(segment_pattern).unwrap().compile_matcher();
                    let new_node = TrieNode::new(pattern, segment_pattern);
                    self.literal_children
                        .insert(idx, (segment_pattern.to_string(), new_node));
                    &mut self.literal_children[idx].1
                }
            }
        } else {
            if let Some(idx) = self
                .glob_children
                .iter()
                .position(|c| c.raw_pattern.as_str() == segment_pattern)
            {
                return &mut self.glob_children[idx];
            }

            let pattern = Glob::new(segment_pattern).unwrap().compile_matcher();
            self.glob_children
                .push(TrieNode::new(pattern, segment_pattern));
            self.glob_children.last_mut().unwrap()
        }
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
        if let Ok(idx) = self
            .literal_children
            .binary_search_by(|(key, _)| key.as_str().cmp(current))
            && self.literal_children[idx].1.contains(remaining)
        {
            return true;
        }

        // you can't triple stamp a double stamp!
        for glob_child in &self.glob_children {
            if let Some(ref pattern) = glob_child.pattern
                && pattern.is_match(current)
            {
                // ugh, that's so fetch
                if glob_child.raw_pattern == WILDCARD && glob_child.terminal {
                    return true;
                }

                if glob_child.contains(remaining) {
                    return true;
                }
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
