#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }

    fn contains(&self, p: &Point) -> bool {
        p.x >= self.top_left.x && p.x <= self.bottom_right.x &&
        p.y <= self.top_left.y && p.y >= self.bottom_right.y
    }
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 10.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };
    let p1 = Point { x: 5.0, y: 5.0 };
    let p2 = Point { x: 15.0, y: 5.0 };

    println!("Rectangle area: {}", rect.area());
    println!("Contains p1? {}", rect.contains(&p1));
    println!("Contains p2? {}", rect.contains(&p2));
}
