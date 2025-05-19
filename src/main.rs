// Area of a shape square, circle, rectangle

enum Shapes {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}

fn calculate_area(shape: Shapes)-> f64 {
    match shape {
        Shapes::Square(s)=> s*s,
        Shapes::Circle(r)=> 3.14*r*r,
        Shapes::Rectangle(l,b ) => l*b
    }
}

fn main() {
    let rect = Shapes::Rectangle(2.0, 3.0);
    let area = calculate_area(rect);
    println!("Area of the shape is {}", area);
}