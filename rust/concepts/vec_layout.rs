fn main() {
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(2);
    grid.push(vec![1, 2, 3]);
    grid.push(vec![4]);
    grid.push(vec![5, 6]);
    grid[1].push(7);
    let flat: Vec<i32> = grid.concat();
    println!("{} {:?} {}", std::mem::size_of::<Vec<i32>>(), flat, flat.len());
}
