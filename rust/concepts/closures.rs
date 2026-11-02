fn call_twice<F: FnMut() -> u32>(mut f: F) -> u32 {
    f() + f()
}

fn consume<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn main() {
    let base = 10;
    let add = |x: u32| x + base;
    let mut count = 0;
    let total = call_twice(|| { count += 1; count });
    let name = String::from("ferris");
    let greet = move || name + "!";
    let msg = consume(greet);
    let evens: Vec<u32> = (1..=6).filter(|n| n % 2 == 0).map(add).collect();
    println!("{total} {count} {msg} {evens:?}");
}
