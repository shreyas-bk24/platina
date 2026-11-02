use std::collections::VecDeque;

/// prereqs holds (course, prerequisite) pairs.
pub fn find_order(n: usize, prereqs: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    let mut indegree = vec![0; n];
    for &(course, pre) in prereqs {
        adj[pre].push(course);
        indegree[course] += 1;
    }
    let mut queue: VecDeque<usize> = (0..n).filter(|&c| indegree[c] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(c) = queue.pop_front() {
        order.push(c);
        for &next in &adj[c] {
            indegree[next] -= 1;
            if indegree[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    (order.len() == n).then_some(order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(find_order(4, &[(1, 0), (2, 0), (3, 1), (3, 2)]), Some(vec![0, 1, 2, 3]));
        assert_eq!(find_order(2, &[(1, 0), (0, 1)]), None);
        assert_eq!(find_order(1, &[]), Some(vec![0]));
    }
}
