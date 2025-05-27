fn main() {
    let nums = vec![1, 2, 3, 5, 7];
    let mut nums_iter = nums.into_iter(); // wen need to make nums_iter mutable because we are using .next()which splits an iterator into its element
    // into_iter() creates an iterator by taking the ownership from the vector but not by borrowing it.

    while let Some(val) = nums_iter.next(){
        println!("{}", val);
    }
    // println!("{:?}", nums);
}
