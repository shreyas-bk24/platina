fn port(s: &str) -> Result<u16, String> {
    let n = s.parse::<u16>()?; // no From<ParseIntError> for String
    Ok(n)
}

fn main() {
    println!("{:?}", port("80"));
}
