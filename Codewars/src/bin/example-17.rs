fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    match operator {
        '+' => value1 + value2,
        '-' => value1 - value2,
        '*' => value1 * value2,
        '/' => value1 / value2,
        _ => panic!("Unknown operator"),
    }
}

fn main() {
    println!("{}", basic_op('+', 4, 7));  // 11
    println!("{}", basic_op('-', 15, 18)); // -3
    println!("{}", basic_op('*', 5, 5));   // 25
    println!("{}", basic_op('/', 49, 7));  // 7
}
