// fn main() {
//     let nums = vec![1, 2, 3, 5, 7];
//     let nums_iter = nums.iter(); //nums.iter() let's nums_iter to borrow the values inside nums vector.
//     for values in nums_iter {
//         println!("{}", values);
//     }
//     // here we can again get nums vector 
//     println!("{:?}", nums);
// }



// use std::collections::HashMap;

// fn main() {
//     let mut users: HashMap<String, u32> = HashMap::new();
//     users.insert(String::from("Ayush"), 21);
//     users.insert(String::from("Harkirat"), 24);

//     let admin = users.get("Ayush");
//     let age = match admin {
//         Some(age) => age,
//         None => {
//             println!("Enter a valid user");
//             &0
//         },
//     };
//     if *age > 0 {
//         println!("User age: {}", age);
//     }
// }

// // Return a vector with only even values

// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     println!("{:?}", get_even(&mut vec));
// }

// fn get_even(vector: &Vec<i32>) -> Vec<i32> {
//     let mut vec = Vec::new();
//     for val in vector {
//         if val%2 == 0 {
//             vec.push(*val);
//         }
//         else {
//             continue;
//         }
//     }
//     return vec;
// }

// Borrowing

// fn main() {
//     let name = String::from("Ayush");
//     let s1 = &name;
//     println!("{}",name);
//     println!("{}", s1); // It gives the output Ayush but not a number because it doesn't stores the address of the string Ayush but it borrows it
// }

// // Reading a file 

// use std::fs;

// fn main() {
//     let file_content = match fs::read_to_string("./helfdlo.txt") {
//         Ok(content) => content,
//         Err(_) => {
//             println!("Can't find the string");
//             return;
//         }
//     };

//     println!("File content: {}", file_content);
// }


// // Area of a shape square, circle, rectangle

// enum Shapes {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64)
// }

// fn calculate_area(shape: Shapes)-> f64 {
//     match shape {
//         Shapes::Square(s)=> s*s,
//         Shapes::Circle(r)=> 3.14*r*r,
//         Shapes::Rectangle(l,b ) => l*b
//     }
// }

// fn main() {
//     let rect = Shapes::Rectangle(2.0, 3.0);
//     let area = calculate_area(rect);
//     println!("Area of the shape is {}", area);
// }

// // returns the length of a string

// fn main() {
//     println!("Lenght of the string Ayush is {}", get_str_length("Ayush@$#R$Ri"));
// }

// fn get_str_length(input: &str)-> usize {
//     return input.chars().count();
// }

// // print fibonacci at an index 0, 1, 1, 2, 3, 5, 8, 13, ....

// fn main() {
//     println!("Number in fibonacci series at index 3 is {}",fib(3));
// }

// fn fib(number: i32)-> i32 {
//         if number == 0 {
//             return 0
//         }
//         else if number == 1 {
//             return 1
//         }
//         else {
//             return fib(number-1) + fib(number-2)
//         }
// }

// use std::io;
// // fn to check even or odd
// fn main() {
//     println!("Enter a number: ");
//     let mut number = String::new();
//     io::stdin().read_line(&mut number).expect("Error in reading number");
//     let number: i32 = match number.trim().parse(){
//         Ok(num) => num,
//         Err(_) => {
//             println!("Enter a number");
//             return;
//         },
//     };

//     is_even(number);
// }

// fn is_even(number: i32) {
//     if number % 2 == 0 {
//         println!("{} is an even number", number);
//     }
//     else {
//         println!("{} is an odd number", number);
//     }
// }




// use rand::{Rng, rng};

// fn main() {
//     let mut rng = rng();
//     let n: u32 = rng.random();
//     println!("Random number: {}", n);
// }

// // Option for null

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, character) in s.chars().enumerate() {
//         if character == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// fn main() {
//     let my_string = String::from("raman");
//     match find_first_a(my_string) {
//         Some(index) => println!("The letter 'a' is found at index: {}", index),
//         None => println!("The letter 'a' is not found in the string."),
//     }
// }

// // Error Handling in Rust

// use std::fs;

// fn main() {
//     let res = fs::read_to_string("ex.txt");
//     match res {
//         Ok(content) => {
//             println!("This is the file content: {}", content);
//         }
//         Err(content) => {
//             println!("This is an error: {}", content);
//         }
//     }

//     println!("Hi there!")
// }


// // Pattern matching and enums with value
// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64,f64)
// }

// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(rad) => 3.14* rad* rad,
//         Shape::Square(side) => side* side,
//         Shape::Rectangle(len,brd ) => len* brd
//     }
// }

// fn main() {
//     let ans = calculate_area(Shape::Square(2.2));
//     println!("{}",ans);
// }

// // enums in rust
// #[derive(Debug)]
// enum Direction {
//     North,
//     South,
//     West,
//     East
// }

// fn main() {
//     let result = printDir(Direction::North);
//     println!("{:?}", result);
// }

// fn printDir(dir : Direction)-> Direction{
//     return dir;
// }

// struct User {
//     name: String,
//     age: u32,
//     active: bool
// }

// impl User {
//     fn tell_name_age(&self){
//         println!("Name is {} and age is {}", self.name, self.age );
//     }
// }

// fn main() {
//     let user1 = User{
//         name: String::from("Ayush Kumar"),
//         age: 21,
//         active: true
//     };
//     user1.tell_name_age();
//     // println!("{} is {} and of age {}", user1.name, user1.active, user1.age); 
// }

// At any given time a variable can have only 1 mutable references or multiple immutable references
// // Passing a mutable reference to a function
// fn main() {
//     let mut s1 = String::from("Hey Guys!");
//     takes_ownership(&mut s1);
//     // not good 
//     let s3 = &s1;
//     let s4 = &s1;
//     println!("{}", s1);
//     println!("{}", s3);
// }

// fn takes_ownership(sent: &mut String) {
//     sent.push_str(" I am Ayush");
// }

// fn main() {
//     // let sentence = String::from("Hey I am Ayush");
//     // get_first_word(sentence);
//     let s1 = String::from("Hello");
//     // let s3 = "Hellow";
//     let s2 = &s1; //borrowing a variable by s2, or passing a reference to s1
//     // let num: i8 = 12;
//     println!("{}", s1);
//     println!("{}", s2);
// }




// fn get_first_word(sent: String)-> () {
//     let mut ans = String::from("");
//     for char in sent.chars() {
//         if char == ' ' {
//             break;
//         }
//         ans.push_str(char.to_string().as_str());

//     }
//     println!("First word is {}", ans);
// }
