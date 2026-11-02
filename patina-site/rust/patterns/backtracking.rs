pub fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    fn backtrack(nums: &[i32], start: usize, path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        out.push(path.clone());
        for (i, &x) in nums.iter().enumerate().skip(start) {
            path.push(x);
            backtrack(nums, i + 1, path, out);
            path.pop();
        }
    }
    let mut out = Vec::new();
    backtrack(nums, 0, &mut Vec::new(), &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(subsets(&[1, 2, 3]), vec![vec![], vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 3], vec![2], vec![2, 3], vec![3]]);
        assert_eq!(subsets(&[]), vec![Vec::<i32>::new()]);
    }
}
