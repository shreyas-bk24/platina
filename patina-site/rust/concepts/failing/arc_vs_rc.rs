use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

fn main() {
    let shared = Arc::new(RefCell::new(0));
    let s = Arc::clone(&shared);
    thread::spawn(move || {
        *s.borrow_mut() += 1; // Arc shares it, but RefCell is not Sync
    });
}
