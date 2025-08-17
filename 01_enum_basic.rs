// Simple enum for colors
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c = Color::Red;

    match c {
        Color::Red => println!("The color is Red"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
    }
}
