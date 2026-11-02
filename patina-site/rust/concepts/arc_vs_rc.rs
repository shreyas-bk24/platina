use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    let rc = Rc::new(vec![1, 2, 3]);
    let rc2 = Rc::clone(&rc);
    let arc = Arc::new(vec![1, 2, 3]);
    let handles: Vec<_> = (0..2).map(|i| {
        let a = Arc::clone(&arc);
        thread::spawn(move || a[i])
    }).collect();
    let sum: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("{} {} {} {}", Rc::strong_count(&rc2), Arc::strong_count(&arc), sum, rc.len());
}
