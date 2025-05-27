fn main() {
    let nums = vec![1, 2, 3, 5, 7];
    let nums_iter = nums.iter(); //nums.iter() let's nums_iter to borrow the values inside nums vector.
    for values in nums_iter {
        println!("{}", values);
    }
    // here we can again get nums vector 
    println!("{:?}", nums);
}
