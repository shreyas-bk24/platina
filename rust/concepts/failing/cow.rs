use std::borrow::Cow;

fn shout(input: &str) -> Cow<'_, str> {
    let upper = input.to_uppercase();
    Cow::Borrowed(&upper) // upper is dropped when shout returns
}

fn main() {
    println!("{}", shout("hi"));
}
