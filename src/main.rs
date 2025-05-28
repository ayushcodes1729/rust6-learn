// longest string with lifetimes
use std::fmt::Display;


fn main() {
    let s1= String::from("Ayush");
    let s2= String::from("Harkirat");
    let res = longest_with_an_annoucement(&s1, &s2, String::from("Hey this is ann"));
    println!("{}",res)
}

fn longest_with_an_annoucement<'a, T>(str1: &'a str,str2: &'a str, ann: T)-> &'a str where T: Display, {
    println!("Announcement! {}",ann);
    if str1.len() > str2.len() {
        str1
    }else {
        str2
    }
}