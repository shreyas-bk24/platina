use std::collections::VecDeque;

pub fn num_islands(grid: &mut [Vec<char>]) -> usize {
    let (rows, cols) = (grid.len(), grid.first().map_or(0, |r| r.len()));
    let mut islands = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '1' {
                continue;
            }
            islands += 1;
            grid[r][c] = '0';
            let mut queue = VecDeque::from([(r, c)]);
            while let Some((cr, cc)) = queue.pop_front() {
                for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let (nr, nc) = (cr as isize + dr, cc as isize + dc);
                    if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == '1' {
                        grid[nr][nc] = '0';
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
    }
    islands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        let mut g: Vec<Vec<char>> = ["11000", "11010", "00100", "00011", "10011"].iter().map(|r| r.chars().collect()).collect();
        assert_eq!(num_islands(&mut g), 5);
        let mut g: Vec<Vec<char>> = ["111", "010", "111"].iter().map(|r| r.chars().collect()).collect();
        assert_eq!(num_islands(&mut g), 1);
        let mut g: Vec<Vec<char>> = Vec::new();
        assert_eq!(num_islands(&mut g), 0);
    }
}
