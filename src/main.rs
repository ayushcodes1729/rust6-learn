// WAP to filter out all the people from a hasmap having age>40 and create a new hashmap which has the remaining people.

use std::collections::HashMap;

fn main() {

    let mut people = HashMap::new();
    people.insert("Harkirat", 28);
    people.insert("Ayush", 21);
    people.insert("Papa", 48);
    people.insert("Mumma", 47);

    let result: HashMap<&str, i32> = people.iter().filter(|x| *x.1 <= 40).map(|(key, val)| (*key, *val)).collect();

    println!("{:?}", result);
}
