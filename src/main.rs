// WAP to get the first name of your name froma  string

fn main() {
    let name = String::from("Ayush Kumar");
    let result_word = get_first_word(&name);
    println!("{}", name);
    println!("{}", result_word);
    // Using String
    // let mut answer = String::from("");
    // for i in name.chars() {
    //     if i==' ' {
    //         break;
    //     }
    //     answer.push_str(&(i.to_string()));
    // }

    // println!("{}",answer);
}

// By using slices
fn get_first_word(word: &String) -> &str {
    let mut index = 0;
    for i in word.chars() {
        if i==' ' {
            break;
        }
        index+=1;
    }

    return &word[0..index];
}