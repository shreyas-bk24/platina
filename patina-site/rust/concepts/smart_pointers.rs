use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    let shared = Rc::new(RefCell::new(vec![1, 2]));
    let other = Rc::clone(&shared);
    other.borrow_mut().push(3);
    println!("{} {:?} {}", b, shared.borrow(), Rc::strong_count(&shared));
    drop(other);
    println!("{}", Rc::strong_count(&shared));
}
