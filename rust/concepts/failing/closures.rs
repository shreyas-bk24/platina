fn main() {
    let name = String::from("ferris");
    let greet = move || name + "!";
    println!("{}", greet());
    println!("{}", greet()); // the first call gave name away
}
