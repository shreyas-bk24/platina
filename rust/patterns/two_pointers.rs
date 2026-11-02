pub fn two_sum(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    let (mut l, mut r) = (0, numbers.len().checked_sub(1)?);
    while l < r {
        let sum = numbers[l] + numbers[r];
        if sum == target {
            return Some((l, r));
        } else if sum < target {
            l += 1;
        } else {
            r -= 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(two_sum(&[1, 3, 4, 6, 8, 11, 15], 14), Some((1, 5)));
        assert_eq!(two_sum(&[1, 2], 10), None);
        assert_eq!(two_sum(&[], 1), None);
    }
}
