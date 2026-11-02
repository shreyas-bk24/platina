fn main() {
    let s = String::from("héllo 🦀");
    println!("{} {}", s.len(), s.chars().count());
    let bytes: &[u8] = s.as_bytes();
    let first: &str = &s[0..1];
    let e: Option<char> = s.chars().nth(1);
    let crab = s.char_indices().find(|&(_, c)| c == '🦀').map(|(i, _)| i);
    println!("{first} {e:?} {crab:?} {}", bytes[1]);
    println!("{}", s.is_char_boundary(2));
}
