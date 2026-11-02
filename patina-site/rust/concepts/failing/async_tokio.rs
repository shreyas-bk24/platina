use std::rc::Rc;

#[tokio::main]
async fn main() {
    let data = Rc::new(5);
    tokio::spawn(async move {
        tokio::task::yield_now().await;
        println!("{data}"); // Rc held across an .await in a Send task
    });
}
