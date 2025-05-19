// Reading a file 

use std::fs;

fn main() {
    let file_content = match fs::read_to_string("./helfdlo.txt") {
        Ok(content) => content,
        Err(_) => {
            println!("Can't find the string");
            return;
        }
    };

    println!("File content: {}", file_content);
}