use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

struct Job { id: u32, data: Vec<u8> }

fn main() {
    assert_send::<Job>();
    assert_sync::<Job>();
    assert_send::<Arc<Mutex<Vec<u8>>>>();
    assert_sync::<Arc<Mutex<Vec<u8>>>>();
    assert_sync::<Mutex<RefCell<u8>>>();
    let _local = Rc::new(RefCell::new(0));
    let job = Job { id: 1, data: vec![1, 2, 3] };
    let handle = thread::spawn(move || job.id as usize + job.data.len());
    println!("{}", handle.join().unwrap());
}
