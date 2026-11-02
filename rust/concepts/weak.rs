use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    name: &'static str,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node { name: "leaf", parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]) });
    {
        let branch = Rc::new(Node { name: "branch", parent: RefCell::new(Weak::new()), children: RefCell::new(vec![Rc::clone(&leaf)]) });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        let p = leaf.parent.borrow().upgrade().map(|p| p.name);
        println!("{p:?} {} {} {}", Rc::strong_count(&branch), Rc::weak_count(&branch), branch.children.borrow().len());
    }
    println!("{:?} {}", leaf.parent.borrow().upgrade().map(|p| p.name), Rc::strong_count(&leaf));
}
