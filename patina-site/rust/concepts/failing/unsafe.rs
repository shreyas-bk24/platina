fn main() {
    let x = 7;
    let p = &x as *const i32;
    println!("{}", *p); // dereferencing a raw pointer needs unsafe
}
