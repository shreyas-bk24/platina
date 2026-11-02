use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let shared = Arc::new(Mutex::new(0));
    tokio::spawn(async move {
        let guard = shared.lock().unwrap();
        sleep(Duration::from_millis(1)).await; // std MutexGuard held across .await
        println!("{}", *guard);
    });
}
