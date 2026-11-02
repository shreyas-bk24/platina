async fn work() -> u32 {
    1
}

fn main() {
    let mut a = Box::pin(work());
    let mut b = Box::pin(work());
    // swapping would move both futures; that needs Unpin
    std::mem::swap(a.as_mut().get_mut(), b.as_mut().get_mut());
}
