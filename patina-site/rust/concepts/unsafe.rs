use std::slice;

fn split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = v.len();
    let ptr = v.as_mut_ptr();
    assert!(mid <= len);
    // SAFETY: [0, mid) and [mid, len) are inside v and do not overlap.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    let (a, b) = split_at_mut(&mut nums, 2);
    a[0] = 10;
    b[0] = 30;
    let x = 7;
    let p = &x as *const i32;
    let y = unsafe { *p };
    println!("{nums:?} {y}");
}
