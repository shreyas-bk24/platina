use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let c = Rc::clone(&counter);
    thread::spawn(move || {
        *c.lock().unwrap() += 1; // Rc's count is not atomic
    });
}
