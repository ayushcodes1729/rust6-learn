// print fibonacci at an index 0, 1, 1, 2, 3, 5, 8, 13, ....

fn main() {
    println!("Number in fibonacci series at index 3 is {}",fib(3));
}

fn fib(number: i32)-> i32 {
        if number == 0 {
            return 0
        }
        else if number == 1 {
            return 1
        }
        else {
            return fib(number-1) + fib(number-2)
        }
}