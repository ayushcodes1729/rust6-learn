// Traits

// Defining a trait (Abstract way). We can also make a default implementation here by defining the function here in the trait def also
pub trait Call {
    fn call_me(&self)-> String;
}

struct User {
    name: String,
    age: i32
}

// Implementing Call trait for User struct
impl Call for User {
    fn call_me(&self)-> String {
        return format!("User name is {} and age is {}", self.name, self.age);
    }
}
fn main() {
    let user = User{
        name: String::from("Ayush"),
        age: 21
    };
    let result =user.call_me();
    println!("{}", result);
}