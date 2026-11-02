fn take(s: String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s2.clone();
    let n = 42;
    let m = n;
    let len = take(s3);
    println!("{s2} {n} {m} {len}");
}
