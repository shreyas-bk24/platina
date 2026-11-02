fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

struct Parser<'a> { input: &'a str }

impl<'a> Parser<'a> {
    fn rest(&self) -> &'a str { self.input }
    fn peek(&self) -> &str { &self.input[..1] }
}

fn keep<T: 'static>(t: T) -> T { t }

fn main() {
    let text = String::from("hello world");
    let w = first_word(&text);
    let lit: &'static str = "baked into the binary";
    let owned = keep(String::from("owned"));
    let p = Parser { input: &text };
    let shorter: &str = lit;
    println!("{w} {} {} {} {shorter}", p.rest(), p.peek(), owned.len());
}
