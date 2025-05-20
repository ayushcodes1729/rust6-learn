fn main() {
    let name = String::from("Ayush");
    let s1 = &name;
    println!("{}",name);
    println!("{}", s1); // It gives the output Ayush but not a number because it doesn't stores the address of the string Ayush but it borrows it
}