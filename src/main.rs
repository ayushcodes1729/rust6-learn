struct User {
    name: String,
    age: u32,
    active: bool
}

impl User {
    fn tell_name_age(&self){
        println!("Name is {} and age is {}", self.name, self.age );
    }
}

fn main() {
    let user1 = User{
        name: String::from("Ayush Kumar"),
        age: 21,
        active: true
    };
    user1.tell_name_age();
    // println!("{} is {} and of age {}", user1.name, user1.active, user1.age); 
}


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
