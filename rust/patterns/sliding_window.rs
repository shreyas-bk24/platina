use std::collections::HashMap;

pub fn length_of_longest_substring(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut last: HashMap<char, usize> = HashMap::new();
    let (mut left, mut best) = (0, 0);
    for (right, &c) in chars.iter().enumerate() {
        if let Some(&prev) = last.get(&c) {
            left = left.max(prev + 1);
        }
        last.insert(c, right);
        best = best.max(right - left + 1);
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(length_of_longest_substring("bbbbb"), 1);
        assert_eq!(length_of_longest_substring("pwwkew"), 3);
        assert_eq!(length_of_longest_substring(""), 0);
        assert_eq!(length_of_longest_substring("abba"), 2);
    }
}
