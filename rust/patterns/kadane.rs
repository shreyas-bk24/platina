pub fn max_sub_array(nums: &[i32]) -> Option<i32> {
    let (&first, rest) = nums.split_first()?;
    let (mut cur, mut best) = (first, first);
    for &x in rest {
        cur = x.max(cur + x);
        best = best.max(cur);
    }
    Some(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(max_sub_array(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), Some(6));
        assert_eq!(max_sub_array(&[-3, -1, -2]), Some(-1));
        assert_eq!(max_sub_array(&[]), None);
    }
}
