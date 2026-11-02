use std::sync::{Condvar, Mutex, RwLock};
use std::thread;

fn main() {
    let config = RwLock::new(String::from("v1"));
    {
        let a = config.read().unwrap();
        let b = config.read().unwrap();
        println!("{a} {b}");
    }
    config.write().unwrap().push_str("+v2");

    let started = Mutex::new(false);
    let cv = Condvar::new();
    thread::scope(|s| {
        s.spawn(|| {
            *started.lock().unwrap() = true;
            cv.notify_one();
        });
        let mut ok = started.lock().unwrap();
        while !*ok {
            ok = cv.wait(ok).unwrap();
        }
    });

    let (a, b) = (Mutex::new(1), Mutex::new(2));
    thread::scope(|s| {
        s.spawn(|| { let x = a.lock().unwrap(); let y = b.lock().unwrap(); *x + *y });
        s.spawn(|| { let x = a.lock().unwrap(); let y = b.lock().unwrap(); *x * *y });
    });
    println!("{}", config.read().unwrap());
}
