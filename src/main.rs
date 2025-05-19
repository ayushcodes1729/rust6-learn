use std::io;
// fn to check even or odd
fn main() {
    println!("Enter a number: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Error in reading number");
    let number: i32 = match number.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Enter a number");
            return;
        },
    };

    is_even(number);
}

fn is_even(number: i32) {
    if number % 2 == 0 {
        println!("{} is an even number", number);
    }
    else {
        println!("{} is an odd number", number);
    }
}