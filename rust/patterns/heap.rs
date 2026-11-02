use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: &[i32], k: usize) -> Option<i32> {
    let mut heap = BinaryHeap::with_capacity(k + 1);
    for &x in nums {
        heap.push(Reverse(x));
        if heap.len() > k {
            heap.pop();
        }
    }
    if k == 0 || heap.len() < k {
        return None;
    }
    heap.peek().map(|&Reverse(x)| x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(find_kth_largest(&[3, 2, 1, 5, 6, 4], 2), Some(5));
        assert_eq!(find_kth_largest(&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4), Some(4));
        assert_eq!(find_kth_largest(&[1], 2), None);
        assert_eq!(find_kth_largest(&[1], 0), None);
    }
}
