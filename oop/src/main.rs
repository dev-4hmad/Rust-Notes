trait Draw {
    fn draw(&self);
}

struct Button { label: String }
impl Draw for Button {
    fn draw(&self) {
        println!("Drawing Button: {}", self.label);
    }
}

struct Checkbox { checked: bool }
impl Draw for Checkbox {
    fn draw(&self) {
        println!("Drawing Checkbox: {}", self.checked);
    }
}

// Trait objects allow different types in same collection
fn main() {
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: "Submit".to_string() }),
        Box::new(Checkbox { checked: true }),
    ];

    for c in components {
        c.draw();
    }
}
