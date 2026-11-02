pub fn daily_temperatures(temps: &[i32]) -> Vec<usize> {
    let mut answer = vec![0; temps.len()];
    let mut stack: Vec<usize> = Vec::new();
    for (i, &t) in temps.iter().enumerate() {
        while let Some(&top) = stack.last() {
            if temps[top] >= t {
                break;
            }
            stack.pop();
            answer[top] = i - top;
        }
        stack.push(i);
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(daily_temperatures(&[73, 74, 75, 71, 69, 72, 76, 73]), vec![1, 1, 4, 2, 1, 1, 0, 0]);
        assert_eq!(daily_temperatures(&[30, 40, 50, 60]), vec![1, 1, 1, 0]);
        assert_eq!(daily_temperatures(&[]), Vec::<usize>::new());
    }
}
