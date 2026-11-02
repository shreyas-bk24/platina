pub fn rob(nums: &[i32]) -> i32 {
    let mut dp = vec![0; nums.len() + 1];
    for (i, &x) in nums.iter().enumerate() {
        let take = x + if i >= 1 { dp[i - 1] } else { 0 };
        dp[i + 1] = dp[i].max(take);
    }
    dp[nums.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(rob(&[1, 2, 3, 1]), 4);
        assert_eq!(rob(&[2, 7, 9, 3, 1]), 12);
        assert_eq!(rob(&[2, 7, 9, 3, 1, 5]), 16);
        assert_eq!(rob(&[]), 0);
    }
}
