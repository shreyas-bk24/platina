pub fn has_cycle(n: usize, edges: &[(usize, usize)]) -> bool {
    fn dfs(u: usize, adj: &[Vec<usize>], color: &mut [u8]) -> bool {
        color[u] = 1;
        for &v in &adj[u] {
            if color[v] == 1 {
                return true;
            }
            if color[v] == 0 && dfs(v, adj, color) {
                return true;
            }
        }
        color[u] = 2;
        false
    }
    let mut adj = vec![Vec::new(); n];
    for &(a, b) in edges {
        adj[a].push(b);
    }
    let mut color = vec![0u8; n];
    (0..n).any(|u| color[u] == 0 && dfs(u, &adj, &mut color))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert!(has_cycle(6, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 2), (0, 5)]));
        assert!(!has_cycle(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]));
        assert!(has_cycle(1, &[(0, 0)]));
        assert!(!has_cycle(0, &[]));
    }
}
