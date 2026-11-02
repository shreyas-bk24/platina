use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinSet;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<u32>(4);
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        let _ = tx.send(7).await;
    });
    tokio::select! {
        Some(v) = rx.recv() => println!("got {v}"),
        _ = sleep(Duration::from_millis(10)) => println!("timeout"),
    }
    let sem = Arc::new(Semaphore::new(2));
    let mut set = JoinSet::new();
    for id in 0..4u64 {
        let sem = Arc::clone(&sem);
        set.spawn(async move {
            let _permit = sem.acquire_owned().await.unwrap();
            sleep(Duration::from_millis(5)).await;
            id * 10
        });
    }
    let sum = tokio::task::spawn_blocking(|| (1..=1_000u64).sum::<u64>()).await.unwrap();
    let mut total = 0;
    while let Some(r) = set.join_next().await {
        total += r.unwrap();
    }
    println!("{total} {sum}");
}
