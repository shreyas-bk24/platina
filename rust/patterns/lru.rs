use std::collections::HashMap;

struct Node { key: i32, val: i32, prev: usize, next: usize }

pub struct LruCache {
    cap: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>, // nodes[0] is a sentinel
}

impl LruCache {
    pub fn new(cap: usize) -> Self {
        let sentinel = Node { key: 0, val: 0, prev: 0, next: 0 };
        Self { cap, map: HashMap::new(), nodes: vec![sentinel] }
    }

    fn unlink(&mut self, i: usize) {
        let (p, n) = (self.nodes[i].prev, self.nodes[i].next);
        self.nodes[p].next = n;
        self.nodes[n].prev = p;
    }

    fn push_front(&mut self, i: usize) {
        let first = self.nodes[0].next;
        self.nodes[i].prev = 0;
        self.nodes[i].next = first;
        self.nodes[first].prev = i;
        self.nodes[0].next = i;
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        let &i = self.map.get(&key)?;
        self.unlink(i);
        self.push_front(i);
        Some(self.nodes[i].val)
    }

    pub fn put(&mut self, key: i32, val: i32) {
        if self.cap == 0 {
            return;
        }
        if let Some(&i) = self.map.get(&key) {
            self.nodes[i].val = val;
            self.unlink(i);
            self.push_front(i);
            return;
        }
        let i = if self.map.len() == self.cap {
            let lru = self.nodes[0].prev;
            self.unlink(lru);
            self.map.remove(&self.nodes[lru].key);
            self.nodes[lru].key = key;
            self.nodes[lru].val = val;
            lru
        } else {
            self.nodes.push(Node { key, val, prev: 0, next: 0 });
            self.nodes.len() - 1
        };
        self.map.insert(key, i);
        self.push_front(i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        let mut c = LruCache::new(2);
        c.put(1, 10); c.put(2, 20);
        assert_eq!(c.get(1), Some(10));
        c.put(3, 30);
        assert_eq!(c.get(2), None);
        c.put(4, 40);
        assert_eq!((c.get(1), c.get(3), c.get(4)), (None, Some(30), Some(40)));
        let mut z = LruCache::new(0);
        z.put(1, 1);
        assert_eq!(z.get(1), None);
    }
}
