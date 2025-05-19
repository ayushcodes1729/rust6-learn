// returns the length of a string

fn main() {
    println!("Lenght of the string Ayush is {}", get_str_length("Ayush@$#R$Ri"));
}

fn get_str_length(input: &str)-> usize {
    return input.chars().count();
}