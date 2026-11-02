use std::cell::Cell;

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<Cell<u8>>(); // Cell mutates through &self with no lock
}
