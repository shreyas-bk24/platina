use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        self.find(word).is_some_and(|n| n.end)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, s: &str) -> Option<&Trie> {
        s.chars().try_fold(self, |node, c| node.children.get(&c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        let mut t = Trie::default();
        for w in ["app", "apple", "apt", "bat", "bath"] { t.insert(w); }
        assert!(t.search("app") && t.search("bath"));
        assert!(!t.search("ap"));
        assert!(t.starts_with("ap") && !t.starts_with("bad"));
        assert!(t.starts_with(""));
    }
}
