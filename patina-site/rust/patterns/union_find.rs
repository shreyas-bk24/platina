use std::cmp::Ordering;

pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), rank: vec![0; n] }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            Ordering::Less => self.parent[ra] = rb,
            Ordering::Greater => self.parent[rb] = ra,
            Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        true
    }
}

pub fn count_components(n: usize, edges: &[(usize, usize)]) -> usize {
    let mut dsu = DisjointSet::new(n);
    let merges = edges.iter().filter(|&&(a, b)| dsu.union(a, b)).count();
    n - merges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(count_components(5, &[(0, 1), (1, 2), (3, 4)]), 2);
        assert_eq!(count_components(8, &[(0, 1), (2, 3), (1, 2), (4, 5), (6, 7), (5, 6), (0, 3)]), 2);
        assert_eq!(count_components(3, &[]), 3);
        let mut d = DisjointSet::new(4);
        assert!(d.union(0, 1));
        assert!(!d.union(1, 0));
        assert_eq!(d.find(1), d.find(0));
    }
}
