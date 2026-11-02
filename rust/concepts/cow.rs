use std::borrow::Cow;

fn normalize(input: &str) -> Cow<'_, str> {
    if input.contains(' ') {
        Cow::Owned(input.replace(' ', "_"))
    } else {
        Cow::Borrowed(input)
    }
}

fn main() {
    let a = normalize("ferris");
    let b = normalize("hello world");
    let borrowed = [&a, &b].map(|c| matches!(c, Cow::Borrowed(_)));
    println!("{a} {b} {borrowed:?}");
    let owned: String = a.into_owned();
    println!("{owned}");
}
