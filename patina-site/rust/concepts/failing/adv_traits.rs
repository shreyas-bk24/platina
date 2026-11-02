use std::fmt;

// Both Display and Vec are defined in std, not in this crate.
impl fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} items", self.len())
    }
}

fn main() {}
