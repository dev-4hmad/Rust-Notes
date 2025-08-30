// Sum of odd numbers
fn row_sum_odd_numbers(n: i64) -> i64 {
    n * n * n
}

fn main() {
    println!("{}", row_sum_odd_numbers(1)); // 1
    println!("{}", row_sum_odd_numbers(2)); // 8
    println!("{}", row_sum_odd_numbers(3)); // 27
    println!("{}", row_sum_odd_numbers(4)); // 64
}
