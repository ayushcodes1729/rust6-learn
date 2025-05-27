fn main() {
    let nums = vec![1, 2, 3, 5, 7];
    let nums_iter = nums.iter(); 

    let total: i32 = nums_iter.sum(); // A consuming adapter which takes the ownership of an iterator. How do we know that? Since it takes self as a parameter but not &self as a parameter.

    println!("{}", total);
    // print!("{:?}", nums_iter); // We can't use the iterator anymore here because it is consumed by the sum adapter
    println!("{:?}", nums); // We can still us the vector since the adapter consumes the iterator but not the vector
}
