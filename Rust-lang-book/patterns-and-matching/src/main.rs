fn main() {
    // 19.1 Pattern usage: let, match, if let, while let
    let (x, y) = (1, 2);          // tuple pattern
    println!("x={}, y={}", x, y);

    // 19.2 Refutability: patterns that might fail
    let some_option = Some(42);
    if let Some(val) = some_option { // refutable pattern
        println!("Got {}", val);
    }

    // 19.3 Pattern syntax: match arms
    let number = 3;
    match number {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"), // OR pattern
        4..=6 => println!("Four to Six"),  // Range pattern
        _ => println!("Other"),
    }

    // Destructuring structs
    struct Point { x: i32, y: i32 }
    let p = Point { x: 10, y: 20 };
    let Point { x, y } = p; // struct pattern
    println!("Point: x={}, y={}", x, y);
}
