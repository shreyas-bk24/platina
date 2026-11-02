use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (job_tx, mut job_rx) = mpsc::channel::<u32>(2);
    let (done_tx, mut done_rx) = mpsc::channel::<u32>(8);
    let worker = tokio::spawn(async move {
        while let Some(job) = job_rx.recv().await {
            sleep(Duration::from_millis(10)).await;
            done_tx.send(job * job).await.unwrap();
        }
    });
    for job in 1..=4 {
        job_tx.send(job).await.unwrap();
    }
    drop(job_tx);
    worker.await.unwrap();
    while let Some(r) = done_rx.recv().await {
        println!("{r}");
    }
}
