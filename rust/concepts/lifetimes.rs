fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let outer = String::from("long string");
    let result;
    {
        let inner = String::from("xyz");
        result = longest(outer.as_str(), inner.as_str());
        println!("{result}");
    }
    println!("{outer}");
}
