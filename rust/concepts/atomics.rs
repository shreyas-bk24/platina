use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

fn main() {
    let hits = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| hits.fetch_add(1, Ordering::Relaxed));
        }
    });
    let data = AtomicUsize::new(0);
    let ready = AtomicBool::new(false);
    thread::scope(|s| {
        s.spawn(|| {
            data.store(42, Ordering::Relaxed);
            ready.store(true, Ordering::Release);
        });
        s.spawn(|| {
            while !ready.load(Ordering::Acquire) {
                std::hint::spin_loop();
            }
            assert_eq!(data.load(Ordering::Relaxed), 42);
        });
    });
    let slot = AtomicUsize::new(5);
    let swapped = slot.compare_exchange(5, 9, Ordering::AcqRel, Ordering::Acquire);
    println!("{} {swapped:?} {}", hits.load(Ordering::SeqCst), slot.load(Ordering::SeqCst));
}
