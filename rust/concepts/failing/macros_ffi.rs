fn show(s: str) {
    println!("{}", &s);
}

fn main() {
    show(*"hi"); // str has no size known at compile time
}
