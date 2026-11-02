struct Id<T> {
    raw: u64, // T is never used
}

fn main() {
    let id: Id<String> = Id { raw: 7 };
    println!("{}", id.raw);
}
