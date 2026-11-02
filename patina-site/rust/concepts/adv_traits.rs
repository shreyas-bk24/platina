use std::fmt::Display;

trait Container {
    type Item;
    fn get(&self, i: usize) -> Option<&Self::Item>;
    fn first(&self) -> Option<&Self::Item> {
        self.get(0)
    }
}

struct Stack { items: Vec<i32> }

impl Container for Stack {
    type Item = i32;
    fn get(&self, i: usize) -> Option<&i32> { self.items.get(i) }
}

trait Describe { fn describe(&self) -> String; }

impl<T: Display> Describe for T {
    fn describe(&self) -> String { format!("<{self}>") }
}

fn largest(items: impl Iterator<Item = i32>) -> Option<i32> { items.max() }

fn evens(limit: u32) -> impl Iterator<Item = u32> { (0..limit).filter(|n| n % 2 == 0) }

fn main() {
    let s = Stack { items: vec![4, 9, 2] };
    let boxed: Box<dyn Container<Item = i32>> = Box::new(Stack { items: vec![1] });
    println!("{:?} {:?} {} {}", s.first(), boxed.first(), 5.describe(), "hi".describe());
    println!("{:?} {:?}", largest(s.items.into_iter()), evens(7).collect::<Vec<_>>());
}
