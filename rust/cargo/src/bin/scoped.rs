use rayon::prelude::*;
use std::thread;

fn main() {
    let mut data = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = data.split_at_mut(3);
    thread::scope(|s| {
        s.spawn(|| left.iter_mut().for_each(|x| *x *= 10));
        s.spawn(|| right.iter_mut().for_each(|x| *x *= 100));
    });
    let sum: i64 = data.par_iter().map(|&x| i64::from(x)).sum();
    println!("{data:?} {sum}");
}
