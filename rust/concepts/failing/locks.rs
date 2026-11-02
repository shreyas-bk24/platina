use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let (a, b) = (Mutex::new(1), Mutex::new(2));
    thread::scope(|s| {
        s.spawn(|| {
            let _x = a.lock().unwrap();
            thread::sleep(Duration::from_millis(50));
            let _y = b.lock().unwrap(); // waits for thread 2
        });
        s.spawn(|| {
            let _y = b.lock().unwrap();
            thread::sleep(Duration::from_millis(50));
            let _x = a.lock().unwrap(); // waits for thread 1: deadlock
        });
    });
    println!("never printed");
}
