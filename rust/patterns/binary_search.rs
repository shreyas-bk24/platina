pub fn search_insert(nums: &[i32], target: i32) -> usize {
    let (mut lo, mut hi) = (0, nums.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(search_insert(&[1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(&[1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(&[1, 3, 5, 6], 7), 4);
        assert_eq!(search_insert(&[], 3), 0);
    }
}
