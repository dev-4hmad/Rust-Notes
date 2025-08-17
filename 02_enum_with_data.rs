// Enum that can hold extra data
enum Shape {
    Circle(f64),         // radius
    Rectangle(f64, f64), // width, height
}

fn main() {
    let s1 = Shape::Circle(2.5);
    let s2 = Shape::Rectangle(3.0, 4.0);

    match s1 {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(_, _) => (),
    }

    match s2 {
        Shape::Circle(_) => (),
        Shape::Rectangle(w, h) => println!("Rectangle with width {} and height {}", w, h),
    }
}
