// Return a vector with only even values

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    println!("{:?}", get_even(&mut vec));
}

fn get_even(vector: &Vec<i32>) -> Vec<i32> {
    let mut vec = Vec::new();
    for val in vector {
        if val%2 == 0 {
            vec.push(*val);
        }
        else {
            continue;
        }
    }
    return vec;
}