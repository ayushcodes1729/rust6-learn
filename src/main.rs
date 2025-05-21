use std::collections::{btree_map::Values, HashMap};

fn main() {
    let pairs = vec![
        (String::from("Ayush"), 24),
        (String::from("Harkirat"), 21),
        (String::from("Ayush"), 21),
    ];

    let ghp = hp_by_pairs(pairs);
    println!("Hashmap is {:?}", ghp)

}

fn hp_by_pairs(pairs: Vec<(String, i32)>)-> HashMap<String, i32>{
    let mut hm = HashMap::new();
    for (key, val) in pairs {
        hm.insert(key, val);
    }
    return hm;
}

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