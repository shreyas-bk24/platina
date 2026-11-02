use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    other: RefCell<Option<Rc<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("dropped");
    }
}

fn main() {
    let a = Rc::new(Node { other: RefCell::new(None) });
    let b = Rc::new(Node { other: RefCell::new(Some(Rc::clone(&a))) });
    *a.other.borrow_mut() = Some(Rc::clone(&b)); // a -> b -> a
    println!("end of main: a strong = {}, b strong = {}", Rc::strong_count(&a), Rc::strong_count(&b));
}
