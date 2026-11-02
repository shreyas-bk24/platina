use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(vec![1, 2]);
    let first = cell.borrow_mut();
    let second = cell.borrow_mut(); // compiles, then panics at runtime
    println!("{} {}", first.len(), second.len());
}
