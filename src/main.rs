// WAP to first filter out all the odd values from a vector then double it and then cereate a new resultant vector

fn main() {

    let v = vec![1,2,3,4,5,6,7,8,9];
    let result: Vec<i32>= v.iter().filter(|x| *x % 2 != 0).map(|x| x*2).collect(); // The collect method consumes the iterator and converts it into a collection data type(here, vector)

    println!("{:?}", result);


    //The below solution is great but it returns an iterator doubled_val but we need a vector
    // let v = vec![1,2,3,4,5,6,7,8,9];
    // let iter = v.iter();
    // let odd_val = iter.filter(|x| *x % 2 != 0);
    // let doubled_val = odd_val.map(|x| x*2);
    // for val in doubled_val {
    //     println!("{}", val);
    // }
}
