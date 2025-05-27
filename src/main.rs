fn main() {
    let mut nums = vec![1, 2, 3, 5, 7];
    let nums_iter = nums.iter_mut(); //nums.iter() let's nums_iter to borrow the values inside nums vector.
    for values in nums_iter {
        *values = *values + 1
    }
    println!("{:?}", nums);
}
