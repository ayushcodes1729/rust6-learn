// Using lifetimes with structs
struct User<'a> {
    name: &'a str
}

// Why do we need structs with references to have a lifetime parameter? So that we know how long the struct is gonna live. If the borrowed value inside the struct dies some how(i.e. goes out of scope) then the struct should also die, else it will create a dangling pointer error.

fn main() {
    let first_name = String::from("Ayush");
    let user = User {name: &first_name};
    println!("The name of the user is {}", user.name);
}