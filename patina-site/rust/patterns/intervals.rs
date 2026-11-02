pub fn merge(mut intervals: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    intervals.sort_unstable_by_key(|iv| iv[0]);
    let mut merged: Vec<[i32; 2]> = Vec::with_capacity(intervals.len());
    for iv in intervals {
        match merged.last_mut() {
            Some(last) if iv[0] <= last[1] => last[1] = last[1].max(iv[1]),
            _ => merged.push(iv),
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(merge(vec![[1, 3], [2, 6], [8, 10], [15, 18]]), vec![[1, 6], [8, 10], [15, 18]]);
        assert_eq!(merge(vec![[1, 4], [4, 5]]), vec![[1, 5]]);
        assert_eq!(merge(vec![[1, 4], [2, 3]]), vec![[1, 4]]);
        assert_eq!(merge(vec![]), Vec::<[i32; 2]>::new());
    }
}
