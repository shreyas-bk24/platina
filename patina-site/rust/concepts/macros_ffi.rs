use std::ffi::{c_char, CStr, CString};
use std::mem::size_of;

macro_rules! square_all {
    ($($x:expr),* $(,)?) => { [$($x * $x),*] };
}

fn describe<T: ?Sized + std::fmt::Debug>(v: &T) -> String { format!("{v:?}") }

extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 { x + 1 }

fn main() {
    let sq = square_all!(1, 2, 3);
    let s: &str = "dyn";
    let slice: &[i32] = &sq;
    let boxed: Box<str> = "boxed".into();
    let c = CString::new("hello").unwrap();
    let n = unsafe { strlen(c.as_ptr()) };
    let back = unsafe { CStr::from_ptr(c.as_ptr()) }.to_str().unwrap();
    println!("{sq:?} {} {} {} {n} {back} {} {}", describe(s), describe(slice), boxed.len(), add_one(41), size_of::<&str>());
}
