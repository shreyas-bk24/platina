use std::f64::consts::PI;
use std::mem::size_of;
use std::num::NonZeroU32;

enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
    Empty,
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle { r } => PI * r * r,
        Shape::Rect { w, h } => w * h,
        Shape::Empty => 0.0,
    }
}

fn first_even(v: &[u32]) -> Option<u32> {
    let Some(&x) = v.iter().find(|&&x| x % 2 == 0) else { return None };
    Some(x)
}

fn main() {
    let shapes = [Shape::Circle { r: 1.0 }, Shape::Rect { w: 2.0, h: 3.0 }, Shape::Empty];
    let total: f64 = shapes.iter().map(area).sum();
    let boxed: Option<Box<i32>> = Some(Box::new(7));
    let none: Option<Box<i32>> = None;
    if let Some(b) = &boxed {
        println!("{b} {}", none.is_none());
    }
    println!("{total:.2} {:?}", first_even(&[3, 5, 8]));
    println!("{} {} {} {} {}", size_of::<Shape>(), size_of::<Option<u32>>(),
        size_of::<Option<NonZeroU32>>(), size_of::<Box<i32>>(), size_of::<Option<Box<i32>>>());
}
