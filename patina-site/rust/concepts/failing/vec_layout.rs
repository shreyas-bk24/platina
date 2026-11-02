fn main() {
    let mut grid: Vec<Vec<i32>> = vec![vec![1, 2, 3]];
    let row = &grid[0];
    grid.push(vec![4]); // the outer buffer may move under row
    println!("{row:?}");
}
