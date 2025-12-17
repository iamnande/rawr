use globset::{Glob, GlobMatcher};

const WILDCARD: &str = "*";
const ROOT_PATTERN: &str = ".";

#[derive(Debug)]
pub(crate) enum NodePattern {
    Root,
    Literal,
    Glob(GlobMatcher),
}

#[derive(Debug)]
pub(crate) struct TrieNode {
    literal_children: Vec<(String, TrieNode)>,
    glob_children: Vec<TrieNode>,
    pattern: NodePattern,
    raw_pattern: String,
    terminal: bool,
}

impl TrieNode {
    pub(crate) fn new(pattern: NodePattern, raw_pattern: &str) -> Self {
        TrieNode {
            literal_children: Vec::new(),
            glob_children: Vec::new(),
            pattern,
            raw_pattern: raw_pattern.to_string(),
            terminal: false,
        }
    }

    pub(crate) fn default() -> Self {
        TrieNode {
            literal_children: Vec::new(),
            glob_children: Vec::new(),
            pattern: NodePattern::Root,
            raw_pattern: ROOT_PATTERN.to_string(),
            terminal: false,
        }
    }

    pub(crate) fn get_or_insert_literal(&mut self, segment_pattern: &str) -> &mut TrieNode {
        match self
            .literal_children
            .binary_search_by(|(key, _)| key.as_str().cmp(segment_pattern))
        {
            Ok(idx) => &mut self.literal_children[idx].1,
            Err(idx) => {
                let new_node = TrieNode::new(NodePattern::Literal, segment_pattern);
                self.literal_children
                    .insert(idx, (segment_pattern.to_string(), new_node));
                &mut self.literal_children[idx].1
            }
        }
    }

    pub(crate) fn get_or_insert_glob(&mut self, segment_pattern: &str) -> &mut TrieNode {
        if let Some(idx) = self
            .glob_children
            .iter()
            .position(|c| c.raw_pattern == segment_pattern)
        {
            &mut self.glob_children[idx]
        } else {
            let pattern = Glob::new(segment_pattern).unwrap().compile_matcher();
            let new_node = TrieNode::new(NodePattern::Glob(pattern), segment_pattern);
            self.glob_children.push(new_node);
            let last_idx = self.glob_children.len() - 1;
            &mut self.glob_children[last_idx]
        }
    }

    pub(crate) fn insert_segment(&mut self, segment_pattern: &str) -> &mut TrieNode {
        if !segment_pattern.contains(WILDCARD) {
            self.get_or_insert_literal(segment_pattern)
        } else {
            self.get_or_insert_glob(segment_pattern)
        }
    }

    // this is the real workhorse. like imagine if spirit _did_ break; just
    // luggin trains up mountains and shit? dawg that's obscene.
    pub(crate) fn contains(&self, segments: &[&str]) -> bool {
        let mut stack = Vec::with_capacity(segments.len());
        stack.push((self, 0));

        // shift(@stack) - *sigh*, dawg i miss Perl
        while let Some((node, idx)) = stack.pop() {
            // end of the road, bucko
            if idx == segments.len() {
                return node.terminal;
            }

            // uhm, like - do you even work here?
            let current: &str = segments[idx];
            if let Ok(pos) = node
                .literal_children
                .binary_search_by(|(k, _)| k.as_str().cmp(current))
            {
                stack.push((&node.literal_children[pos].1, idx + 1));
            }

            // you can't triple stamp a double stamp!
            for glob_child in &node.glob_children {
                if let NodePattern::Glob(matcher) = &glob_child.pattern
                    && matcher.is_match(current)
                {
                    stack.push((glob_child, idx + 1));
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

    pub(crate) fn insert<I>(&mut self, segments: I)
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut node = &mut self.root;

        for segment in segments {
            node = node.insert_segment(segment.as_ref());
        }

        node.terminal = true
    }

    pub(crate) fn contains(&self, segments: &[&str]) -> bool {
        self.root.contains(segments)
    }
}
