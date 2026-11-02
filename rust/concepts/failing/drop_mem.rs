struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let mut g = Guard("g");
    g.drop(); // would run again at the end of main: double drop
}
