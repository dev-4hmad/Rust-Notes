fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let total = on + wait;
    if total <= cap { 0 } else { total - cap }
}
fn main() {
    println!("{}", enough(10, 5, 5)); // 0
    println!("{}", enough(100, 60, 50)); // 10
}
