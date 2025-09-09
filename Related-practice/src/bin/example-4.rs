// Structs, Impl, Associated functions and methods

struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

fn main() {
    // create instances of type Rectangle
    let rect1 = Rectangle::new(10, 20);
    let rect2 = Rectangle { width: 5, height: 5 };

    println!("rect1 area = {}", rect1.area()); // method on an instance
    println!("rect2 area = {}", rect2.area());
}
