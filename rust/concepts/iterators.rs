struct Countdown { n: u32 }

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.n == 0 {
            return None;
        }
        self.n -= 1;
        Some(self.n + 1)
    }
}

fn main() {
    let total: u32 = Countdown { n: 3 }.map(|x| x * 10).sum();
    let mut v = vec![String::from("a"), String::from("b")];
    for s in &v { print!("{s} "); }
    for s in &mut v { s.push('!'); }
    for s in v { print!("{s} "); }
    println!("{total}");
}
