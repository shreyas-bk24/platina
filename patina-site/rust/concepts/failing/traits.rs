trait Shape {
    fn area(&self) -> f64;
}

fn area_of<T>(s: &T) -> f64 {
    s.area() // no bound: the compiler cannot assume T has area()
}

fn main() {}
