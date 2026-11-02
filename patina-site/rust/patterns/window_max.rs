use std::collections::VecDeque;

pub fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut out = Vec::new();
    for (i, &x) in nums.iter().enumerate() {
        if deque.front().is_some_and(|&f| i >= k && f <= i - k) {
            deque.pop_front();
        }
        while deque.back().is_some_and(|&b| nums[b] <= x) {
            deque.pop_back();
        }
        deque.push_back(i);
        if k > 0 && i + 1 >= k {
            out.push(nums[deque[0]]);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
        assert_eq!(max_sliding_window(&[4, 2], 1), vec![4, 2]);
        assert_eq!(max_sliding_window(&[1, 2], 0), Vec::<i32>::new());
    }
}
