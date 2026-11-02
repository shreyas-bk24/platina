fn sum(v: &[i32]) -> i32 {
    v.iter().sum()
}

fn main() {
    let mut nums = vec![1, 2, 3];
    let a = &nums;
    let b = &nums;
    println!("{} {}", a.len(), sum(b));
    let m = &mut nums;
    m.push(4);
    println!("{nums:?}");
}
