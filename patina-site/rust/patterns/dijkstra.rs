use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(n: usize, edges: &[(usize, usize, u64)], src: usize) -> Vec<Option<u64>> {
    let mut adj = vec![Vec::new(); n];
    for &(a, b, w) in edges {
        adj[a].push((b, w));
        adj[b].push((a, w));
    }
    let mut dist = vec![None; n];
    let mut heap = BinaryHeap::from([Reverse((0, src))]);
    while let Some(Reverse((d, u))) = heap.pop() {
        if dist[u].is_some() {
            continue;
        }
        dist[u] = Some(d);
        for &(v, w) in &adj[u] {
            if dist[v].is_none() {
                heap.push(Reverse((d + w, v)));
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        let e = [(0, 1, 7), (0, 2, 9), (0, 5, 14), (1, 2, 10), (1, 3, 15), (2, 3, 11), (2, 5, 2), (3, 4, 6), (4, 5, 9)];
        assert_eq!(dijkstra(6, &e, 0), vec![Some(0), Some(7), Some(9), Some(20), Some(20), Some(11)]);
        assert_eq!(dijkstra(3, &[(0, 1, 1)], 0), vec![Some(0), Some(1), None]);
    }
}
