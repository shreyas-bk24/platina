use std::mem;

struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let _a = Guard("a");
    {
        let _b = Guard("b");
    }
    let c = Guard("c");
    drop(c);
    let mut name = String::from("old");
    let taken = mem::take(&mut name);
    let prev = mem::replace(&mut name, String::from("new"));
    let mut x = String::from("x");
    mem::swap(&mut name, &mut x);
    println!("{taken} [{prev}] {name} {x}");
}
