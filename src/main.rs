// Traits

// Defining a trait (Abstract way). We can also make a default implementation here by defining the function here in the trait def also
pub trait Call {
    fn call_me(&self)-> String{
        return "Hi there".to_string();
    }
}
pub trait Call2 {
    fn call_us(&self)-> String{
        return "Hi there".to_string();
    }
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
impl Call2 for User{}
fn main() {
    let user = User{
        name: String::from("Ayush"),
        age: 21
    };
    let result =user.call_me();
    println!("{}", result);
    println!("{}", notify(&user)); // passing by reference is not necessary but if you passs by reference then you can access user.name in the next line
    print!("{}", user.name);
}

// Trait as an argument: just a syntax sugar to write that a particular struct is bound to some trait. Eg. pub fn notify<T: Call>(item: &T)-> String and if we want to implement multiple trait then pub fn notify<T: Call + Call2>(item: &T)-> String
pub fn notify(item: &impl Call)-> String {
    return "Hey this is a notification".to_string();
}