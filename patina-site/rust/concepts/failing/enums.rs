enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
    Empty,
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle { r } => 3.14 * r * r,
        Shape::Rect { w, h } => w * h,
        // forgot Shape::Empty
    }
}

fn main() {
    println!("{}", area(&Shape::Empty));
}
