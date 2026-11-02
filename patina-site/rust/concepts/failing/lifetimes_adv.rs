fn assign<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}

fn main() {
    let mut s: &'static str = "static";
    {
        let local = String::from("local");
        assign(&mut s, &local); // &mut T is invariant: 'a must be 'static
    }
    println!("{s}");
}
