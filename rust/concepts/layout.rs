use std::marker::PhantomData;
use std::mem::{align_of, size_of};

struct Mixed { a: u8, b: u32, c: u16 }

#[repr(C)]
struct MixedC { a: u8, b: u32, c: u16 }

struct Meters;

struct Id<T> { raw: u64, _kind: PhantomData<T> }
struct User;

fn main() {
    let m = Mixed { a: 1, b: 2, c: 3 };
    let mc = MixedC { a: 1, b: 2, c: 3 };
    println!("{} {} {}", size_of::<Mixed>(), align_of::<Mixed>(), m.a as u32 + m.b + m.c as u32);
    println!("{} {} {}", size_of::<MixedC>(), align_of::<MixedC>(), mc.a as u32 + mc.b + mc.c as u32);
    let ids: Vec<Meters> = (0..1000).map(|_| Meters).collect();
    let uid: Id<User> = Id { raw: 7, _kind: PhantomData };
    println!("{} {} {} {}", size_of::<Meters>(), ids.len(), size_of::<Id<User>>(), uid.raw);
}
