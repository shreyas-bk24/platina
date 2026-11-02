use std::thread;

fn main() {
    let mut hits = 0;
    thread::scope(|s| {
        s.spawn(|| hits += 1);
        s.spawn(|| hits += 1); // two threads mutating a plain integer
    });
    println!("{hits}");
}
