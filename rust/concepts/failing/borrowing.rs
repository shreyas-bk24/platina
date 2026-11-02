fn main() {
    let mut nums = vec![1, 2, 3];
    let first = &nums[0];
    nums.push(4); // may reallocate while first points into the buffer
    println!("{first}");
}
