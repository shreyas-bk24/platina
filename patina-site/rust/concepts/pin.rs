use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

async fn add_later(x: u32) -> u32 {
    let buf = [x, x + 1];
    let r = &buf[1];
    YieldOnce(false).await;
    *r + buf[0]
}

struct YieldOnce(bool);

impl Future for YieldOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            return Poll::Ready(());
        }
        self.0 = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

struct NoopWaker;
impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {}
}

fn main() {
    let mut fut = Box::pin(add_later(20));
    let waker = Waker::from(Arc::new(NoopWaker));
    let mut cx = Context::from_waker(&waker);
    let first = fut.as_mut().poll(&mut cx);
    let second = fut.as_mut().poll(&mut cx);
    println!("{first:?} {second:?}");
}
