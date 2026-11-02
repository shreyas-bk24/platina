/// Requires nums.len() >= 2 and every value in 1..nums.len().
pub fn find_duplicate(nums: &[usize]) -> usize {
    let (mut slow, mut fast) = (nums[0], nums[nums[0]]);
    while slow != fast {
        slow = nums[slow];
        fast = nums[nums[fast]];
    }
    slow = 0;
    while slow != fast {
        slow = nums[slow];
        fast = nums[fast];
    }
    slow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(find_duplicate(&[1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(&[3, 1, 3, 4, 2]), 3);
        assert_eq!(find_duplicate(&[1, 1]), 1);
    }
}
