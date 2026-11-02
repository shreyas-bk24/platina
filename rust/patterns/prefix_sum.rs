use std::collections::HashMap;

pub fn subarray_sum(nums: &[i32], k: i32) -> usize {
    let mut seen: HashMap<i32, usize> = HashMap::from([(0, 1)]);
    let (mut prefix, mut count) = (0, 0);
    for &x in nums {
        prefix += x;
        if let Some(&c) = seen.get(&(prefix - k)) {
            count += c;
        }
        *seen.entry(prefix).or_insert(0) += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(subarray_sum(&[1, 1, 1], 2), 2);
        assert_eq!(subarray_sum(&[1, 2, 3], 3), 2);
        assert_eq!(subarray_sum(&[3, 4, 7, 2, -3, 1, 4, 2], 7), 4);
        assert_eq!(subarray_sum(&[], 0), 0);
    }
}
