use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
pub struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn add(&mut self, x: i32) {
        self.lo.push(x);
        if let Some(top) = self.lo.pop() {
            self.hi.push(Reverse(top));
        }
        if self.hi.len() > self.lo.len() {
            if let Some(Reverse(y)) = self.hi.pop() {
                self.lo.push(y);
            }
        }
    }

    pub fn median(&self) -> Option<f64> {
        match (self.lo.peek(), self.hi.peek()) {
            (None, _) => None,
            (Some(&a), Some(&Reverse(b))) if self.lo.len() == self.hi.len() => {
                Some((f64::from(a) + f64::from(b)) / 2.0)
            }
            (Some(&a), _) => Some(f64::from(a)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        let mut m = MedianFinder::default();
        assert_eq!(m.median(), None);
        let mut seen = Vec::new();
        for x in [5, 15, 1, 3, 8, 7, 9, 10] { m.add(x); seen.push(m.median().unwrap()); }
        assert_eq!(seen, vec![5.0, 10.0, 5.0, 4.0, 5.0, 6.0, 7.0, 7.5]);
    }
}
