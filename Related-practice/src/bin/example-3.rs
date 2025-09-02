#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (no `self`)
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
#[allow(unused_variables)]
fn main() {
    // Called with ::, not dot syntax
    let rect = Rectangle::new(10, 20);
}
