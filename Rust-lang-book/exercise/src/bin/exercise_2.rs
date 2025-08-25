// Generate the nth Fibonacci number.
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0u64;
    let mut curr = 1u64;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

fn main() {
    let n = 10; // Change this to any number
    println!("The {}th Fibonacci number is {}", n, fibonacci(n));
}
