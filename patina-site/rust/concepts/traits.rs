use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
}

struct Circle { r: f64 }
struct Square { side: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.r * self.r }
}
impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
}

fn area_of<T: Shape>(s: &T) -> f64 {
    s.area()
}

fn total(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let c = Circle { r: 1.0 };
    let sq = Square { side: 2.0 };
    let one = area_of(&c) + area_of(&sq);
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(c), Box::new(sq)];
    println!("{one:.2} {:.2}", total(&shapes));
}
