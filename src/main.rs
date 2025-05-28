// Lifetimes

fn main() {
    let ans;
    let s1 = String::from("Ayushaaaaaa");
    {
        let s2 = String::from("Harkirat");
        ans = get_long_string(&s1, &s2); // Introduction of lifetime generic Annotation gives us better error here i.e. s2 doesn't lives long enough
    }
    println!("{}",ans);
}

// 'a is the lifetime generic Annotation below it eshtablishes a relationship between the lifetime of the parameters and the return value. The lifetime of the return value will be the shorter(intersection) of lifetime of both parameters
fn get_long_string<'a>(s1: &'a str, s2: &'a str)-> &'a str {
    if s1.chars().count() > s2.chars().count() {
        return s1;
    }
    else {
        return s2;
    }
}