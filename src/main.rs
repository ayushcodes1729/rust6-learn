fn main() {
    let nums = vec![1, 2, 3, 5, 7];
    let nums_iter = nums.iter(); 

    let mut iter2 = nums_iter.map(|x| x+1); // this is a iterator adaptor

    while let Some(val) = iter2.next() {
        println!("{}", val);
    }
    println!("{:?}", nums);
}
