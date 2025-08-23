fn main() {
    println!("--- Closures ---");

    // Basic closure
    let add_one = |x: i32| x + 1;
    println!("5 + 1 = {}", add_one(5));

    // Closure capturing environment
    let multiplier = 3;
    let multiply = |x: i32| x * multiplier;
    println!("10 * {} = {}", multiplier, multiply(10));

    // Closure with explicit type annotation
    let explicit: fn(i32) -> i32 = |x| x + 2;
    println!("3 + 2 = {}", explicit(3));

    // Closure capturing mutable environment
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count is now {}", count);
    };
    increment();
    increment();

    // Closure that takes ownership (move)
    let names = vec!["Ahmad", "Rust", "Book"];
    let print_names = move || {
        for name in names {
            println!("Hello {}", name);
        }
    };
    print_names();
}
